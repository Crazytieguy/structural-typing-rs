use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type};

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
        /// Type aliases for field presence combinations.
        pub mod with {
            use super::*;

            #(#field_type_aliases)*

            /// All fields with the same presence state.
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
    let with_modules = generate_with_modules(&field_names);

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
        /// Remainder after extracting F2 from F1.
        #[allow(type_alias_bounds)]
        pub type Remainder<#remainder_params> = FieldSet<
            #(#remainder_fields),*
        >;
    };

    let canonical_fields: Vec<_> = field_names.iter().map(|name| quote! { F::#name }).collect();

    let canonical_type = quote! {
        /// Convert a Fields trait bound to its canonical FieldSet representation.
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

            /// Trait representing field presence states.
            #[allow(non_camel_case_types)]
            pub trait Fields: sealed::Sealed {
                #(#field_type_assocs)*
            }

            /// Type-level representation of field presence states.
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

            /// Merge two field sets (F2 takes precedence over F1).
            pub type Merge<F1, F2> = FieldSet<
                #(#merge_fields),*
            >;

            #remainder_type

            #canonical_type

            #with_modules
        }
    }
}
