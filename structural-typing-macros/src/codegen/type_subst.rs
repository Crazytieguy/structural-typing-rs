use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use std::collections::{HashMap, HashSet};
use syn::{GenericParam, Generics, Type};

pub struct SingleFieldGenericInfo {
    pub original_ident: Ident,
    pub new_ident: Ident,
    pub bounds: Vec<TokenStream>,
}

pub fn extract_single_field_generics_from_type(
    ty: &Type,
    single_field_generics: &HashSet<String>,
    generics: &Generics,
) -> Vec<SingleFieldGenericInfo> {
    let mut result = Vec::new();
    let mut visited = HashSet::new();

    extract_generics_recursive(ty, single_field_generics, &mut visited);

    // Sort to ensure deterministic order
    let mut sorted_generics: Vec<_> = visited.into_iter().collect();
    sorted_generics.sort();

    for generic_name in sorted_generics {
        let new_ident = format_ident!("New{}", generic_name);

        let mut bounds = Vec::new();

        for param in &generics.params {
            if let GenericParam::Type(type_param) = param {
                if type_param.ident == generic_name {
                    bounds.extend(type_param.bounds.iter().map(|bound| quote! { #bound }));
                    break;
                }
            }
        }

        if let Some(where_clause) = &generics.where_clause {
            for predicate in &where_clause.predicates {
                if let syn::WherePredicate::Type(type_predicate) = predicate {
                    if let Type::Path(type_path) = &type_predicate.bounded_ty {
                        if let Some(ident) = type_path.path.get_ident() {
                            if ident == &generic_name {
                                bounds.extend(
                                    type_predicate.bounds.iter().map(|bound| quote! { #bound }),
                                );
                            }
                        }
                    }
                }
            }
        }

        result.push(SingleFieldGenericInfo {
            original_ident: Ident::new(&generic_name, Span::call_site()),
            new_ident,
            bounds,
        });
    }

    result
}

fn extract_generics_recursive(
    ty: &Type,
    single_field_generics: &HashSet<String>,
    visited: &mut HashSet<String>,
) {
    match ty {
        Type::Path(type_path) => {
            if let Some(ident) = type_path.path.get_ident() {
                let ident_str = ident.to_string();
                if single_field_generics.contains(&ident_str) {
                    visited.insert(ident_str);
                }
            }

            for segment in &type_path.path.segments {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    for arg in &args.args {
                        if let syn::GenericArgument::Type(ty) = arg {
                            extract_generics_recursive(ty, single_field_generics, visited);
                        }
                    }
                }
            }
        }
        Type::Reference(type_ref) => {
            extract_generics_recursive(&type_ref.elem, single_field_generics, visited);
        }
        Type::Ptr(type_ptr) => {
            extract_generics_recursive(&type_ptr.elem, single_field_generics, visited);
        }
        Type::Array(type_array) => {
            extract_generics_recursive(&type_array.elem, single_field_generics, visited);
        }
        Type::Slice(type_slice) => {
            extract_generics_recursive(&type_slice.elem, single_field_generics, visited);
        }
        Type::Tuple(type_tuple) => {
            for elem in &type_tuple.elems {
                extract_generics_recursive(elem, single_field_generics, visited);
            }
        }
        Type::Paren(type_paren) => {
            extract_generics_recursive(&type_paren.elem, single_field_generics, visited);
        }
        _ => {}
    }
}

pub fn substitute_type(ty: &Type, subst_map: &HashMap<String, Ident>) -> Type {
    match ty {
        Type::Path(type_path) => {
            let mut new_path = type_path.clone();

            if let Some(ident) = type_path.path.get_ident() {
                let ident_str = ident.to_string();
                if let Some(new_ident) = subst_map.get(&ident_str) {
                    new_path.path.segments.clear();
                    new_path.path.segments.push(syn::PathSegment {
                        ident: new_ident.clone(),
                        arguments: syn::PathArguments::None,
                    });
                    return Type::Path(new_path);
                }
            }

            for segment in &mut new_path.path.segments {
                if let syn::PathArguments::AngleBracketed(args) = &mut segment.arguments {
                    for arg in &mut args.args {
                        if let syn::GenericArgument::Type(ty) = arg {
                            *ty = substitute_type(ty, subst_map);
                        }
                    }
                }
            }

            Type::Path(new_path)
        }
        Type::Reference(type_ref) => {
            let mut new_ref = type_ref.clone();
            *new_ref.elem = substitute_type(&type_ref.elem, subst_map);
            Type::Reference(new_ref)
        }
        Type::Ptr(type_ptr) => {
            let mut new_ptr = type_ptr.clone();
            *new_ptr.elem = substitute_type(&type_ptr.elem, subst_map);
            Type::Ptr(new_ptr)
        }
        Type::Array(type_array) => {
            let mut new_array = type_array.clone();
            *new_array.elem = substitute_type(&type_array.elem, subst_map);
            Type::Array(new_array)
        }
        Type::Slice(type_slice) => {
            let mut new_slice = type_slice.clone();
            *new_slice.elem = substitute_type(&type_slice.elem, subst_map);
            Type::Slice(new_slice)
        }
        Type::Tuple(type_tuple) => {
            let mut new_tuple = type_tuple.clone();
            new_tuple.elems = type_tuple
                .elems
                .iter()
                .map(|elem| substitute_type(elem, subst_map))
                .collect();
            Type::Tuple(new_tuple)
        }
        Type::Paren(type_paren) => {
            let mut new_paren = type_paren.clone();
            *new_paren.elem = substitute_type(&type_paren.elem, subst_map);
            Type::Paren(new_paren)
        }
        _ => ty.clone(),
    }
}

pub fn substitute_type_args(
    user_type_args: &[TokenStream],
    generics: &Generics,
    subst_map: &HashMap<String, Ident>,
) -> Vec<TokenStream> {
    user_type_args
        .iter()
        .zip(generics.params.iter())
        .map(|(arg, param)| {
            if let GenericParam::Type(type_param) = param {
                let param_name = type_param.ident.to_string();
                if let Some(new_ident) = subst_map.get(&param_name) {
                    return quote! { #new_ident };
                }
            }
            arg.clone()
        })
        .collect()
}
