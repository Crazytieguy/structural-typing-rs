use proc_macro2::TokenStream;
use quote::quote;

use crate::parsing::StructInfo;

pub fn generate(info: &StructInfo) -> TokenStream {
    let struct_name = &info.name;
    let module_name = &info.module_name;

    let project_where_clauses = info.fields.iter().map(|field| {
        let field_name = &field.name;
        quote! {
            F::#field_name: ::structural_typing::presence::Project<F2::#field_name>
        }
    });

    let project_fields = info.fields.iter().map(|field| {
        let field_name = &field.name;
        quote! {
            #field_name: <F::#field_name as ::structural_typing::presence::Project<F2::#field_name>>::project(self.#field_name)
        }
    });

    let try_project_where_clauses = info.fields.iter().map(|field| {
        let field_name = &field.name;
        quote! {
            F::#field_name: ::structural_typing::presence::TryProject<F2::#field_name>
        }
    });

    let try_project_fields = info.fields.iter().map(|field| {
        let field_name = &field.name;
        quote! {
            #field_name: <F::#field_name as ::structural_typing::presence::TryProject<F2::#field_name>>::try_project(self.#field_name)?
        }
    });

    quote! {
        impl<F: #module_name::Fields> #struct_name<F> {
            pub fn project<F2: #module_name::Fields>(self) -> #struct_name<F2>
            where
                #(#project_where_clauses),*
            {
                #struct_name {
                    #(#project_fields),*
                }
            }

            pub fn try_project<F2: #module_name::Fields>(self) -> Option<#struct_name<F2>>
            where
                #(#try_project_where_clauses),*
            {
                Some(#struct_name {
                    #(#try_project_fields),*
                })
            }
        }
    }
}
