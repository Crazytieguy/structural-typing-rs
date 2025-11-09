use proc_macro2::TokenStream;
use quote::quote;

use crate::parsing::StructInfo;

pub fn generate(info: &StructInfo) -> TokenStream {
    let struct_name = &info.name;
    let struct_vis = &info.vis;
    let module_name = &info.module_name;
    let other_attrs = &info.other_attrs;

    let has_serialize = info.derives.iter().any(|d| d == "Serialize");
    let has_deserialize = info.derives.iter().any(|d| d == "Deserialize");
    let has_serde = has_serialize || has_deserialize;

    let field_defs: Vec<_> = info.fields.iter().map(|field| {
        let field_name = &field.name;
        let field_ty = &field.ty;
        let field_vis = &field.vis;

        let serde_attr = if has_serde {
            quote! { #[serde(skip_serializing_if = "::structural_typing::access::is_absent")] }
        } else {
            quote! {}
        };

        quote! {
            #serde_attr
            #field_vis #field_name: <F::#field_name as ::structural_typing::presence::Presence>::Output<#field_ty>
        }
    }).collect();

    let derive_bounds: Vec<_> = info
        .fields
        .iter()
        .map(|field| {
            let field_name = &field.name;
            let field_ty = &field.ty;
            quote! {
                <F::#field_name as ::structural_typing::presence::Presence>::Output<#field_ty>
            }
        })
        .collect();

    let derives = &info.derives;
    let derive_clause = if derives.is_empty() {
        quote! {}
    } else {
        quote! {
            #[::derive_where::derive_where(#(#derives),*; #(#derive_bounds),*)]
        }
    };

    quote! {
        #derive_clause
        #(#other_attrs)*
        #struct_vis struct #struct_name<F: #module_name::Fields = #module_name::with::all::Present> {
            #(#field_defs),*
        }
    }
}
