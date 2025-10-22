use proc_macro2::TokenStream;
use quote::quote;

use crate::parsing::StructInfo;

pub fn generate(info: &StructInfo) -> TokenStream {
    let struct_name = &info.name;
    let field_names = info.fields.iter().map(|f| &f.name);

    quote! {
        impl #struct_name {
            pub fn empty() -> Self {
                Self {
                    #(#field_names: ::std::marker::PhantomData),*
                }
            }
        }
    }
}
