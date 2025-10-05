use syn::{DeriveInput, Data, Fields, Field, Error, Result};

pub struct StructuralStruct {
    pub vis: syn::Visibility,
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    pub fields: Vec<StructField>,
    pub attrs: Vec<syn::Attribute>,
}

pub struct StructField {
    pub ident: syn::Ident,
    pub ty: syn::Type,
    pub vis: syn::Visibility,
    pub always_present: bool,
}

pub fn parse_structural_struct(input: DeriveInput) -> Result<StructuralStruct> {
    let Data::Struct(data_struct) = &input.data else {
        return Err(Error::new_spanned(&input, "#[structural] can only be used on structs"));
    };

    let Fields::Named(fields) = &data_struct.fields else {
        return Err(Error::new_spanned(&input, "#[structural] requires named fields"));
    };
    let fields = &fields.named;

    let struct_fields = fields
        .iter()
        .map(|f: &Field| {
            let ident = f.ident.clone().ok_or_else(|| {
                Error::new_spanned(f, "Field must have a name")
            })?;

            let always_present = f.attrs.iter().any(|attr| {
                attr.path().is_ident("always")
            });

            Ok(StructField {
                ident,
                ty: f.ty.clone(),
                vis: f.vis.clone(),
                always_present,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(StructuralStruct {
        vis: input.vis,
        ident: input.ident,
        generics: input.generics,
        fields: struct_fields,
        attrs: input.attrs,
    })
}
