use proc_macro2::TokenStream;
use quote::quote;
use syn::{GenericArgument, GenericParam, Ident, Path, PathArguments, Type};

use crate::codegen::generics_utils::{impl_generics_with_f, type_args_with_f};
use crate::parsing::{FieldInfo, StructInfo};

fn extract_generic_from_field_type(field_ty: &Type) -> Option<Ident> {
    if let Type::Path(type_path) = field_ty {
        if let Some(segment) = type_path.path.segments.last() {
            if let PathArguments::AngleBracketed(args) = &segment.arguments {
                for arg in &args.args {
                    if let GenericArgument::Type(Type::Path(nested_path)) = arg {
                        if let Some(ident) = nested_path.path.get_ident() {
                            return Some(ident.clone());
                        }
                    }
                }
            }
        }
    }
    None
}

fn build_return_type_args(
    info: &StructInfo,
    nested_module_path: &Path,
    nested_field: &Ident,
    nested_generic_param: Option<Ident>,
) -> TokenStream {
    use syn::GenericParam;

    let module_name = &info.module_name;
    let canonical_f = quote! { #module_name::Canonical<F> };

    // If no user generics, just return F
    if info.generics.params.is_empty() {
        return quote! { <#canonical_f> };
    }

    // Build the nested field type expression using select! macro
    let nested_field_type = if let Some(ref nested_param) = nested_generic_param {
        quote! {
            ::structural_typing::select!(
                #nested_module_path: #nested_field<V::Presence>, ..#nested_param
            )
        }
    } else {
        quote! {
            ::structural_typing::select!(
                #nested_module_path: #nested_field<V::Presence>
            )
        }
    };

    // Build type arguments, replacing nested generic param if present
    let mut lifetime_args = vec![];
    let mut other_args = vec![];

    for param in &info.generics.params {
        match param {
            GenericParam::Lifetime(lifetime_param) => {
                let lifetime = &lifetime_param.lifetime;
                lifetime_args.push(quote! { #lifetime });
            }
            GenericParam::Type(type_param) => {
                let ident = &type_param.ident;
                if nested_generic_param.as_ref() == Some(ident) {
                    other_args.push(nested_field_type.clone());
                } else {
                    other_args.push(quote! { #ident });
                }
            }
            GenericParam::Const(const_param) => {
                let ident = &const_param.ident;
                other_args.push(quote! { #ident });
            }
        }
    }

    quote! { <#(#lifetime_args,)* #canonical_f, #(#other_args),*> }
}

fn generate_nested_setter(
    info: &StructInfo,
    field: &FieldInfo,
    nested_module_path: &Path,
    nested_field: &Ident,
) -> syn::Result<TokenStream> {
    let struct_name = &info.name;
    let module_name = &info.module_name;
    let field_name = &field.name;

    let setter_name = Ident::new(
        &format!("{}_{}", field_name, nested_field),
        field_name.span(),
    );

    let nested_generic_param = extract_generic_from_field_type(&field.ty);

    let return_type_args =
        build_return_type_args(info, nested_module_path, nested_field, nested_generic_param);

    // For the InferPresence bound, we need to reference the nested field's type.
    // The challenge is that type_of::field may have generic parameters (like lifetimes).
    // Rather than trying to figure out which specific generics to pass, we can use
    // the fact that the nested struct's setter method already has the right signature.
    // We just need to make sure our V parameter is compatible with whatever that setter expects.
    //
    // Instead of using type_of, we can directly infer from V being used in the setter call.
    // However, we still need some bound on V. The simplest approach is to use the parent's
    // lifetime parameters since those are the most common case for generics.

    // Extract only lifetimes from parent struct (most common case for type_of generics)
    let lifetime_params: Vec<_> = info
        .generics
        .params
        .iter()
        .filter_map(|param| {
            if let GenericParam::Lifetime(lp) = param {
                Some(&lp.lifetime)
            } else {
                None
            }
        })
        .collect();

    let type_ref = if lifetime_params.is_empty() {
        quote! { #nested_module_path::type_of::#nested_field }
    } else {
        quote! { #nested_module_path::type_of::#nested_field<#(#lifetime_params),*> }
    };

    Ok(quote! {
        #[allow(non_snake_case)]
        #[must_use]
        pub fn #setter_name<V: ::structural_typing::presence::InferPresence<#type_ref>>(
            self,
            #nested_field: V
        ) -> #struct_name #return_type_args
        where
            F: #module_name::Fields<#field_name = ::structural_typing::presence::Present>
        {
            let (nested_only, rest) = self.extract::<::structural_typing::select!(#module_name: #field_name)>();
            rest.#field_name(nested_only.#field_name.#nested_field(#nested_field))
        }
    })
}

pub fn generate(info: &StructInfo) -> TokenStream {
    let struct_name = &info.name;
    let module_name = &info.module_name;

    let mut methods = Vec::new();
    let mut errors = Vec::new();

    for field in &info.fields {
        if let Some(nested_fields_info) = &field.nested_fields {
            for nested_field in &nested_fields_info.field_names {
                match generate_nested_setter(
                    info,
                    field,
                    &nested_fields_info.module_path,
                    nested_field,
                ) {
                    Ok(method) => methods.push(method),
                    Err(e) => errors.push(e),
                }
            }
        }
    }

    if !errors.is_empty() {
        let compile_errors = errors.iter().map(|e| e.to_compile_error());
        return quote! {
            #(#compile_errors)*
        };
    }

    if methods.is_empty() {
        return quote! {};
    }

    let (impl_generics, user_type_args) = impl_generics_with_f(&info.generics, module_name);
    let (impl_generics, _, where_clause) = impl_generics.split_for_impl();
    let impl_type_args = type_args_with_f(&info.generics, &user_type_args, quote! { F });

    quote! {
        impl #impl_generics #struct_name #impl_type_args #where_clause {
            #(#methods)*
        }
    }
}
