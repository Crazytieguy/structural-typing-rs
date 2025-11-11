use proc_macro2::TokenStream;
use quote::quote;

use crate::codegen::generics_utils::{impl_generics_with_f, remainder_type_args};
use crate::parsing::StructInfo;

fn sanitize_ident(ident: &syn::Ident) -> String {
    let s = ident.to_string();
    s.strip_prefix("r#").unwrap_or(&s).to_string()
}

pub fn generate(info: &StructInfo) -> TokenStream {
    let struct_name = &info.name;
    let module_name = &info.module_name;

    let (impl_generics, user_type_args) = impl_generics_with_f(&info.generics, module_name);
    let (impl_generics, _, where_clause) = impl_generics.split_for_impl();
    let remainder_type_args = remainder_type_args(&info.generics, &user_type_args);

    let field_names: Vec<_> = info.fields.iter().map(|f| &f.name).collect();

    let extract_where_clauses = info.fields.iter().map(|field| {
        let field_name = &field.name;
        let field_type = &field.ty;
        quote! {
            <F::#field_name as ::structural_typing::presence::Presence>::Output<#field_type>: ::structural_typing::extract::Extract<<F2::#field_name as ::structural_typing::presence::Presence>::Output<#field_type>, #field_type>
        }
    });

    let extract_field_extracts = info.fields.iter().map(|field| {
        let field_name = &field.name;
        let field_type = &field.ty;
        let sanitized = sanitize_ident(field_name);
        let field_name_r = syn::Ident::new(&format!("{}_r", sanitized), field_name.span());
        let field_name_o = syn::Ident::new(&format!("{}_o", sanitized), field_name.span());
        quote! {
            let (#field_name_o, #field_name_r) = <<F::#field_name as ::structural_typing::presence::Presence>::Output<#field_type> as ::structural_typing::extract::Extract<<F2::#field_name as ::structural_typing::presence::Presence>::Output<#field_type>, #field_type>>::extract(self.#field_name);
        }
    });

    let extract_remainder_fields: Vec<_> = info
        .fields
        .iter()
        .map(|field| {
            let field_name = &field.name;
            let sanitized = sanitize_ident(field_name);
            let field_name_r = syn::Ident::new(&format!("{}_r", sanitized), field_name.span());
            quote! {
                #field_name: #field_name_r
            }
        })
        .collect();

    let extract_output_fields: Vec<_> = info
        .fields
        .iter()
        .map(|field| {
            let field_name = &field.name;
            let sanitized = sanitize_ident(field_name);
            let field_name_o = syn::Ident::new(&format!("{}_o", sanitized), field_name.span());
            quote! {
                #field_name: #field_name_o
            }
        })
        .collect();

    let try_extract_where_clauses = info.fields.iter().map(|field| {
        let field_name = &field.name;
        let field_type = &field.ty;
        quote! {
            <F::#field_name as ::structural_typing::presence::Presence>::Output<#field_type>: ::structural_typing::extract::TryExtract<<F2::#field_name as ::structural_typing::presence::Presence>::Output<#field_type>, #field_type>,
            <<
                <<<F2::#field_name as ::structural_typing::presence::Presence>::Output<#field_type> as ::structural_typing::access::Access<#field_type>>::RemainderFrom<<F::#field_name as ::structural_typing::presence::Presence>::Output<#field_type>>
                as ::structural_typing::presence::InferPresence<#field_type>>::Presence
                as ::structural_typing::presence::Presence
            >::Or<F2::#field_name>
            as ::structural_typing::presence::Presence>::Output<#field_type>: ::structural_typing::extract::TryExtract<<F::#field_name as ::structural_typing::presence::Presence>::Output<#field_type>, #field_type>
        }
    });

    let try_extract_field_extracts = info.fields.iter().enumerate().map(|(idx, field)| {
        let field_name = &field.name;
        let field_type = &field.ty;
        let sanitized = sanitize_ident(field_name);
        let field_name_r = syn::Ident::new(&format!("{}_r", sanitized), field_name.span());
        let field_name_o = syn::Ident::new(&format!("{}_o", sanitized), field_name.span());

        // Generate reconstruction for all previously extracted fields
        let previous_reconstructions: Vec<_> = info.fields.iter().take(idx).map(|prev_field| {
            let prev_name = &prev_field.name;
            let prev_type = &prev_field.ty;
            let prev_sanitized = sanitize_ident(prev_name);
            let prev_r = syn::Ident::new(&format!("{}_r", prev_sanitized), prev_name.span());
            let prev_o = syn::Ident::new(&format!("{}_o", prev_sanitized), prev_name.span());
            quote! {
                #prev_name: match <
                    <<
                        <<<F2::#prev_name as ::structural_typing::presence::Presence>::Output<#prev_type> as ::structural_typing::access::Access<#prev_type>>::RemainderFrom<<F::#prev_name as ::structural_typing::presence::Presence>::Output<#prev_type>>
                        as ::structural_typing::presence::InferPresence<#prev_type>>::Presence
                        as ::structural_typing::presence::Presence
                    >::Or<F2::#prev_name>
                    as ::structural_typing::presence::Presence>::Output<#prev_type>
                    as ::structural_typing::extract::TryExtract<<F::#prev_name as ::structural_typing::presence::Presence>::Output<#prev_type>, #prev_type>
                >::try_extract(
                    <
                        <<<F2::#prev_name as ::structural_typing::presence::Presence>::Output<#prev_type> as ::structural_typing::access::Access<#prev_type>>::RemainderFrom<<F::#prev_name as ::structural_typing::presence::Presence>::Output<#prev_type>>
                        as ::structural_typing::presence::InferPresence<#prev_type>>::Presence
                        as ::structural_typing::presence::Presence
                    >::or(#prev_r, #prev_o)
                ) {
                    Ok((reconstructed, _)) => reconstructed,
                    Err(_) => unreachable!("reconstruction from extracted parts cannot fail"),
                }
            }
        }).collect();

        // Fields not yet consumed - keep as identifiers for struct destructuring
        let unconsumed_fields: Vec<_> = field_names.iter().skip(idx + 1).collect();

        quote! {
            let (#field_name_o, #field_name_r) = match <<F::#field_name as ::structural_typing::presence::Presence>::Output<#field_type> as ::structural_typing::extract::TryExtract<<F2::#field_name as ::structural_typing::presence::Presence>::Output<#field_type>, #field_type>>::try_extract(#field_name) {
                Ok(result) => result,
                Err(original_field) => {
                    return Err(#struct_name {
                        #(#previous_reconstructions,)*
                        #field_name: original_field,
                        #(#unconsumed_fields),*
                    });
                }
            };
        }
    });

    quote! {
        impl #impl_generics #struct_name<#(#user_type_args,)* F> #where_clause {
            /// Extracts selected fields and remainder. Always succeeds.
            pub fn extract<F2: #module_name::Fields>(self) -> (#struct_name<#(#user_type_args,)* F2>, #struct_name<#(#user_type_args,)* #module_name::Remainder #remainder_type_args>)
            where
                #(#extract_where_clauses),*
            {
                #(#extract_field_extracts)*

                (#struct_name {
                    #(#extract_output_fields),*
                }, #struct_name {
                    #(#extract_remainder_fields),*
                })
            }

            /// Extracts selected fields and remainder. Returns `Err(self)` if any Optional field is None but target needs Present.
            pub fn try_extract<F2: #module_name::Fields>(self) -> Result<(#struct_name<#(#user_type_args,)* F2>, #struct_name<#(#user_type_args,)* #module_name::Remainder #remainder_type_args>), Self>
            where
                #(#try_extract_where_clauses),*
            {
                let #struct_name { #(#field_names),* } = self;

                #(#try_extract_field_extracts)*

                Ok((#struct_name {
                    #(#extract_output_fields),*
                }, #struct_name {
                    #(#extract_remainder_fields),*
                }))
            }
        }
    }
}
