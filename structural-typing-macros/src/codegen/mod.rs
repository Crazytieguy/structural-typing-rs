mod builders;
mod constructors;
mod fields_module;
mod generics_utils;
mod getters;
mod merge;
mod extract;
mod serde_deserialize;
mod struct_def;

use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::parsing;

pub fn generate(input: DeriveInput) -> syn::Result<TokenStream> {
    let info = parsing::parse_struct(input)?;

    let (serde_helper, serde_try_from) = serde_deserialize::generate(&info)?;
    let fields_mod = fields_module::generate(&info, serde_helper);
    let struct_def = struct_def::generate(&info);
    let constructors = constructors::generate(&info);
    let builders = builders::generate(&info);
    let getters = getters::generate(&info);
    let merge = merge::generate(&info);
    let extract = extract::generate(&info);

    Ok(quote! {
        #fields_mod

        #serde_try_from

        #struct_def

        #constructors

        #builders

        #getters

        #merge

        #extract
    })
}
