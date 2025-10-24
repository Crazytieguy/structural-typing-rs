use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::parsing::StructInfo;

fn generate_fields_trait_parts(field_names: &[&Ident]) -> Vec<TokenStream> {
    field_names
        .iter()
        .map(|name| {
            quote! { type #name: Presence; }
        })
        .collect()
}

fn generate_fieldset_parts(
    field_names: &[&Ident],
) -> (Vec<TokenStream>, Vec<TokenStream>, Vec<TokenStream>) {
    let phantom_types = field_names
        .iter()
        .map(|name| {
            quote! { #name }
        })
        .collect();

    let params = field_names
        .iter()
        .map(|name| {
            quote! { #name: Presence }
        })
        .collect();

    let assocs = field_names
        .iter()
        .map(|name| {
            quote! { type #name = #name; }
        })
        .collect();

    (phantom_types, params, assocs)
}

fn generate_merge_fields(field_names: &[&Ident]) -> Vec<TokenStream> {
    field_names
        .iter()
        .map(|name| {
            quote! {
                <<F2 as Fields>::#name as Presence>::Or<<F1 as Fields>::#name>
            }
        })
        .collect()
}

fn generate_presence_type_aliases(
    field_count: usize,
) -> (Vec<TokenStream>, Vec<TokenStream>, Vec<TokenStream>) {
    let all_present = std::iter::repeat_n(quote! { Present }, field_count).collect();
    let all_optional = std::iter::repeat_n(quote! { Optional }, field_count).collect();
    let all_absent = std::iter::repeat_n(quote! { Absent }, field_count).collect();
    (all_present, all_optional, all_absent)
}

fn sanitize_ident_for_macro_name(ident: &Ident) -> String {
    let s = ident.to_string();
    // Strip r# prefix from raw identifiers
    s.strip_prefix("r#").unwrap_or(&s).to_string()
}

fn generate_select_validator(module_name: &Ident, field_names: &[&Ident]) -> TokenStream {
    let field_names_str = field_names
        .iter()
        .map(|f| f.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    let optional_patterns: Vec<_> = field_names
        .iter()
        .map(|name| {
            quote! { (? #name $($rest:tt)*) => { #module_name::select_validator!($($rest)*) }; }
        })
        .collect();

    let present_patterns: Vec<_> = field_names
        .iter()
        .map(|name| {
            quote! { (#name $($rest:tt)*) => { #module_name::select_validator!($($rest)*) }; }
        })
        .collect();

    let validator_name = syn::Ident::new(&format!("{}_select_validator", module_name), module_name.span());

    quote! {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! #validator_name {
            // Valid: ?field for each known field
            #(#optional_patterns)*
            // Valid: field for each known field
            #(#present_patterns)*
            // Valid: commas
            (, $($rest:tt)*) => { #validator_name!($($rest)*) };
            // Base case - validation passed
            () => {};
            // CATCH-ALL: anything else is invalid
            ($invalid:tt $($rest:tt)*) => {
                compile_error!(concat!(
                    "Invalid syntax in select!: '",
                    stringify!($invalid),
                    "'. Expected: field or ?field. Valid fields: ",
                    #field_names_str
                ))
            };
        }
    }
}

fn generate_select_helpers(module_name: &Ident, field_names: &[&Ident]) -> Vec<TokenStream> {
    field_names
        .iter()
        .map(|field_name| {
            let field_name_str = sanitize_ident_for_macro_name(field_name);
            let helper_name = syn::Ident::new(
                &format!("__{}__select_{}", module_name, field_name_str),
                field_name.span(),
            );
            quote! {
                #[doc(hidden)]
                #[macro_export]
                macro_rules! #helper_name {
                    // Match with ? prefix for Optional
                    (? #field_name $($rest:tt)*) => { ::structural_typing::presence::Optional };
                    (? #field_name, $($rest:tt)*) => { ::structural_typing::presence::Optional };
                    (? #field_name) => { ::structural_typing::presence::Optional };
                    // Match without prefix for Present
                    (#field_name $($rest:tt)*) => { ::structural_typing::presence::Present };
                    (#field_name, $($rest:tt)*) => { ::structural_typing::presence::Present };
                    (#field_name) => { ::structural_typing::presence::Present };
                    // Skip other tokens (validator catches invalid syntax)
                    (? $skip:tt, $($rest:tt)*) => { #helper_name!($($rest)*) };
                    (? $skip:tt $($rest:tt)*) => { #helper_name!($($rest)*) };
                    ($skip:tt, $($rest:tt)*) => { #helper_name!($($rest)*) };
                    ($skip:tt $($rest:tt)*) => { #helper_name!($($rest)*) };
                    // Not found - Absent
                    () => { ::structural_typing::presence::Absent };
                }
            }
        })
        .collect()
}

fn generate_select_macro(module_name: &Ident, field_names: &[&Ident]) -> TokenStream {
    let field_names_str = field_names
        .iter()
        .map(|f| f.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    // Generate valid field patterns for validation (save original input in brackets)
    let valid_optional: Vec<_> = field_names
        .iter()
        .map(|name| {
            quote! { (@validate [$($all_saved:tt)*] ? #name $($rest:tt)*) => { #module_name::select!(@ok [$($all_saved)*] $($rest)*) }; }
        })
        .collect();

    let valid_present: Vec<_> = field_names
        .iter()
        .map(|name| {
            quote! { (@validate [$($all_saved:tt)*] #name $($rest:tt)*) => { #module_name::select!(@ok [$($all_saved)*] $($rest)*) }; }
        })
        .collect();

    let helper_calls: Vec<_> = field_names
        .iter()
        .map(|field_name| {
            let field_name_str = sanitize_ident_for_macro_name(field_name);
            let helper_name = syn::Ident::new(
                &format!("__{}__select_{}", module_name, field_name_str),
                field_name.span(),
            );
            quote! { #helper_name!($($all)*) }
        })
        .collect();

    let macro_name = syn::Ident::new(&format!("{}_select", module_name), module_name.span());

    quote! {
        /// Construct a FieldSet by selecting fields.
        ///
        /// # Syntax
        /// - `select!(field)` - field is Present
        /// - `select!(?field)` - field is Optional
        /// - Unlisted fields are Absent
        /// - `select!()` - all fields Absent (equivalent to AllAbsent)
        ///
        /// # First-Match-Wins
        /// If a field appears multiple times, the first specification is used:
        /// - `select!(name, name)` → name is Present (first match)
        /// - `select!(name, ?name)` → name is Present (first match)
        /// - `select!(?name, name)` → name is Optional (first match)
        ///
        /// # Example
        /// ```ignore
        /// // For User { name, email, id }
        /// select!(name, id)       // FieldSet<Present, Absent, Present>
        /// select!(?email, name)   // FieldSet<Present, Optional, Absent>
        /// select!()               // FieldSet<Absent, Absent, Absent>
        /// ```
        #[macro_export]
        macro_rules! #macro_name {
            // Validation: skip commas (keep saved input in brackets)
            (@validate [$($all_saved:tt)*] , $($rest:tt)*) => {
                #module_name::select!(@validate [$($all_saved)*] $($rest)*)
            };

            // Validation: valid optional fields (keep saved input in brackets)
            #(#valid_optional)*

            // Validation: valid present fields (keep saved input in brackets)
            #(#valid_present)*

            // After validating one token, continue with rest (keep saved input in brackets)
            (@ok [$($all_saved:tt)*] $($rest:tt)*) => {
                #module_name::select!(@validate [$($all_saved)*] $($rest)*)
            };

            // All validation passed - construct type (MUST come before catch-all patterns)
            (@validate [$($all:tt)*] @ DONE) => {
                #module_name::FieldSet<
                    #(#helper_calls),*
                >
            };

            // Validation: invalid prefix +
            (@validate [$($saved:tt)*] + $tok:tt $($rest:tt)*) => {
                compile_error!(concat!(
                    "Invalid prefix '+' in select!. Did you mean modify!? ",
                    "select! only supports: field or ?field"
                ))
            };

            // Validation: invalid prefix -
            (@validate [$($saved:tt)*] - $tok:tt $($rest:tt)*) => {
                compile_error!(concat!(
                    "Invalid prefix '-' in select!. Did you mean modify!? ",
                    "select! only supports: field or ?field"
                ))
            };

            // Validation: unknown identifier
            (@validate [$($saved:tt)*] $unknown:ident $($rest:tt)*) => {
                compile_error!(concat!(
                    "Unknown field '", stringify!($unknown),
                    "' in select!. Valid fields: ", #field_names_str
                ))
            };

            // Validation: other invalid syntax
            (@validate [$($saved:tt)*] $invalid:tt $($rest:tt)*) => {
                compile_error!(concat!(
                    "Invalid syntax in select!: '", stringify!($invalid),
                    "'. Expected: field or ?field"
                ))
            };

            // Entry point - save input in brackets and start validation
            // MUST come last so it doesn't match internal @validate/@ok calls
            ($($all:tt)*) => {
                #module_name::select!(@validate [$($all)*] $($all)* @ DONE)
            };
        }
    }
}

fn generate_modify_validator(module_name: &Ident, field_names: &[&Ident]) -> TokenStream {
    let field_names_str = field_names
        .iter()
        .map(|f| f.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    let add_patterns: Vec<_> = field_names
        .iter()
        .map(|name| {
            quote! { (@validate + #name $($rest:tt)*) => { #module_name::modify_validator!(@validate $($rest)*) }; }
        })
        .collect();

    let remove_patterns: Vec<_> = field_names
        .iter()
        .map(|name| {
            quote! { (@validate - #name $($rest:tt)*) => { #module_name::modify_validator!(@validate $($rest)*) }; }
        })
        .collect();

    let optional_patterns: Vec<_> = field_names
        .iter()
        .map(|name| {
            quote! { (@validate ? #name $($rest:tt)*) => { #module_name::modify_validator!(@validate $($rest)*) }; }
        })
        .collect();

    let validator_name = syn::Ident::new(&format!("{}_modify_validator", module_name), module_name.span());

    quote! {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! #validator_name {
            // Valid: +field for each known field
            #(#add_patterns)*
            // Valid: -field for each known field
            #(#remove_patterns)*
            // Valid: ?field for each known field
            #(#optional_patterns)*
            // Valid: commas
            (@validate , $($rest:tt)*) => { #validator_name!(@validate $($rest)*) };
            // Base case
            (@validate) => {};
            // CATCH-ALL: anything else is invalid
            (@validate $invalid:tt $($rest:tt)*) => {
                compile_error!(concat!(
                    "Invalid syntax in modify!: '",
                    stringify!($invalid),
                    "'. Expected: +field, -field, or ?field. Valid fields: ",
                    #field_names_str
                ))
            };
        }
    }
}

fn generate_modify_helpers(module_name: &Ident, field_names: &[&Ident]) -> Vec<TokenStream> {
    field_names
        .iter()
        .map(|field_name| {
            let field_name_str = sanitize_ident_for_macro_name(field_name);
            let helper_name = syn::Ident::new(
                &format!("__{}__modify_{}", module_name, field_name_str),
                field_name.span(),
            );
            quote! {
                #[doc(hidden)]
                #[macro_export]
                macro_rules! #helper_name {
                    // Match modifications for this field
                    ($base:ty, + #field_name, $($rest:tt)*) => { ::structural_typing::presence::Present };
                    ($base:ty, + #field_name) => { ::structural_typing::presence::Present };
                    ($base:ty, - #field_name, $($rest:tt)*) => { ::structural_typing::presence::Absent };
                    ($base:ty, - #field_name) => { ::structural_typing::presence::Absent };
                    ($base:ty, ? #field_name, $($rest:tt)*) => { ::structural_typing::presence::Optional };
                    ($base:ty, ? #field_name) => { ::structural_typing::presence::Optional };
                    // Skip other tokens (validator catches invalid syntax)
                    ($base:ty, $prefix:tt $skip:tt, $($rest:tt)*) => {
                        #helper_name!($base, $($rest)*)
                    };
                    ($base:ty, $prefix:tt $skip:tt) => { <$base as #module_name::Fields>::#field_name };
                    // Not found in modifications, use base type's field
                    ($base:ty,) => { <$base as #module_name::Fields>::#field_name };
                    ($base:ty) => { <$base as #module_name::Fields>::#field_name };
                }
            }
        })
        .collect()
}

fn generate_modify_macro(module_name: &Ident, field_names: &[&Ident]) -> TokenStream {
    let field_names_str = field_names
        .iter()
        .map(|f| f.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    // Generate valid field patterns for validation (save original inputs in brackets)
    let valid_add: Vec<_> = field_names
        .iter()
        .map(|name| {
            quote! { (@validate [$($all_saved:tt)*] + #name $($rest:tt)*) => { #module_name::modify!(@ok [$($all_saved)*] $($rest)*) }; }
        })
        .collect();

    let valid_remove: Vec<_> = field_names
        .iter()
        .map(|name| {
            quote! { (@validate [$($all_saved:tt)*] - #name $($rest:tt)*) => { #module_name::modify!(@ok [$($all_saved)*] $($rest)*) }; }
        })
        .collect();

    let valid_optional: Vec<_> = field_names
        .iter()
        .map(|name| {
            quote! { (@validate [$($all_saved:tt)*] ? #name $($rest:tt)*) => { #module_name::modify!(@ok [$($all_saved)*] $($rest)*) }; }
        })
        .collect();

    let helper_calls: Vec<_> = field_names
        .iter()
        .map(|field_name| {
            let field_name_str = sanitize_ident_for_macro_name(field_name);
            let helper_name = syn::Ident::new(
                &format!("__{}__modify_{}", module_name, field_name_str),
                field_name.span(),
            );
            quote! { #helper_name!($base, $($mods)*) }
        })
        .collect();

    let macro_name = syn::Ident::new(&format!("{}_modify", module_name), module_name.span());

    quote! {
        /// Modify an existing FieldSet by changing specific fields.
        ///
        /// # Syntax
        /// - `modify!(BaseType, +field)` - make field Present
        /// - `modify!(BaseType, -field)` - make field Absent
        /// - `modify!(BaseType, ?field)` - make field Optional
        /// - Unmodified fields keep their presence from BaseType
        ///
        /// # First-Match-Wins
        /// If a field is modified multiple times, the first modification is used:
        /// - `modify!(Base, +name, -name)` → name is Present (first match)
        /// - `modify!(Base, ?name, +name)` → name is Optional (first match)
        ///
        /// # Example
        /// ```ignore
        /// // For User { name, email, id }
        /// modify!(AllAbsent, +name, +email)    // FieldSet<Present, Present, Absent>
        /// modify!(AllPresent, -email, ?id)     // FieldSet<Present, Absent, Optional>
        /// ```
        #[macro_export]
        macro_rules! #macro_name {
            // Validation: skip commas (keep saved inputs in brackets)
            (@validate [$($all_saved:tt)*] , $($rest:tt)*) => {
                #module_name::modify!(@validate [$($all_saved)*] $($rest)*)
            };

            // Validation: valid +field (keep saved inputs in brackets)
            #(#valid_add)*

            // Validation: valid -field (keep saved inputs in brackets)
            #(#valid_remove)*

            // Validation: valid ?field (keep saved inputs in brackets)
            #(#valid_optional)*

            // After validating one token, continue with rest (keep saved inputs in brackets)
            (@ok [$($all_saved:tt)*] $($rest:tt)*) => {
                #module_name::modify!(@validate [$($all_saved)*] $($rest)*)
            };

            // All validation passed - construct type (MUST come before catch-all patterns)
            (@validate [$base:ty, $($mods:tt)*] @ DONE) => {
                #module_name::FieldSet<
                    #(#helper_calls),*
                >
            };

            // Validation: invalid operator
            (@validate [$($saved:tt)*] * $tok:tt $($rest:tt)*) => {
                compile_error!(concat!(
                    "Invalid operator '*' in modify!. ",
                    "Valid operators: + (Present), - (Absent), ? (Optional)"
                ))
            };
            (@validate [$($saved:tt)*] ! $tok:tt $($rest:tt)*) => {
                compile_error!(concat!(
                    "Invalid operator '!' in modify!. ",
                    "Valid operators: + (Present), - (Absent), ? (Optional)"
                ))
            };

            // Validation: missing operator (bare identifier, not the @DONE marker)
            (@validate [$($saved:tt)*] $field:ident $($rest:tt)*) => {
                compile_error!(concat!(
                    "Missing operator before field '", stringify!($field),
                    "' in modify!. Use: +field, -field, or ?field"
                ))
            };

            // Validation: unknown field with operator
            (@validate [$($saved:tt)*] $op:tt $unknown:ident $($rest:tt)*) => {
                compile_error!(concat!(
                    "Unknown field '", stringify!($unknown),
                    "' in modify!. Valid fields: ", #field_names_str
                ))
            };

            // Validation: other invalid syntax
            (@validate [$($saved:tt)*] $invalid:tt $($rest:tt)*) => {
                compile_error!(concat!(
                    "Invalid syntax in modify!: '", stringify!($invalid),
                    "'. Expected: +field, -field, or ?field"
                ))
            };

            // Entry point with modifications - save original inputs in brackets, then validate
            // MUST come last so it doesn't match internal @validate/@ok calls
            ($base:ty, $($mods:tt)*) => {
                #module_name::modify!(@validate [$base, $($mods)*] $($mods)* @ DONE)
            };

            // No-op: no modifications, just return the base type
            ($base:ty) => { $base };
        }
    }
}

pub fn generate(info: &StructInfo) -> TokenStream {
    let module_name = &info.module_name;
    let field_names: Vec<_> = info.fields.iter().map(|f| &f.name).collect();

    let field_type_assocs = generate_fields_trait_parts(&field_names);
    let (fieldset_phantom_types, fieldset_params, fieldset_assocs) =
        generate_fieldset_parts(&field_names);
    let merge_fields = generate_merge_fields(&field_names);
    let (all_present, all_optional, all_absent) = generate_presence_type_aliases(info.fields.len());

    let select_validator = generate_select_validator(module_name, &field_names);
    let select_helpers = generate_select_helpers(module_name, &field_names);
    let select_macro = generate_select_macro(module_name, &field_names);
    let modify_validator = generate_modify_validator(module_name, &field_names);
    let modify_helpers = generate_modify_helpers(module_name, &field_names);
    let modify_macro = generate_modify_macro(module_name, &field_names);

    let select_macro_name = syn::Ident::new(&format!("{}_select", module_name), module_name.span());
    let modify_macro_name = syn::Ident::new(&format!("{}_modify", module_name), module_name.span());
    let select_validator_name = syn::Ident::new(&format!("{}_select_validator", module_name), module_name.span());
    let modify_validator_name = syn::Ident::new(&format!("{}_modify_validator", module_name), module_name.span());

    quote! {
        // Export validators, helper macros and main macros to crate root
        #select_validator
        #(#select_helpers)*
        #select_macro
        #modify_validator
        #(#modify_helpers)*
        #modify_macro

        mod #module_name {
            use super::*;
            use ::std::marker::PhantomData;
            use ::structural_typing::presence::{Presence, Present, Optional, Absent};

            // Re-export macros with simple names
            pub use #select_validator_name as select_validator;
            pub use #select_macro_name as select;
            pub use #modify_validator_name as modify_validator;
            pub use #modify_macro_name as modify;

            mod sealed {
                pub trait Sealed {}
            }

            #[allow(non_camel_case_types)]
            pub trait Fields: sealed::Sealed {
                #(#field_type_assocs)*
            }

            #[allow(non_camel_case_types)]
            #[derive(Clone, Copy, Debug)]
            pub struct FieldSet<#(#fieldset_params),*>(
                PhantomData<(#(#fieldset_phantom_types),*)>,
            );

            #[allow(non_camel_case_types)]
            impl<#(#fieldset_params),*> sealed::Sealed for FieldSet<#(#field_names),*> {}

            #[allow(non_camel_case_types)]
            impl<#(#fieldset_params),*> Fields for FieldSet<#(#field_names),*> {
                #(#fieldset_assocs)*
            }

            pub type Merge<F1, F2> = FieldSet<
                #(#merge_fields),*
            >;

            pub type AllPresent = FieldSet<#(#all_present),*>;
            pub type AllOptional = FieldSet<#(#all_optional),*>;
            pub type AllAbsent = FieldSet<#(#all_absent),*>;
        }
    }
}
