use proc_macro2::TokenStream;
use quote::quote;

use crate::codegen::serde_deserialize::{filter_serialize_attrs, helper_path};
use crate::parsing::StructInfo;

pub fn generate(info: &StructInfo) -> TokenStream {
    let struct_name = &info.name;
    let struct_vis = &info.vis;
    let module_name = &info.module_name;
    let other_attrs = &info.other_attrs;

    let has_serialize = info.derives.iter().any(|d| d == "Serialize");
    let has_deserialize = info.derives.iter().any(|d| d == "Deserialize");

    let helper_path_str = helper_path(module_name, struct_name);
    let try_from_attr = if has_deserialize {
        quote! { #[serde(try_from = #helper_path_str)] }
    } else {
        quote! {}
    };

    let field_defs: Vec<_> = info.fields.iter().map(|field| {
        let field_name = &field.name;
        let field_ty = &field.ty;
        let field_vis = &field.vis;

        let mut serde_attrs = Vec::new();
        if has_serialize {
            serde_attrs.push(quote! { #[serde(skip_serializing_if = "::structural_typing::access::is_absent")] });
        }

        let preserved_attrs = filter_serialize_attrs(&field.attrs);

        quote! {
            #(#serde_attrs)*
            #(#preserved_attrs)*
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

    let derives_to_use: Vec<_> = if has_deserialize {
        info.derives
            .iter()
            .filter(|d| *d != "Deserialize")
            .collect()
    } else {
        info.derives.iter().collect()
    };

    let derive_clause = if derives_to_use.is_empty() {
        quote! {}
    } else {
        quote! {
            #[::derive_where::derive_where(#(#derives_to_use),*; #(#derive_bounds),*)]
        }
    };

    let deserialize_derive = if has_deserialize {
        quote! {
            #[derive(::serde::Deserialize)]
        }
    } else {
        quote! {}
    };

    quote! {
        #derive_clause
        #deserialize_derive
        #try_from_attr
        #(#other_attrs)*
        #struct_vis struct #struct_name<F: #module_name::Fields = #module_name::with::all::Present> {
            #(#field_defs),*
        }
    }
}
