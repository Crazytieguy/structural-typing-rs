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

pub fn generate(info: &StructInfo) -> TokenStream {
    let module_name = &info.module_name;
    let field_names: Vec<_> = info.fields.iter().map(|f| &f.name).collect();

    let field_type_assocs = generate_fields_trait_parts(&field_names);
    let (fieldset_phantom_types, fieldset_params, fieldset_assocs) =
        generate_fieldset_parts(&field_names);
    let merge_fields = generate_merge_fields(&field_names);
    let (all_present, all_optional, all_absent) = generate_presence_type_aliases(info.fields.len());

    quote! {
        mod #module_name {
            use super::*;
            use ::std::marker::PhantomData;
            use ::structural_typing::presence::{Presence, Present, Optional, Absent};

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
