use proc_macro2::TokenStream;
use quote::quote;

use crate::codegen::generics_utils::impl_generics_with_f;
use crate::parsing::StructInfo;

pub fn generate(info: &StructInfo) -> TokenStream {
    let struct_name = &info.name;
    let module_name = &info.module_name;

    let (impl_generics, user_type_args) = impl_generics_with_f(&info.generics, module_name);
    let (impl_generics, _, where_clause) = impl_generics.split_for_impl();

    let field_merges = info.fields.iter().map(|field| {
        let field_name = &field.name;
        quote! {
            #field_name: <F2::#field_name as ::structural_typing::presence::Presence>::or(other.#field_name, self.#field_name)
        }
    });

    quote! {
        impl #impl_generics #struct_name<#(#user_type_args,)* F> #where_clause {
            /// Combines two instances, preferring `other`'s fields when both are present.
            pub fn merge<F2: #module_name::Fields>(self, other: #struct_name<#(#user_type_args,)* F2>) -> #struct_name<#(#user_type_args,)* #module_name::Merge<F, F2>> {
                #struct_name {
                    #(#field_merges),*
                }
            }
        }
    }
}
