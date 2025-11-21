use proc_macro2::TokenStream;
use quote::quote;

use crate::codegen::generics_utils::{impl_generics_with_f, type_args_with_f};
use crate::parsing::StructInfo;

pub fn generate(info: &StructInfo) -> TokenStream {
    let struct_name = &info.name;
    let module_name = &info.module_name;

    let (impl_generics, user_type_args) = impl_generics_with_f(&info.generics, module_name);
    let (impl_generics, _, where_clause) = impl_generics.split_for_impl();
    let impl_type_args = type_args_with_f(&info.generics, &user_type_args, quote! { F });
    let f2_type_args = type_args_with_f(&info.generics, &user_type_args, quote! { F2 });
    let merge_type_args = type_args_with_f(
        &info.generics,
        &user_type_args,
        quote! { #module_name::Merge<F, F2> },
    );

    let field_merges = info.fields.iter().map(|field| {
        let field_name = &field.name;
        quote! {
            #field_name: <F2::#field_name as ::structural_typing::presence::Presence>::or(other.#field_name, self.#field_name)
        }
    });

    quote! {
        impl #impl_generics #struct_name #impl_type_args #where_clause {
            /// Combines two instances, preferring `other`'s fields when both are present.
            pub fn merge<F2: #module_name::Fields>(self, other: #struct_name #f2_type_args) -> #struct_name #merge_type_args {
                #struct_name {
                    #(#field_merges),*
                }
            }
        }
    }
}
