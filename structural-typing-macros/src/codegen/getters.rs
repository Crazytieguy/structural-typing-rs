use proc_macro2::TokenStream;
use quote::quote;

use crate::parsing::StructInfo;

pub fn generate(info: &StructInfo) -> TokenStream {
    let struct_name = &info.name;
    let module_name = &info.module_name;

    let methods = info.fields.iter().map(|field| {
        let field_name = &field.name;
        let field_ty = &field.ty;
        let get_method = quote::format_ident!("get_{}", field_name);
        let get_mut_method = quote::format_ident!("get_{}_mut", field_name);

        quote! {
            /// Returns `Some(&value)` if field is Present or Optional with Some; `None` if Absent or Optional with None.
            pub fn #get_method(&self) -> Option<&#field_ty> {
                ::structural_typing::access::Access::<#field_ty>::get(&self.#field_name)
            }

            /// Returns `Some(&mut value)` if field is Present or Optional with Some; `None` if Absent or Optional with None.
            pub fn #get_mut_method(&mut self) -> Option<&mut #field_ty> {
                ::structural_typing::access::Access::<#field_ty>::get_mut(&mut self.#field_name)
            }
        }
    });

    quote! {
        impl<F: #module_name::Fields> #struct_name<F> {
            #(#methods)*
        }
    }
}
