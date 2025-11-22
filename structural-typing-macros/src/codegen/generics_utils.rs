use proc_macro2::TokenStream;
use quote::quote;
use syn::{GenericParam, Generics, Ident};

/// Extracts just the identifiers from generic parameters for use in type arguments
pub fn extract_type_param_idents(
    params: &syn::punctuated::Punctuated<GenericParam, syn::token::Comma>,
) -> Vec<TokenStream> {
    params
        .iter()
        .map(|param| match param {
            GenericParam::Type(type_param) => {
                let ident = &type_param.ident;
                quote! { #ident }
            }
            GenericParam::Lifetime(lifetime_param) => {
                let lifetime = &lifetime_param.lifetime;
                quote! { #lifetime }
            }
            GenericParam::Const(const_param) => {
                let ident = &const_param.ident;
                quote! { #ident }
            }
        })
        .collect()
}

/// Creates impl generics by adding F parameter after lifetimes but before other params
/// Also returns user type args for convenience
pub fn impl_generics_with_f(
    user_generics: &Generics,
    module_name: &Ident,
) -> (Generics, Vec<TokenStream>) {
    let mut impl_generics = user_generics.clone();
    let f_param: GenericParam = syn::parse_quote!(F: #module_name::Fields);

    // Find insertion point: after all lifetimes
    let mut insertion_point = impl_generics.params.len();
    for (i, param) in impl_generics.params.iter().enumerate() {
        if !matches!(param, GenericParam::Lifetime(_)) {
            insertion_point = i;
            break;
        }
    }

    impl_generics.params.insert(insertion_point, f_param);

    let user_type_args = extract_type_param_idents(&user_generics.params);

    (impl_generics, user_type_args)
}

/// Generates type arguments with proper lifetime ordering
/// Returns <lifetimes, f_value, other_params> or <f_value> if no user generics
pub fn type_args_with_f(
    generics: &Generics,
    user_type_args: &[TokenStream],
    f_value: TokenStream,
) -> TokenStream {
    if generics.params.is_empty() {
        quote! { <#f_value> }
    } else {
        let (lifetime_args, other_args): (Vec<_>, Vec<_>) = user_type_args
            .iter()
            .zip(generics.params.iter())
            .partition(|(_, param)| matches!(param, GenericParam::Lifetime(_)));
        let lifetime_tokens: Vec<_> = lifetime_args.into_iter().map(|(tok, _)| tok).collect();
        let other_tokens: Vec<_> = other_args.into_iter().map(|(tok, _)| tok).collect();
        quote! { <#(#lifetime_tokens,)* #f_value, #(#other_tokens),*> }
    }
}

/// Generates remainder type arguments with proper lifetime ordering
/// Returns <lifetimes, F, F2, other_params> or <F, F2> if no user generics
pub fn remainder_type_args(generics: &Generics, user_type_args: &[TokenStream]) -> TokenStream {
    if generics.params.is_empty() {
        quote! { <F, F2> }
    } else {
        let (lifetime_args, other_args): (Vec<_>, Vec<_>) = user_type_args
            .iter()
            .zip(generics.params.iter())
            .partition(|(_, param)| matches!(param, GenericParam::Lifetime(_)));
        let lifetime_tokens: Vec<_> = lifetime_args.into_iter().map(|(tok, _)| tok).collect();
        let other_tokens: Vec<_> = other_args.into_iter().map(|(tok, _)| tok).collect();
        quote! { <#(#lifetime_tokens,)* F, F2, #(#other_tokens),*> }
    }
}

/// Returns user generic params that don't have default values (not trailing defaults)
pub fn non_defaulted_params(generics: &Generics) -> Vec<GenericParam> {
    let params: Vec<_> = generics.params.iter().cloned().collect();

    let mut first_default_idx = params.len();
    for (i, param) in params.iter().enumerate().rev() {
        if let GenericParam::Type(type_param) = param {
            if type_param.default.is_some() {
                first_default_idx = i;
            } else {
                break;
            }
        }
        // Lifetimes and const generics don't have defaults, so we continue
    }

    params.into_iter().take(first_default_idx).collect()
}
