use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

use crate::parsing::StructInfo;

fn generate_field_types_with_override(
    info: &StructInfo,
    target_field: &Ident,
    override_presence: TokenStream,
) -> Vec<TokenStream> {
    info.fields
        .iter()
        .map(|f| {
            if f.name == *target_field {
                override_presence.clone()
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
        let maybe_method = format_ident!("maybe_{}", field_name);
        let unset_method = format_ident!("unset_{}", field_name);

        let present_field_types = generate_field_types_with_override(
            info,
            field_name,
            quote! { ::structural_typing::presence::Present }
        );

        let optional_field_types = generate_field_types_with_override(
            info,
            field_name,
            quote! { ::structural_typing::presence::Optional }
        );

        let absent_field_types = generate_field_types_with_override(
            info,
            field_name,
            quote! { ::structural_typing::presence::Absent }
        );

        let field_assignments: Vec<_> = info.fields.iter().map(|f| {
            let name = &f.name;
            if f.name == *field_name {
                quote! { #name }
            } else {
                quote! { #name: self.#name }
            }
        }).collect();

        let field_assignments_unset: Vec<_> = info.fields.iter().map(|f| {
            let name = &f.name;
            if f.name == *field_name {
                quote! { #name: ::std::marker::PhantomData }
            } else {
                quote! { #name: self.#name }
            }
        }).collect();

        quote! {
            /// Sets this field to Present with the given value.
            pub fn #field_name(self, #field_name: #field_ty) -> #struct_name<#module_name::FieldSet<
                #(#present_field_types),*
            >> {
                #struct_name {
                    #(#field_assignments),*
                }
            }

            /// Sets this field to Optional with the given Option value.
            pub fn #maybe_method(self, #field_name: Option<#field_ty>) -> #struct_name<#module_name::FieldSet<
                #(#optional_field_types),*
            >> {
                #struct_name {
                    #(#field_assignments),*
                }
            }

            /// Sets this field to Absent.
            pub fn #unset_method(self) -> #struct_name<#module_name::FieldSet<
                #(#absent_field_types),*
            >> {
                #struct_name {
                    #(#field_assignments_unset),*
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
