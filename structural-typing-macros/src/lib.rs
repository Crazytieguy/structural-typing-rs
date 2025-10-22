use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod codegen;
mod parsing;

#[proc_macro_attribute]
pub fn structural(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    match codegen::generate(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
