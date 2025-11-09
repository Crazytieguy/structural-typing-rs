use proc_macro2::TokenStream;
use quote::quote;

use crate::parsing::StructInfo;

pub fn generate(info: &StructInfo) -> TokenStream {
    let struct_name = &info.name;
    let module_name = &info.module_name;
    let field_names = info.fields.iter().map(|f| &f.name);

    quote! {
        impl #struct_name {
            /// Creates an empty instance with all fields absent.
            pub fn empty() -> #struct_name<#module_name::with::all::Absent> {
                #struct_name {
                    #(#field_names: ::core::marker::PhantomData),*
                }
            }
        }
    }
}
