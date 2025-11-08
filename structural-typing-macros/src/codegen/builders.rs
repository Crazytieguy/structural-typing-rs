use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::parsing::StructInfo;

fn generate_field_types_with_inferred(
    info: &StructInfo,
    target_field: &Ident,
) -> Vec<TokenStream> {
    info.fields
        .iter()
        .map(|f| {
            if f.name == *target_field {
                quote! { V::Presence }
            } else {
                let name = &f.name;
                quote! { F::#name }
            }
        })
        .collect()
}

pub fn generate(info: &StructInfo) -> TokenStream {
    let struct_name = &info.name;
    let module_name = &info.module_name;

    let methods = info.fields.iter().map(|field| {
        let field_name = &field.name;
        let field_ty = &field.ty;

        let field_types = generate_field_types_with_inferred(info, field_name);

        let field_assignments: Vec<_> = info.fields.iter().map(|f| {
            let name = &f.name;
            if f.name == *field_name {
                quote! { #name }
            } else {
                quote! { #name: self.#name }
            }
        }).collect();

        quote! {
            pub fn #field_name<V: ::structural_typing::presence::InferPresence<#field_ty>>(
                self,
                #field_name: V
            ) -> #struct_name<#module_name::FieldSet<
                #(#field_types),*
            >> {
                #struct_name {
                    #(#field_assignments),*
                }
            }
        }
    });

    quote! {
        impl<F: #module_name::Fields> #struct_name<F> {
            #(#methods)*
        }
    }
}
