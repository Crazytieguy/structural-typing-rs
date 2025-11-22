use heck::ToSnakeCase;
use proc_macro2::Span;
use std::collections::HashSet;
use syn::{
    Attribute, Data, DeriveInput, Fields, Generics, Ident, Path, Type, Visibility, spanned::Spanned,
};

use crate::analysis;

#[derive(Debug, Clone)]
pub struct NestedFieldsInfo {
    pub module_path: Path,
    pub field_names: Vec<Ident>,
}

fn parse_nested_attribute(attrs: &[Attribute]) -> syn::Result<Option<NestedFieldsInfo>> {
    use syn::Token;
    use syn::punctuated::Punctuated;

    for attr in attrs {
        if attr.path().is_ident("nested") {
            return attr.parse_args_with(|input: syn::parse::ParseStream| {
                // Parse module path
                let module_path: Path = input.parse()?;

                // Parse colon
                input.parse::<Token![:]>()?;

                // Parse comma-separated field names
                let field_idents = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;

                if field_idents.is_empty() {
                    return Err(syn::Error::new(
                        input.span(),
                        "#[nested] attribute requires at least one field name after the module path",
                    ));
                }

                // Check for duplicates
                let mut seen = HashSet::new();
                let mut field_names = Vec::new();

                for ident in field_idents {
                    if !seen.insert(ident.to_string()) {
                        return Err(syn::Error::new_spanned(
                            &ident,
                            format!("duplicate nested field name '{}'", ident),
                        ));
                    }
                    field_names.push(ident);
                }

                Ok(Some(NestedFieldsInfo {
                    module_path,
                    field_names,
                }))
            });
        }
    }
    Ok(None)
}

#[derive(Debug)]
pub struct StructInfo {
    pub name: Ident,
    pub module_name: Ident,
    pub vis: Visibility,
    pub fields: Vec<FieldInfo>,
    pub derives: Vec<Ident>,
    pub other_attrs: Vec<Attribute>,
    pub generics: Generics,
    pub single_field_generics: HashSet<String>,
}

#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub name: Ident,
    pub ty: Type,
    pub vis: Visibility,
    pub attrs: Vec<Attribute>,
    pub nested_fields: Option<NestedFieldsInfo>,
}

pub fn parse_struct(input: DeriveInput) -> syn::Result<StructInfo> {
    let name = input.ident.clone();
    let span = input.span();

    let Data::Struct(data_struct) = input.data else {
        return Err(syn::Error::new(
            span,
            "#[structural] can only be applied to structs",
        ));
    };

    let Fields::Named(fields_named) = data_struct.fields else {
        return Err(syn::Error::new(
            data_struct.fields.span(),
            "#[structural] requires named fields",
        ));
    };

    let fields: Result<Vec<FieldInfo>, syn::Error> = fields_named
        .named
        .into_iter()
        .map(|field| {
            let name = field.ident.expect("named field must have an ident");
            let ty = field.ty;
            let vis = field.vis;
            let attrs = field.attrs;
            let nested_fields = parse_nested_attribute(&attrs)?;
            Ok(FieldInfo {
                name,
                ty,
                vis,
                attrs,
                nested_fields,
            })
        })
        .collect();

    let fields = fields?;

    if fields.is_empty() {
        return Err(syn::Error::new(
            span,
            "#[structural] requires at least one field",
        ));
    }

    // Check for invalid nested annotations on single-field structs
    if fields.len() == 1 && fields[0].nested_fields.is_some() {
        return Err(syn::Error::new_spanned(
            &fields[0].name,
            "nested setters require at least 2 fields in the struct. \
             Single-field structs cannot use #[nested] because there are no \
             other fields to preserve with the spread operator. \
             Consider adding another field or using regular setters instead.",
        ));
    }

    let module_name = Ident::new(&name.to_string().to_snake_case(), Span::call_site());

    let (derives, other_attrs) = split_derives_and_attrs(input.attrs)?;

    let field_types: Vec<(String, Type)> = fields
        .iter()
        .map(|f| (f.name.to_string(), f.ty.clone()))
        .collect();

    let usage_map = analysis::analyze_generic_usage(&input.generics, &field_types);
    let single_field_generics = analysis::identify_single_field_generics(&usage_map);

    Ok(StructInfo {
        name,
        module_name,
        vis: input.vis,
        fields,
        derives,
        other_attrs,
        generics: input.generics,
        single_field_generics,
    })
}

fn split_derives_and_attrs(attrs: Vec<Attribute>) -> syn::Result<(Vec<Ident>, Vec<Attribute>)> {
    let mut derives = Vec::new();
    let mut other_attrs = Vec::new();

    for attr in attrs {
        if attr.path().is_ident("derive") {
            attr.parse_nested_meta(|meta| {
                if let Some(ident) = meta.path.get_ident() {
                    derives.push(ident.clone());
                } else if let Some(last_segment) = meta.path.segments.last() {
                    derives.push(last_segment.ident.clone());
                }
                Ok(())
            })?;
        } else {
            other_attrs.push(attr);
        }
    }

    Ok((derives, other_attrs))
}
