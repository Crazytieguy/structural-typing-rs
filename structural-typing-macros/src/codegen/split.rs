use proc_macro2::TokenStream;
use quote::quote;

use crate::parsing::StructInfo;

fn sanitize_ident(ident: &syn::Ident) -> String {
    let s = ident.to_string();
    s.strip_prefix("r#").unwrap_or(&s).to_string()
}

pub fn generate(info: &StructInfo) -> TokenStream {
    let struct_name = &info.name;
    let module_name = &info.module_name;
    let field_names: Vec<_> = info.fields.iter().map(|f| &f.name).collect();

    let split_where_clauses = info.fields.iter().map(|field| {
        let field_name = &field.name;
        quote! {
            F::#field_name: ::structural_typing::split::Split<F2::#field_name>
        }
    });

    let split_field_splits = info.fields.iter().map(|field| {
        let field_name = &field.name;
        let sanitized = sanitize_ident(field_name);
        let field_name_r = syn::Ident::new(&format!("{}_r", sanitized), field_name.span());
        let field_name_o = syn::Ident::new(&format!("{}_o", sanitized), field_name.span());
        quote! {
            let (#field_name_o, #field_name_r) = <F::#field_name as ::structural_typing::split::Split<F2::#field_name>>::split(self.#field_name);
        }
    });

    let split_remainder_fields: Vec<_> = info.fields.iter().map(|field| {
        let field_name = &field.name;
        let sanitized = sanitize_ident(field_name);
        let field_name_r = syn::Ident::new(&format!("{}_r", sanitized), field_name.span());
        quote! {
            #field_name: #field_name_r
        }
    }).collect();

    let split_output_fields: Vec<_> = info.fields.iter().map(|field| {
        let field_name = &field.name;
        let sanitized = sanitize_ident(field_name);
        let field_name_o = syn::Ident::new(&format!("{}_o", sanitized), field_name.span());
        quote! {
            #field_name: #field_name_o
        }
    }).collect();

    let try_split_where_clauses = info.fields.iter().map(|field| {
        let field_name = &field.name;
        quote! {
            F::#field_name: ::structural_typing::split::TrySplit<F2::#field_name>,
            <<F2::#field_name as ::structural_typing::presence::Presence>::RemainderFrom<F::#field_name> as ::structural_typing::presence::Presence>::Or<F2::#field_name>: ::structural_typing::split::TrySplit<F::#field_name>
        }
    });

    let try_split_field_splits = info.fields.iter().enumerate().map(|(idx, field)| {
        let field_name = &field.name;
        let sanitized = sanitize_ident(field_name);
        let field_name_r = syn::Ident::new(&format!("{}_r", sanitized), field_name.span());
        let field_name_o = syn::Ident::new(&format!("{}_o", sanitized), field_name.span());

        // Generate reconstruction for all previously split fields
        let previous_reconstructions: Vec<_> = info.fields.iter().take(idx).map(|prev_field| {
            let prev_name = &prev_field.name;
            let prev_sanitized = sanitize_ident(prev_name);
            let prev_r = syn::Ident::new(&format!("{}_r", prev_sanitized), prev_name.span());
            let prev_o = syn::Ident::new(&format!("{}_o", prev_sanitized), prev_name.span());
            quote! {
                #prev_name: match <
                    <<F2::#prev_name as ::structural_typing::presence::Presence>::RemainderFrom<F::#prev_name> as ::structural_typing::presence::Presence>::Or<F2::#prev_name>
                    as ::structural_typing::split::TrySplit<F::#prev_name>
                >::try_split(
                    <<F2::#prev_name as ::structural_typing::presence::Presence>::RemainderFrom<F::#prev_name> as ::structural_typing::presence::Presence>::or(
                        #prev_r,
                        #prev_o
                    )
                ) {
                    Ok((reconstructed, _)) => reconstructed,
                    Err(_) => unreachable!("reconstruction from split parts cannot fail"),
                }
            }
        }).collect();

        // Fields not yet consumed
        let unconsumed_fields: Vec<_> = field_names.iter().skip(idx + 1).map(|name| {
            quote! { #name }
        }).collect();

        quote! {
            let (#field_name_o, #field_name_r) = match <F::#field_name as ::structural_typing::split::TrySplit<F2::#field_name>>::try_split(#field_name) {
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
        impl<F: #module_name::Fields> #struct_name<F> {
            /// Splits into selected fields and remainder. Always succeeds.
            pub fn split<F2: #module_name::Fields>(self) -> (#struct_name<F2>, #struct_name<#module_name::Omit<F, F2>>)
            where
                #(#split_where_clauses),*
            {
                #(#split_field_splits)*

                (#struct_name {
                    #(#split_output_fields),*
                }, #struct_name {
                    #(#split_remainder_fields),*
                })
            }

            /// Splits into selected fields and remainder. Returns `Err(self)` if any Optional field is None but target needs Present.
            pub fn try_split<F2: #module_name::Fields>(self) -> Result<(#struct_name<F2>, #struct_name<#module_name::Omit<F, F2>>), Self>
            where
                #(#try_split_where_clauses),*
            {
                let #struct_name { #(#field_names),* } = self;

                #(#try_split_field_splits)*

                Ok((#struct_name {
                    #(#split_output_fields),*
                }, #struct_name {
                    #(#split_remainder_fields),*
                }))
            }
        }
    }
}
