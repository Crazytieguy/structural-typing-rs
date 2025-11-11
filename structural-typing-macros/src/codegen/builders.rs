use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::codegen::generics_utils::impl_generics_with_f;
use crate::parsing::StructInfo;

fn generate_field_types_with_inferred(info: &StructInfo, target_field: &Ident) -> Vec<TokenStream> {
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

    let (impl_generics, user_type_args) = impl_generics_with_f(&info.generics, module_name);
    let (impl_generics, _, where_clause) = impl_generics.split_for_impl();

    let methods = info.fields.iter().map(|field| {
        let field_name = &field.name;
        let field_ty = &field.ty;

        let field_types = generate_field_types_with_inferred(info, field_name);

        let field_assignments: Vec<_> = info
            .fields
            .iter()
            .map(|f| {
                let name = &f.name;
                if f.name == *field_name {
                    quote! { #name }
                } else {
                    quote! { #name: self.#name }
                }
            })
            .collect();

        quote! {
            #[must_use]
            pub fn #field_name<V: ::structural_typing::presence::InferPresence<#field_ty>>(
                self,
                #field_name: V
            ) -> #struct_name<#(#user_type_args,)* #module_name::FieldSet<
                #(#field_types),*
            >> {
                #struct_name {
                    #(#field_assignments),*
                }
            }
        }
    });

    quote! {
        impl #impl_generics #struct_name<#(#user_type_args,)* F> #where_clause {
            #(#methods)*
        }
    }
}
