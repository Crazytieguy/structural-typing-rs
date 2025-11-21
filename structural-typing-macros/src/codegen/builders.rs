use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::Ident;

use crate::codegen::generics_utils::{impl_generics_with_f, type_args_with_f};
use crate::codegen::type_subst;
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

        let single_field_generic_infos = type_subst::extract_single_field_generics_from_type(
            field_ty,
            &info.single_field_generics,
            &info.generics,
        );

        let field_set = quote! { #module_name::FieldSet<#(#field_types),*> };
        let return_type_args = type_args_with_f(&info.generics, &user_type_args, field_set);

        if single_field_generic_infos.is_empty() {
            quote! {
                #[must_use]
                pub fn #field_name<V: ::structural_typing::presence::InferPresence<#field_ty>>(
                    self,
                    #field_name: V
                ) -> #struct_name #return_type_args {
                    #struct_name {
                        #(#field_assignments),*
                    }
                }
            }
        } else {
            let subst_map: HashMap<String, Ident> = single_field_generic_infos
                .iter()
                .map(|info| (info.original_ident.to_string(), info.new_ident.clone()))
                .collect();

            let substituted_field_ty = type_subst::substitute_type(field_ty, &subst_map);
            let substituted_user_type_args = type_subst::substitute_type_args(&user_type_args, &info.generics, &subst_map);

            let field_set = quote! { #module_name::FieldSet<#(#field_types),*> };
            let substituted_return_type_args = type_args_with_f(&info.generics, &substituted_user_type_args, field_set);

            let new_generic_params = single_field_generic_infos.iter().map(|info| {
                let new_ident = &info.new_ident;
                let bounds = &info.bounds;
                quote! { #new_ident: #(#bounds)+* }
            });

            quote! {
                #[must_use]
                pub fn #field_name<#(#new_generic_params,)* V: ::structural_typing::presence::InferPresence<#substituted_field_ty>>(
                    self,
                    #field_name: V
                ) -> #struct_name #substituted_return_type_args {
                    #struct_name {
                        #(#field_assignments),*
                    }
                }
            }
        }
    });

    let impl_type_args = type_args_with_f(&info.generics, &user_type_args, quote! { F });

    quote! {
        impl #impl_generics #struct_name #impl_type_args #where_clause {
            #(#methods)*
        }
    }
}
