use proc_macro2::TokenStream;
use quote::quote;

use crate::parsing::StructInfo;

pub fn generate(info: &StructInfo) -> TokenStream {
    let struct_name = &info.name;
    let module_name = &info.module_name;

    let field_merges = info.fields.iter().map(|field| {
        let field_name = &field.name;
        quote! {
            #field_name: <F2::#field_name as ::structural_typing::presence::Presence>::or(other.#field_name, self.#field_name)
        }
    });

    quote! {
        impl<F: #module_name::Fields> #struct_name<F> {
            /// Combines two instances, preferring `other`'s fields when both are present.
            pub fn merge<F2: #module_name::Fields>(self, other: #struct_name<F2>) -> #struct_name<#module_name::Merge<F, F2>> {
                #struct_name {
                    #(#field_merges),*
                }
            }
        }
    }
}
