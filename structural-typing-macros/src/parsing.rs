use heck::ToSnakeCase;
use proc_macro2::Span;
use syn::{Attribute, Data, DeriveInput, Fields, Ident, Type, Visibility, spanned::Spanned};

#[derive(Debug)]
pub struct StructInfo {
    pub name: Ident,
    pub module_name: Ident,
    pub vis: Visibility,
    pub fields: Vec<FieldInfo>,
    pub derives: Vec<Ident>,
    pub other_attrs: Vec<Attribute>,
}

#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub name: Ident,
    pub ty: Type,
    pub vis: Visibility,
    pub attrs: Vec<Attribute>,
}

pub fn parse_struct(input: DeriveInput) -> syn::Result<StructInfo> {
    let name = input.ident.clone();
    let span = input.span();

    if !input.generics.params.is_empty() {
        return Err(syn::Error::new(
            input.generics.span(),
            "structural typing does not support generic structs yet",
        ));
    }

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

    let fields: Vec<FieldInfo> = fields_named
        .named
        .into_iter()
        .map(|field| {
            let name = field.ident.expect("named field must have an ident");
            let ty = field.ty;
            let vis = field.vis;
            let attrs = field.attrs;
            FieldInfo {
                name,
                ty,
                vis,
                attrs,
            }
        })
        .collect();

    if fields.is_empty() {
        return Err(syn::Error::new(
            span,
            "#[structural] requires at least one field",
        ));
    }

    let module_name = Ident::new(&name.to_string().to_snake_case(), Span::call_site());

    let (derives, other_attrs) = split_derives_and_attrs(input.attrs)?;

    Ok(StructInfo {
        name,
        module_name,
        vis: input.vis,
        fields,
        derives,
        other_attrs,
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
