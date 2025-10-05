//! Derive macro implementation generators.

use crate::parsing::StructuralStruct;
use proc_macro2::TokenStream;
use quote::{quote, format_ident};

/// Generates trait implementations for requested derive macros.
pub fn generate_derive_impls(structural: &StructuralStruct) -> TokenStream {
    // Extract derive trait names from attributes
    let mut derives = Vec::new();
    for attr in &structural.attrs {
        if attr.path().is_ident("derive") {
            let _ = attr.parse_nested_meta(|meta| {
                if let Some(ident) = meta.path.get_ident() {
                    derives.push(ident.to_string());
                }
                Ok(())
            });
        }
    }

    let struct_ident = &structural.ident;
    let state_mod_name = format_ident!("{}_state", struct_ident.to_string().to_lowercase());

    let mut impls = Vec::new();

    // Generate implementations for each requested derive
    for derive in &derives {
        match derive.as_str() {
            "Debug" => impls.push(generate_debug(structural, &state_mod_name)),
            "Clone" => impls.push(generate_clone(structural, &state_mod_name)),
            "PartialEq" => impls.push(generate_partial_eq(structural, &state_mod_name)),
            "Eq" => impls.push(generate_eq(structural, &state_mod_name)),
            "Hash" => impls.push(generate_hash(structural, &state_mod_name)),
            "Serialize" => impls.push(generate_serialize(structural, &state_mod_name)),
            "Deserialize" => impls.push(generate_deserialize(structural, &state_mod_name)),
            _ => {
                // For unknown derives, emit a warning but don't fail
                // The user might have a custom derive that works
            }
        }
    }

    quote! {
        #( #impls )*
    }
}

fn to_pascal_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;
    for ch in s.chars() {
        if ch == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(ch.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(ch);
        }
    }
    result
}

fn generate_debug(structural: &StructuralStruct, state_mod_name: &syn::Ident) -> TokenStream {
    let struct_ident = &structural.ident;
    let struct_name = struct_ident.to_string();
    let (_impl_generics, ty_generics, where_clause) = structural.generics.split_for_impl();

    let generics = &structural.generics;
    let mut impl_generics = generics.clone();
    impl_generics.params.push(syn::parse_quote!(__State: #state_mod_name::State));

    let field_names: Vec<_> = structural.fields.iter().map(|f| f.ident.to_string()).collect();
    let field_idents: Vec<_> = structural.fields.iter().map(|f| &f.ident).collect();

    let stateful_fields: Vec<_> = structural.fields.iter().filter(|f| !f.always_present).collect();
    let stateful_bounds: Vec<_> = stateful_fields.iter().map(|f| {
        let pascal = format_ident!("{}", to_pascal_case(&f.ident.to_string()));
        let ty = &f.ty;
        quote! {
            <<__State as #state_mod_name::State>::#pascal as ::structural_typing::Presence>::Output<#ty>: ::core::fmt::Debug
        }
    }).collect();

    let (impl_gen, _, _) = impl_generics.split_for_impl();

    quote! {
        impl #impl_gen ::core::fmt::Debug for #struct_ident #ty_generics<__State>
        where
            #where_clause
            __State: ::core::fmt::Debug,
            #( #stateful_bounds, )*
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_struct(#struct_name)
                    #( .field(#field_names, &self.#field_idents) )*
                    .finish()
            }
        }
    }
}

fn generate_clone(structural: &StructuralStruct, state_mod_name: &syn::Ident) -> TokenStream {
    let struct_ident = &structural.ident;
    let (_impl_generics, ty_generics, where_clause) = structural.generics.split_for_impl();

    let generics = &structural.generics;
    let mut impl_generics = generics.clone();
    impl_generics.params.push(syn::parse_quote!(__State: #state_mod_name::State));

    let field_idents: Vec<_> = structural.fields.iter().map(|f| &f.ident).collect();

    let stateful_fields: Vec<_> = structural.fields.iter().filter(|f| !f.always_present).collect();
    let stateful_bounds: Vec<_> = stateful_fields.iter().map(|f| {
        let pascal = format_ident!("{}", to_pascal_case(&f.ident.to_string()));
        let ty = &f.ty;
        quote! {
            <<__State as #state_mod_name::State>::#pascal as ::structural_typing::Presence>::Output<#ty>: ::core::clone::Clone
        }
    }).collect();

    let (impl_gen, _, _) = impl_generics.split_for_impl();

    quote! {
        impl #impl_gen ::core::clone::Clone for #struct_ident #ty_generics<__State>
        where
            #where_clause
            __State: ::core::clone::Clone,
            #( #stateful_bounds, )*
        {
            fn clone(&self) -> Self {
                Self {
                    #( #field_idents: ::core::clone::Clone::clone(&self.#field_idents), )*
                    _phantom: ::core::marker::PhantomData,
                }
            }
        }
    }
}

fn generate_partial_eq(structural: &StructuralStruct, state_mod_name: &syn::Ident) -> TokenStream {
    let struct_ident = &structural.ident;
    let (_impl_generics, ty_generics, where_clause) = structural.generics.split_for_impl();

    let generics = &structural.generics;
    let mut impl_generics = generics.clone();
    impl_generics.params.push(syn::parse_quote!(__State: #state_mod_name::State));

    let field_idents: Vec<_> = structural.fields.iter().map(|f| &f.ident).collect();

    let stateful_fields: Vec<_> = structural.fields.iter().filter(|f| !f.always_present).collect();
    let stateful_bounds: Vec<_> = stateful_fields.iter().map(|f| {
        let pascal = format_ident!("{}", to_pascal_case(&f.ident.to_string()));
        let ty = &f.ty;
        quote! {
            <<__State as #state_mod_name::State>::#pascal as ::structural_typing::Presence>::Output<#ty>: ::core::cmp::PartialEq
        }
    }).collect();

    let (impl_gen, _, _) = impl_generics.split_for_impl();

    quote! {
        impl #impl_gen ::core::cmp::PartialEq for #struct_ident #ty_generics<__State>
        where
            #where_clause
            __State: ::core::cmp::PartialEq,
            #( #stateful_bounds, )*
        {
            fn eq(&self, other: &Self) -> bool {
                true #( && self.#field_idents == other.#field_idents )*
            }
        }
    }
}

fn generate_eq(structural: &StructuralStruct, state_mod_name: &syn::Ident) -> TokenStream {
    let struct_ident = &structural.ident;
    let (_impl_generics, ty_generics, where_clause) = structural.generics.split_for_impl();

    let generics = &structural.generics;
    let mut impl_generics = generics.clone();
    impl_generics.params.push(syn::parse_quote!(__State: #state_mod_name::State));

    let stateful_fields: Vec<_> = structural.fields.iter().filter(|f| !f.always_present).collect();
    let stateful_bounds: Vec<_> = stateful_fields.iter().map(|f| {
        let pascal = format_ident!("{}", to_pascal_case(&f.ident.to_string()));
        let ty = &f.ty;
        quote! {
            <<__State as #state_mod_name::State>::#pascal as ::structural_typing::Presence>::Output<#ty>: ::core::cmp::Eq
        }
    }).collect();

    let (impl_gen, _, _) = impl_generics.split_for_impl();

    quote! {
        impl #impl_gen ::core::cmp::Eq for #struct_ident #ty_generics<__State>
        where
            #where_clause
            __State: ::core::cmp::Eq,
            #( #stateful_bounds, )*
        {}
    }
}

fn generate_hash(structural: &StructuralStruct, state_mod_name: &syn::Ident) -> TokenStream {
    let struct_ident = &structural.ident;
    let (_impl_generics, ty_generics, where_clause) = structural.generics.split_for_impl();

    let generics = &structural.generics;
    let mut impl_generics = generics.clone();
    impl_generics.params.push(syn::parse_quote!(__State: #state_mod_name::State));

    let field_idents: Vec<_> = structural.fields.iter().map(|f| &f.ident).collect();

    let stateful_fields: Vec<_> = structural.fields.iter().filter(|f| !f.always_present).collect();
    let stateful_bounds: Vec<_> = stateful_fields.iter().map(|f| {
        let pascal = format_ident!("{}", to_pascal_case(&f.ident.to_string()));
        let ty = &f.ty;
        quote! {
            <<__State as #state_mod_name::State>::#pascal as ::structural_typing::Presence>::Output<#ty>: ::core::hash::Hash
        }
    }).collect();

    let (impl_gen, _, _) = impl_generics.split_for_impl();

    quote! {
        impl #impl_gen ::core::hash::Hash for #struct_ident #ty_generics<__State>
        where
            #where_clause
            __State: ::core::hash::Hash,
            #( #stateful_bounds, )*
        {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                #( self.#field_idents.hash(state); )*
            }
        }
    }
}

fn generate_serialize(structural: &StructuralStruct, state_mod_name: &syn::Ident) -> TokenStream {
    let struct_ident = &structural.ident;
    let struct_name = struct_ident.to_string();
    let (_impl_generics, ty_generics, where_clause) = structural.generics.split_for_impl();

    let generics = &structural.generics;
    let mut impl_generics = generics.clone();
    impl_generics.params.push(syn::parse_quote!(__State: #state_mod_name::State));

    let field_count = structural.fields.len();
    let field_names: Vec<_> = structural.fields.iter().map(|f| f.ident.to_string()).collect();
    let field_idents: Vec<_> = structural.fields.iter().map(|f| &f.ident).collect();

    let stateful_fields: Vec<_> = structural.fields.iter().filter(|f| !f.always_present).collect();
    let stateful_bounds: Vec<_> = stateful_fields.iter().map(|f| {
        let pascal = format_ident!("{}", to_pascal_case(&f.ident.to_string()));
        let ty = &f.ty;
        quote! {
            <<__State as #state_mod_name::State>::#pascal as ::structural_typing::Presence>::Output<#ty>: ::serde::Serialize
        }
    }).collect();

    let (impl_gen, _, _) = impl_generics.split_for_impl();

    quote! {
        impl #impl_gen ::serde::Serialize for #struct_ident #ty_generics<__State>
        where
            #where_clause
            __State: ::serde::Serialize,
            #( #stateful_bounds, )*
        {
            fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                use ::serde::ser::SerializeStruct;
                let mut state = serializer.serialize_struct(#struct_name, #field_count)?;
                #(
                    state.serialize_field(#field_names, &self.#field_idents)?;
                )*
                state.end()
            }
        }
    }
}

#[allow(clippy::too_many_lines)]
fn generate_deserialize(structural: &StructuralStruct, state_mod_name: &syn::Ident) -> TokenStream {
    // Deserialize is complex and state-dependent
    // For now, we'll generate a basic implementation that deserializes to the "all fields present" state
    let struct_ident = &structural.ident;
    let struct_name = struct_ident.to_string();
    let (_impl_generics, ty_generics, where_clause) = structural.generics.split_for_impl();

    let generics = &structural.generics;
    let impl_generics = generics.clone();

    // For deserialize, we need to specify which state we're deserializing to
    // Let's create impls for the "fully populated" state
    let stateful_fields: Vec<_> = structural.fields.iter().filter(|f| !f.always_present).collect();

    // Build the state type with all fields Present
    let state_type = if stateful_fields.is_empty() {
        quote! { #state_mod_name::Empty }
    } else {
        let mut state = quote! { #state_mod_name::Empty };
        for field in &stateful_fields {
            let pascal = format_ident!("{}", to_pascal_case(&field.ident.to_string()));
            let set_struct = format_ident!("Set{}", pascal);
            state = quote! { #state_mod_name::#set_struct<#state> };
        }
        state
    };

    let field_names: Vec<_> = structural.fields.iter().map(|f| f.ident.to_string()).collect();
    let field_idents: Vec<_> = structural.fields.iter().map(|f| &f.ident).collect();

    let deserialize_bounds: Vec<_> = structural.fields.iter().map(|f| {
        let ty = &f.ty;
        quote! { #ty: ::serde::Deserialize<'de> }
    }).collect();

    let (impl_gen, _, _) = impl_generics.split_for_impl();

    quote! {
        impl #impl_gen<'de> ::serde::Deserialize<'de> for #struct_ident #ty_generics<#state_type>
        where
            #where_clause
            #( #deserialize_bounds, )*
        {
            fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                #[derive(::serde::Deserialize)]
                #[serde(field_identifier, rename_all = "lowercase")]
                enum Field {
                    #( #field_idents, )*
                }

                struct StructVisitor;

                impl<'de> ::serde::de::Visitor<'de> for StructVisitor {
                    type Value = #struct_ident #ty_generics<#state_type>;

                    fn expecting(&self, formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        formatter.write_str(concat!("struct ", #struct_name))
                    }

                    fn visit_map<V>(self, mut map: V) -> ::core::result::Result<Self::Value, V::Error>
                    where
                        V: ::serde::de::MapAccess<'de>,
                    {
                        #(
                            let mut #field_idents = ::core::option::Option::None;
                        )*

                        while let ::core::option::Option::Some(key) = map.next_key()? {
                            match key {
                                #(
                                    Field::#field_idents => {
                                        if #field_idents.is_some() {
                                            return ::core::result::Result::Err(::serde::de::Error::duplicate_field(#field_names));
                                        }
                                        #field_idents = ::core::option::Option::Some(map.next_value()?);
                                    }
                                )*
                            }
                        }

                        #(
                            let #field_idents = #field_idents.ok_or_else(|| ::serde::de::Error::missing_field(#field_names))?;
                        )*

                        ::core::result::Result::Ok(#struct_ident {
                            #( #field_idents, )*
                            _phantom: ::core::marker::PhantomData,
                        })
                    }
                }

                deserializer.deserialize_struct(
                    #struct_name,
                    &[#( #field_names ),*],
                    StructVisitor
                )
            }
        }
    }
}
