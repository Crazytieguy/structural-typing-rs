use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, GenericArgument, Ident, PathArguments, Type};

use crate::codegen::generics_utils;
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

fn generate_remainder_fields(field_names: &[&Ident], field_types: &[&Type]) -> Vec<TokenStream> {
    field_names
        .iter()
        .zip(field_types.iter())
        .map(|(name, ty)| {
            quote! {
                <<<<F2 as Fields>::#name as ::structural_typing::presence::Presence>::Output<#ty> as ::structural_typing::access::Access<#ty>>::RemainderFrom<<<F1 as Fields>::#name as ::structural_typing::presence::Presence>::Output<#ty>> as ::structural_typing::presence::InferPresence<#ty>>::Presence
            }
        })
        .collect()
}

fn generate_empty_constructor(info: &StructInfo) -> TokenStream {
    let struct_name = &info.name;
    let field_names: Vec<_> = info.fields.iter().map(|f| &f.name).collect();

    let non_defaulted_params = generics_utils::non_defaulted_params(&info.generics);
    let non_defaulted_params_with_bounds: Vec<_> = non_defaulted_params.iter().collect();

    // Build a generics object with only non-defaulted params for type_args_with_f
    let mut non_defaulted_generics = info.generics.clone();
    non_defaulted_generics.params = non_defaulted_params.iter().cloned().collect();

    let user_type_args_for_return: Vec<_> =
        generics_utils::extract_type_param_idents(&non_defaulted_generics.params);

    let f_value = quote! { with::all<::structural_typing::presence::Absent> };
    let type_args = generics_utils::type_args_with_f(
        &non_defaulted_generics,
        &user_type_args_for_return,
        f_value,
    );

    let where_clause = &info.generics.where_clause;

    let generic_params = if non_defaulted_params.is_empty() {
        quote! {}
    } else {
        quote! { <#(#non_defaulted_params_with_bounds),*> }
    };

    quote! {
        /// Creates an empty instance with all fields absent.
        pub fn empty #generic_params () -> #struct_name #type_args #where_clause {
            #struct_name {
                #(#field_names: ::core::marker::PhantomData),*
            }
        }
    }
}

fn extract_generic_idents_from_type(
    ty: &Type,
    user_generics: &[String],
    generic_params: &syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
) -> Vec<TokenStream> {
    let mut found_idents = Vec::new();
    visit_type_for_generics(ty, user_generics, &mut found_idents);

    // Map found identifiers back to their full GenericParam declarations
    // Preserve declaration order by iterating through generic_params in order
    let mut result = Vec::new();
    let found_strs: std::collections::HashSet<String> =
        found_idents.iter().map(|t| t.to_string()).collect();

    for param in generic_params {
        let (param_str, token) = match param {
            syn::GenericParam::Type(tp) => {
                let ident = &tp.ident;
                (ident.to_string(), quote! { #ident })
            }
            syn::GenericParam::Lifetime(lp) => {
                let lifetime = &lp.lifetime;
                (lifetime.to_string(), quote! { #lifetime })
            }
            syn::GenericParam::Const(cp) => {
                let ident = &cp.ident;
                let ty = &cp.ty;
                (ident.to_string(), quote! { const #ident: #ty })
            }
        };

        // Use exact match instead of substring match to avoid false positives
        if found_strs.contains(&param_str) {
            result.push(token);
        }
    }
    result
}

fn visit_type_for_generics(
    ty: &Type,
    user_generics: &[String],
    generics: &mut Vec<TokenStream>,
) {
    match ty {
        Type::Path(type_path) => {
            if let Some(ident) = type_path.path.get_ident() {
                let ident_str = ident.to_string();
                if user_generics.contains(&ident_str) {
                    let token = quote! { #ident };
                    if !generics.iter().any(|g| g.to_string() == token.to_string()) {
                        generics.push(token);
                    }
                }
            }

            for segment in &type_path.path.segments {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    for arg in &args.args {
                        match arg {
                            GenericArgument::Type(ty) => {
                                visit_type_for_generics(ty, user_generics, generics);
                            }
                            GenericArgument::Lifetime(lifetime) => {
                                let lifetime_str = lifetime.ident.to_string();
                                if user_generics.contains(&lifetime_str) {
                                    let token = quote! { #lifetime };
                                    if !generics
                                        .iter()
                                        .any(|g| g.to_string() == token.to_string())
                                    {
                                        generics.push(token);
                                    }
                                }
                            }
                            GenericArgument::Const(expr) => {
                                visit_expr_for_generics(expr, user_generics, generics);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        Type::Reference(type_ref) => {
            // Check for explicit lifetime in reference
            if let Some(lifetime) = &type_ref.lifetime {
                let lifetime_str = lifetime.ident.to_string();
                if user_generics.contains(&lifetime_str) {
                    let token = quote! { #lifetime };
                    if !generics.iter().any(|g| g.to_string() == token.to_string()) {
                        generics.push(token);
                    }
                }
            }
            visit_type_for_generics(&type_ref.elem, user_generics, generics);
        }
        Type::Ptr(type_ptr) => {
            visit_type_for_generics(&type_ptr.elem, user_generics, generics);
        }
        Type::Array(type_array) => {
            visit_type_for_generics(&type_array.elem, user_generics, generics);
            // Check if the array length uses a const generic
            visit_expr_for_generics(&type_array.len, user_generics, generics);
        }
        Type::Slice(type_slice) => {
            visit_type_for_generics(&type_slice.elem, user_generics, generics);
        }
        Type::Tuple(type_tuple) => {
            for elem in &type_tuple.elems {
                visit_type_for_generics(elem, user_generics, generics);
            }
        }
        Type::Paren(type_paren) => {
            visit_type_for_generics(&type_paren.elem, user_generics, generics);
        }
        _ => {}
    }
}

fn visit_expr_for_generics(
    expr: &Expr,
    user_generics: &[String],
    generics: &mut Vec<TokenStream>,
) {
    match expr {
        Expr::Path(expr_path) => {
            if let Some(ident) = expr_path.path.get_ident() {
                let ident_str = ident.to_string();
                if user_generics.contains(&ident_str) {
                    // Store just the identifier token for const generics
                    let token = quote! { #ident };
                    if !generics.iter().any(|g| g.to_string() == token.to_string()) {
                        generics.push(token);
                    }
                }
            }
        }
        Expr::Binary(binary) => {
            // Handle binary expressions like N + M, N * 2
            visit_expr_for_generics(&binary.left, user_generics, generics);
            visit_expr_for_generics(&binary.right, user_generics, generics);
        }
        Expr::Block(block) => {
            // Handle const blocks like { N * 2 }
            if let Some(syn::Stmt::Expr(expr, _)) = block.block.stmts.last() {
                visit_expr_for_generics(expr, user_generics, generics);
            }
        }
        _ => {
            // For other expression types, we don't extract generics
            // This is conservative but avoids false positives
        }
    }
}

fn generate_type_of_module(info: &StructInfo) -> TokenStream {
    let user_generics: Vec<String> = info
        .generics
        .params
        .iter()
        .map(|param| match param {
            syn::GenericParam::Type(type_param) => type_param.ident.to_string(),
            syn::GenericParam::Lifetime(lifetime_param) => {
                lifetime_param.lifetime.ident.to_string()
            }
            syn::GenericParam::Const(const_param) => const_param.ident.to_string(),
        })
        .collect();

    let field_type_aliases: Vec<_> = info
        .fields
        .iter()
        .map(|field| {
            let field_name = &field.name;
            let field_ty = &field.ty;
            let generics_in_field =
                extract_generic_idents_from_type(field_ty, &user_generics, &info.generics.params);

            if generics_in_field.is_empty() {
                quote! {
                    #[allow(non_camel_case_types)]
                    pub type #field_name = #field_ty;
                }
            } else {
                quote! {
                    #[allow(non_camel_case_types)]
                    pub type #field_name<#(#generics_in_field),*> = #field_ty;
                }
            }
        })
        .collect();

    quote! {
        /// Type aliases for accessing field types independent of presence state.
        ///
        /// Use these when you need to reference the underlying field type in generic code,
        /// e.g., for function parameters or return types that work with field values directly.
        pub mod type_of {
            use super::*;

            #(#field_type_aliases)*
        }
    }
}

fn generate_with_modules(field_names: &[&Ident]) -> TokenStream {
    let has_multiple_fields = field_names.len() > 1;

    let all_absent_default: Vec<_> = std::iter::repeat_n(
        quote! { ::structural_typing::presence::Absent },
        field_names.len(),
    )
    .collect();

    let field_type_aliases: Vec<_> = field_names
        .iter()
        .enumerate()
        .map(|(current_idx, field_name)| {
            let field_types: Vec<_> = field_names
                .iter()
                .enumerate()
                .map(|(idx, name)| {
                    if idx == current_idx {
                        quote! { P }
                    } else {
                        quote! { F::#name }
                    }
                })
                .collect();

            if has_multiple_fields {
                quote! {
                    /// Parameterized field presence type alias.
                    #[allow(non_camel_case_types)]
                    pub type #field_name<
                        P: Presence = ::structural_typing::presence::Present,
                        F: Fields = FieldSet<#(#all_absent_default),*>
                    > = FieldSet<#(#field_types),*>;
                }
            } else {
                // Single-field structs don't get F parameter since there are no other fields to spread.
                // Attempting to use nested setters with single-field structs will fail at compile time
                // with "type parameter F is never used" error when the select! macro tries to use
                // with::field<P, F> - this is intentional and documents the limitation clearly.
                quote! {
                    /// Parameterized field presence type alias.
                    #[allow(non_camel_case_types)]
                    pub type #field_name<P: Presence = ::structural_typing::presence::Present> = FieldSet<P>;
                }
            }
        })
        .collect();

    let all_fields: Vec<_> = std::iter::repeat_n(quote! { P }, field_names.len()).collect();

    quote! {
        /// Type constructors for building field presence combinations.
        ///
        /// Each field has a type alias that sets its presence while inheriting others from a base.
        /// Use these with the `select!` macro or directly for type annotations.
        pub mod with {
            use super::*;

            #(#field_type_aliases)*

            /// Sets all fields to the same presence state.
            #[allow(non_camel_case_types)]
            pub type all<P: Presence = ::structural_typing::presence::Present> = FieldSet<#(#all_fields),*>;
        }
    }
}

pub fn generate(info: &StructInfo, serde_helper: Option<TokenStream>) -> TokenStream {
    let module_name = &info.module_name;
    let vis = &info.vis;
    let field_names: Vec<_> = info.fields.iter().map(|f| &f.name).collect();
    let field_types: Vec<_> = info.fields.iter().map(|f| &f.ty).collect();

    let field_type_assocs = generate_fields_trait_parts(&field_names);
    let (fieldset_phantom_types, fieldset_params, fieldset_assocs) =
        generate_fieldset_parts(&field_names);
    let merge_fields = generate_merge_fields(&field_names);
    let remainder_fields = generate_remainder_fields(&field_names, &field_types);
    let type_of_module = generate_type_of_module(info);
    let with_modules = generate_with_modules(&field_names);
    let empty_constructor = generate_empty_constructor(info);

    let remainder_params = if info.generics.params.is_empty() {
        quote! { F1, F2 }
    } else {
        let (lifetimes, others): (Vec<_>, Vec<_>) = info
            .generics
            .params
            .iter()
            .partition(|p| matches!(p, syn::GenericParam::Lifetime(_)));
        quote! { #(#lifetimes,)* F1, F2, #(#others),* }
    };

    let remainder_type = quote! {
        /// Computes the field presence state after extracting `F2` from `F1`.
        ///
        /// When a field is extracted from Present → Present, the remainder is Absent.
        /// When extracted from Present → Optional, the remainder is also Optional.
        #[allow(type_alias_bounds)]
        pub type Remainder<#remainder_params> = FieldSet<
            #(#remainder_fields),*
        >;
    };

    let canonical_fields: Vec<_> = field_names.iter().map(|name| quote! { F::#name }).collect();

    let canonical_type = quote! {
        /// Converts a `Fields` trait bound to its concrete `FieldSet` representation.
        ///
        /// Use this when you need a concrete type alias from a generic `F: Fields` bound,
        /// e.g., to store in a struct field or name the type explicitly.
        pub type Canonical<F: Fields> = FieldSet<
            #(#canonical_fields),*
        >;
    };

    quote! {
        #vis mod #module_name {
            use super::*;
            use ::core::marker::PhantomData;
            use ::structural_typing::presence::{Presence, Present, Optional, Absent};

            #serde_helper

            mod sealed {
                pub trait Sealed {}
            }

            /// Trait for constraining field presence states in generic code.
            ///
            /// Use this in trait bounds to require specific fields:
            /// `fn foo<F: Fields<name = Present, id = Present>>(...)`.
            #[allow(non_camel_case_types)]
            pub trait Fields: sealed::Sealed {
                #(#field_type_assocs)*
            }

            /// Concrete type representing a specific combination of field presence states.
            ///
            /// Each type parameter corresponds to a field's presence (Present, Optional, or Absent).
            /// Use `with::` aliases or `select!` macro instead of constructing this directly.
            #[allow(non_camel_case_types)]
            pub struct FieldSet<#(#fieldset_params),*>(
                PhantomData<(#(#fieldset_phantom_types),*)>,
            );

            #[allow(non_camel_case_types)]
            impl<#(#fieldset_params),*> sealed::Sealed for FieldSet<#(#field_names),*> {}

            #[allow(non_camel_case_types)]
            impl<#(#fieldset_params),*> Fields for FieldSet<#(#field_names),*> {
                #(#fieldset_assocs)*
            }

            /// Computes the result of merging two field sets (F2 takes precedence over F1).
            ///
            /// Present in either → Present. Optional + Absent → Optional.
            pub type Merge<F1, F2> = FieldSet<
                #(#merge_fields),*
            >;

            #remainder_type

            #canonical_type

            #type_of_module

            #with_modules

            #empty_constructor
        }
    }
}
