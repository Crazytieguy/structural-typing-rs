use proc_macro2::TokenStream;
use quote::quote;

use crate::codegen::generics_utils::extract_type_param_idents;
use crate::parsing::StructInfo;

pub fn generate(info: &StructInfo) -> TokenStream {
    let struct_name = &info.name;
    let module_name = &info.module_name;

    let (impl_generics, _, where_clause) = info.generics.split_for_impl();

    let user_type_args = extract_type_param_idents(&info.generics.params);
    let field_names = info.fields.iter().map(|f| &f.name);

    quote! {
        impl #impl_generics #struct_name<#(#user_type_args,)* #module_name::with::all<::structural_typing::presence::Absent>> #where_clause {
            /// Creates an empty instance with all fields absent.
            pub fn empty() -> Self {
                #struct_name {
                    #(#field_names: ::core::marker::PhantomData),*
                }
            }
        }
    }
}
