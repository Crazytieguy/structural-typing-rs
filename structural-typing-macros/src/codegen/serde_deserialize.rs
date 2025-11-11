use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Ident};

use crate::parsing::StructInfo;

pub fn generate(info: &StructInfo) -> syn::Result<(Option<TokenStream>, Option<TokenStream>)> {
    let has_deserialize = info.derives.iter().any(|d| d == "Deserialize");

    if !has_deserialize {
        return Ok((None, None));
    }

    validate_serde_attributes(info)?;

    let helper_struct = generate_helper_struct(info);
    let try_from_impl = generate_try_from_impl(info)?;

    Ok((Some(helper_struct), Some(try_from_impl)))
}

fn validate_serde_attributes(info: &StructInfo) -> syn::Result<()> {
    for attr in &info.other_attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }

        let attr_clone = attr.clone();
        let mut has_error = None;
        let _ = attr_clone.parse_nested_meta(|meta| {
            let path = meta.path.get_ident().map(|i| i.to_string());
            match path.as_deref() {
                Some("default") => {
                    has_error = Some(syn::Error::new_spanned(
                        attr,
                        "structural typing does not support #[serde(default)] on containers",
                    ));
                }
                Some("transparent") => {
                    has_error = Some(syn::Error::new_spanned(
                        attr,
                        "structural typing does not support #[serde(transparent)]",
                    ));
                }
                _ => {}
            }
            Ok(())
        });

        if let Some(err) = has_error {
            return Err(err);
        }
    }

    for field in &info.fields {
        for attr in &field.attrs {
            if !attr.path().is_ident("serde") {
                continue;
            }

            let attr_clone = attr.clone();
            let mut has_error = None;
            let _ = attr_clone.parse_nested_meta(|meta| {
                let path = meta.path.get_ident().map(|i| i.to_string());
                match path.as_deref() {
                    Some("default") => {
                        has_error = Some(syn::Error::new_spanned(
                            attr,
                            "structural typing does not support #[serde(default)] on fields",
                        ));
                    }
                    Some("skip") | Some("skip_deserializing") => {
                        has_error = Some(syn::Error::new_spanned(
                            attr,
                            "structural typing does not support #[serde(skip)] or #[serde(skip_deserializing)]",
                        ));
                    }
                    Some("flatten") => {
                        has_error = Some(syn::Error::new_spanned(
                            attr,
                            "structural typing does not support #[serde(flatten)]",
                        ));
                    }
                    _ => {}
                }
                Ok(())
            });

            if let Some(err) = has_error {
                return Err(err);
            }
        }
    }

    Ok(())
}

fn generate_helper_struct(info: &StructInfo) -> TokenStream {
    let helper_name = helper_struct_name(&info.name);
    let fields = info.fields.iter().map(|field| {
        let name = &field.name;
        let ty = &field.ty;
        let preserved_attrs = filter_deserialize_attrs(&field.attrs);

        quote! {
            #(#preserved_attrs)*
            #[serde(default)]
            pub #name: Option<#ty>
        }
    });

    let preserved_struct_attrs = filter_container_serde_attrs(&info.other_attrs);

    quote! {
        #[derive(::serde::Deserialize)]
        #[doc(hidden)]
        #(#preserved_struct_attrs)*
        pub(super) struct #helper_name {
            #(#fields),*
        }
    }
}

fn generate_try_from_impl(info: &StructInfo) -> syn::Result<TokenStream> {
    let name = &info.name;
    let module_name = &info.module_name;
    let helper_name = helper_struct_name(&info.name);

    let field_conversions = info.fields.iter().map(|field| {
        let field_name = &field.name;
        let field_name_str = field_name.to_string();

        quote! {
            #field_name: value.#field_name
                .try_extract()
                .map_err(|_| missing_field(#field_name_str))?
                .0
        }
    });

    let where_bounds = info.fields.iter().map(|field| {
        let field_name = &field.name;
        let field_ty = &field.ty;
        quote! {
            Option<#field_ty>: ::structural_typing::extract::TryExtract<<<F as #module_name::Fields>::#field_name as ::structural_typing::presence::Presence>::Output<#field_ty>, #field_ty>
        }
    });

    Ok(quote! {
        impl<F: #module_name::Fields> ::core::convert::TryFrom<#module_name::#helper_name> for #name<F>
        where
            #(#where_bounds),*
        {
            type Error = String;

            fn try_from(value: #module_name::#helper_name) -> ::core::result::Result<Self, Self::Error> {
                use ::structural_typing::extract::TryExtract;

                let missing_field = |field: &'static str| -> String {
                    format!("missing field `{}`", field)
                };

                Ok(Self {
                    #(#field_conversions),*
                })
            }
        }
    })
}

fn helper_struct_name(struct_name: &Ident) -> Ident {
    Ident::new(
        &format!("__{}Deserialize", struct_name),
        struct_name.span(),
    )
}

pub fn helper_path(module_name: &Ident, struct_name: &Ident) -> String {
    format!("{}::{}",
        module_name,
        helper_struct_name(struct_name))
}

fn filter_deserialize_attrs(attrs: &[Attribute]) -> Vec<Attribute> {
    attrs
        .iter()
        .filter(|attr| {
            if !attr.path().is_ident("serde") {
                return false;
            }

            let mut should_keep = true;
            let _ = attr.parse_nested_meta(|meta| {
                if let Some(ident) = meta.path.get_ident() {
                    let ident_str = ident.to_string();
                    if ident_str == "skip_serializing"
                        || ident_str == "serialize_with"
                    {
                        should_keep = false;
                    }
                }
                Ok(())
            });

            should_keep
        })
        .cloned()
        .collect()
}

fn filter_container_serde_attrs(attrs: &[Attribute]) -> Vec<Attribute> {
    attrs
        .iter()
        .filter(|attr| attr.path().is_ident("serde"))
        .cloned()
        .collect()
}

pub fn filter_serialize_attrs(attrs: &[Attribute]) -> Vec<Attribute> {
    attrs
        .iter()
        .filter(|attr| {
            if !attr.path().is_ident("serde") {
                return false;
            }

            let mut should_keep = true;
            let _ = attr.parse_nested_meta(|meta| {
                if let Some(ident) = meta.path.get_ident() {
                    let ident_str = ident.to_string();
                    if ident_str == "skip_deserializing"
                        || ident_str == "deserialize_with"
                        || ident_str == "alias"
                    {
                        should_keep = false;
                    }
                }
                Ok(())
            });

            should_keep
        })
        .cloned()
        .collect()
}
