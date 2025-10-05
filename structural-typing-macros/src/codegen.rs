use crate::parsing::{parse_structural_struct, StructuralStruct};
use crate::derive::generate_derive_impls;
use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use syn::{DeriveInput, Result};

pub fn generate_structural(input: DeriveInput) -> Result<TokenStream> {
    let structural_struct = parse_structural_struct(input)?;

    let struct_def = generate_struct(&structural_struct);
    let state_mod = generate_state_mod(&structural_struct);
    let impl_empty = generate_empty_impl(&structural_struct);
    let impl_setters = generate_setters(&structural_struct);
    let merge_state_def = generate_merge_state(&structural_struct);
    let impl_merge = generate_merge(&structural_struct);
    let impl_require = generate_require(&structural_struct);
    let derive_impls = generate_derive_impls(&structural_struct);

    Ok(quote! {
        #state_mod
        #struct_def
        #merge_state_def
        #impl_empty
        #impl_setters
        #impl_merge
        #impl_require
        #derive_impls
    })
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

fn generate_struct(structural: &StructuralStruct) -> TokenStream {
    let vis = &structural.vis;
    let ident = &structural.ident;
    let (impl_generics, _ty_generics, where_clause) = structural.generics.split_for_impl();

    // Filter out derive attributes - we implement them manually
    let non_derive_attrs = structural.attrs.iter().filter(|attr| {
        !attr.path().is_ident("derive")
    });

    let state_mod_name = format_ident!("{}_state", ident.to_string().to_lowercase());

    let always_fields = structural.fields.iter().filter(|f| f.always_present);
    let stateful_fields = structural.fields.iter().filter(|f| !f.always_present);

    let always_idents = always_fields.clone().map(|f| &f.ident);
    let always_types = always_fields.clone().map(|f| &f.ty);
    let always_vis = always_fields.map(|f| &f.vis);

    let stateful_idents = stateful_fields.clone().map(|f| &f.ident);
    let stateful_types = stateful_fields.clone().map(|f| &f.ty);
    let stateful_vis = stateful_fields.clone().map(|f| &f.vis);
    let stateful_pascal = stateful_fields.map(|f| {
        format_ident!("{}", to_pascal_case(&f.ident.to_string()))
    });

    quote! {
        #( #non_derive_attrs )*
        #vis struct #ident #impl_generics<__State: #state_mod_name::State = #state_mod_name::Empty>
        #where_clause
        {
            #(
                #always_vis #always_idents: #always_types,
            )*
            #(
                #stateful_vis #stateful_idents: <<__State as #state_mod_name::State>::#stateful_pascal as ::structural_typing::Presence>::Output<#stateful_types>,
            )*
            _phantom: ::structural_typing::__private::PhantomData<__State>,
        }
    }
}

fn generate_state_mod(structural: &StructuralStruct) -> TokenStream {
    let struct_ident = &structural.ident;
    let state_mod_name = format_ident!("{}_state", struct_ident.to_string().to_lowercase());

    let stateful_fields = structural.fields.iter()
        .filter(|f| !f.always_present)
        .collect::<Vec<_>>();

    let field_idents = stateful_fields.iter().map(|f| &f.ident).collect::<Vec<_>>();
    let field_pascal_idents = field_idents.iter().map(|ident| {
        format_ident!("{}", to_pascal_case(&ident.to_string()))
    }).collect::<Vec<_>>();

    let assoc_type_decls = field_pascal_idents.iter().map(|ident| {
        quote! {
            type #ident: ::structural_typing::Presence;
        }
    });

    let empty_impls = field_pascal_idents.iter().map(|ident| {
        quote! {
            type #ident = ::structural_typing::Absent;
        }
    });

    let set_struct_defs = field_pascal_idents.iter().map(|field_pascal| {
        let set_struct_name = format_ident!("Set{}", field_pascal);
        quote! {
            #[derive(
                ::structural_typing::__private::Clone,
                ::structural_typing::__private::fmt::Debug,
                ::core::cmp::PartialEq,
                ::core::cmp::Eq,
                ::core::hash::Hash
            )]
            pub struct #set_struct_name<S: State>(::structural_typing::__private::PhantomData<fn() -> S>);
        }
    });

    let set_struct_impls = field_pascal_idents.iter().enumerate().map(|(idx, target_field)| {
        let set_struct_name = format_ident!("Set{}", target_field);

        let field_type_impls = field_pascal_idents.iter().enumerate().map(|(field_idx, field_pascal)| {
            if field_idx == idx {
                quote! {
                    type #field_pascal = ::structural_typing::Present;
                }
            } else {
                quote! {
                    type #field_pascal = S::#field_pascal;
                }
            }
        });

        quote! {
            impl<S: State> State for #set_struct_name<S> {
                #( #field_type_impls )*
            }
        }
    });

    // Generate serde impls for state types if serde derives are requested
    let has_serde = structural.attrs.iter().any(|attr| {
        if attr.path().is_ident("derive") {
            let mut found_serde = false;
            let _ = attr.parse_nested_meta(|meta| {
                if let Some(ident) = meta.path.get_ident()
                    && (ident == "Serialize" || ident == "Deserialize")
                {
                    found_serde = true;
                }
                Ok(())
            });
            found_serde
        } else {
            false
        }
    });

    let serde_impls = if has_serde {
        let empty_serde = quote! {
            impl ::serde::Serialize for Empty {
                fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
                where S: ::serde::Serializer {
                    serializer.serialize_unit()
                }
            }

            impl<'de> ::serde::Deserialize<'de> for Empty {
                fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
                where D: ::serde::Deserializer<'de> {
                    struct EmptyVisitor;
                    impl<'de> ::serde::de::Visitor<'de> for EmptyVisitor {
                        type Value = Empty;
                        fn expecting(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            f.write_str("unit")
                        }
                        fn visit_unit<E>(self) -> ::core::result::Result<Empty, E> {
                            ::core::result::Result::Ok(Empty(()))
                        }
                    }
                    deserializer.deserialize_unit(EmptyVisitor)
                }
            }
        };

        let set_serde = field_pascal_idents.iter().map(|field_pascal| {
            let set_struct_name = format_ident!("Set{}", field_pascal);
            quote! {
                impl<S: State> ::serde::Serialize for #set_struct_name<S> {
                    fn serialize<Ser>(&self, serializer: Ser) -> ::core::result::Result<Ser::Ok, Ser::Error>
                    where Ser: ::serde::Serializer {
                        serializer.serialize_unit()
                    }
                }

                impl<'de, S: State> ::serde::Deserialize<'de> for #set_struct_name<S> {
                    fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
                    where D: ::serde::Deserializer<'de> {
                        struct SetVisitor<S>(::core::marker::PhantomData<S>);
                        impl<'de, S: State> ::serde::de::Visitor<'de> for SetVisitor<S> {
                            type Value = #set_struct_name<S>;
                            fn expecting(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                                f.write_str("unit")
                            }
                            fn visit_unit<E>(self) -> ::core::result::Result<#set_struct_name<S>, E> {
                                ::core::result::Result::Ok(#set_struct_name(::core::marker::PhantomData))
                            }
                        }
                        deserializer.deserialize_unit(SetVisitor(::core::marker::PhantomData))
                    }
                }
            }
        });

        quote! {
            #empty_serde
            #( #set_serde )*
        }
    } else {
        quote! {}
    };

    quote! {
        #[allow(non_camel_case_types)]
        pub mod #state_mod_name {
            pub trait State: Sized {
                #( #assoc_type_decls )*
            }

            #[derive(
                ::structural_typing::__private::Clone,
                ::structural_typing::__private::fmt::Debug,
                ::core::cmp::PartialEq,
                ::core::cmp::Eq,
                ::core::hash::Hash,
                ::core::default::Default
            )]
            pub struct Empty(());

            impl State for Empty {
                #( #empty_impls )*
            }

            #( #set_struct_defs )*
            #( #set_struct_impls )*

            #serde_impls
        }
    }
}

fn generate_empty_impl(structural: &StructuralStruct) -> TokenStream {
    let ident = &structural.ident;
    let (impl_generics, ty_generics, where_clause) = structural.generics.split_for_impl();

    let always_fields = structural.fields.iter().filter(|f| f.always_present);
    let stateful_fields = structural.fields.iter().filter(|f| !f.always_present);

    let always_params = always_fields.clone().map(|f| {
        let field_ident = &f.ident;
        let field_type = &f.ty;
        quote! { #field_ident: #field_type }
    });

    let always_inits = always_fields.map(|f| {
        let field_ident = &f.ident;
        quote! { #field_ident }
    });

    let stateful_inits = stateful_fields.map(|f| {
        let field_ident = &f.ident;
        quote! {
            #field_ident: ::structural_typing::__private::PhantomData
        }
    });

    quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            pub fn empty(#( #always_params ),*) -> Self {
                Self {
                    #( #always_inits, )*
                    #( #stateful_inits, )*
                    _phantom: ::structural_typing::__private::PhantomData,
                }
            }
        }
    }
}

fn generate_setters(structural: &StructuralStruct) -> TokenStream {
    let struct_ident = &structural.ident;
    let state_mod_name = format_ident!("{}_state", struct_ident.to_string().to_lowercase());
    let (impl_generics, ty_generics, where_clause) = structural.generics.split_for_impl();

    let setter_impls = structural.fields.iter().map(|field| {
        let field_ident = &field.ident;
        let field_type = &field.ty;

        let other_field_idents = structural.fields.iter()
            .filter(|f| f.ident != *field_ident)
            .map(|f| &f.ident);

        if field.always_present {
            // For always-present fields, don't change the state
            quote! {
                pub fn #field_ident(self, value: #field_type) -> #struct_ident #ty_generics<__State> {
                    #struct_ident {
                        #field_ident: value,
                        #( #other_field_idents: self.#other_field_idents, )*
                        _phantom: ::structural_typing::__private::PhantomData,
                    }
                }
            }
        } else {
            // For stateful fields, transition to new state
            let field_pascal = format_ident!("{}", to_pascal_case(&field_ident.to_string()));
            let set_struct_name = format_ident!("Set{}", field_pascal);
            quote! {
                pub fn #field_ident(self, value: #field_type) -> #struct_ident #ty_generics<#state_mod_name::#set_struct_name<__State>> {
                    #struct_ident {
                        #field_ident: value,
                        #( #other_field_idents: self.#other_field_idents, )*
                        _phantom: ::structural_typing::__private::PhantomData,
                    }
                }
            }
        }
    });

    quote! {
        impl #impl_generics<__State: #state_mod_name::State> #struct_ident #ty_generics<__State> #where_clause {
            #( #setter_impls )*
        }
    }
}

fn generate_merge_state(structural: &StructuralStruct) -> TokenStream {
    let struct_ident = &structural.ident;
    let state_mod_name = format_ident!("{}_state", struct_ident.to_string().to_lowercase());

    // Only stateful fields participate in state merging
    let field_pascal_idents = structural.fields.iter()
        .filter(|f| !f.always_present)
        .map(|f| format_ident!("{}", to_pascal_case(&f.ident.to_string())));

    quote! {
        struct __MergeState<__S1, __S2>(::structural_typing::__private::PhantomData<(__S1, __S2)>);

        impl<__S1: #state_mod_name::State, __S2: #state_mod_name::State> #state_mod_name::State for __MergeState<__S1, __S2> {
            #(
                type #field_pascal_idents = <<__S2 as #state_mod_name::State>::#field_pascal_idents as ::structural_typing::Presence>::Or<<__S1 as #state_mod_name::State>::#field_pascal_idents>;
            )*
        }
    }
}

fn generate_merge(structural: &StructuralStruct) -> TokenStream {
    let struct_ident = &structural.ident;
    let state_mod_name = format_ident!("{}_state", struct_ident.to_string().to_lowercase());
    let (impl_generics, ty_generics, where_clause) = structural.generics.split_for_impl();

    let merge_field_assigns = structural.fields.iter().map(|field| {
        let ident = &field.ident;
        if field.always_present {
            // For always-present fields, just take from other
            quote! {
                #ident: other.#ident
            }
        } else {
            // For stateful fields, use Presence::or
            let pascal = format_ident!("{}", to_pascal_case(&ident.to_string()));
            quote! {
                #ident: <<__State2 as #state_mod_name::State>::#pascal as ::structural_typing::Presence>::or(
                    other.#ident,
                    self.#ident
                )
            }
        }
    });

    quote! {
        impl #impl_generics<__State: #state_mod_name::State> #struct_ident #ty_generics<__State> #where_clause {
            pub fn merge<__State2: #state_mod_name::State>(
                self,
                other: #struct_ident #ty_generics<__State2>
            ) -> #struct_ident #ty_generics<__MergeState<__State, __State2>>
            {
                #struct_ident {
                    #( #merge_field_assigns, )*
                    _phantom: ::structural_typing::__private::PhantomData,
                }
            }
        }
    }
}

fn generate_require(structural: &StructuralStruct) -> TokenStream {
    let struct_ident = &structural.ident;
    let state_mod_name = format_ident!("{}_state", struct_ident.to_string().to_lowercase());
    let (impl_generics, ty_generics, where_clause) = structural.generics.split_for_impl();

    // Only generate require methods for stateful fields
    let require_methods = structural.fields.iter()
        .filter(|field| !field.always_present)
        .map(|field| {
            let field_ident = &field.ident;
            let method_name = format_ident!("require_{}", field_ident);
            let field_pascal = format_ident!("{}", to_pascal_case(&field_ident.to_string()));
            let set_struct_name = format_ident!("Set{}", field_pascal);

            let other_field_idents = structural.fields.iter()
                .filter(|f| f.ident != *field_ident)
                .map(|f| &f.ident);

            quote! {
                pub fn #method_name(self) -> ::structural_typing::__private::Option<#struct_ident #ty_generics<#state_mod_name::#set_struct_name<__State>>> {
                    use ::structural_typing::access::Access;
                    match self.#field_ident.remove() {
                        ::structural_typing::__private::Option::Some(value) => {
                            ::structural_typing::__private::Option::Some(#struct_ident {
                                #field_ident: value,
                                #( #other_field_idents: self.#other_field_idents, )*
                                _phantom: ::structural_typing::__private::PhantomData,
                            })
                        }
                        ::structural_typing::__private::Option::None => ::structural_typing::__private::Option::None,
                    }
                }
            }
        });

    quote! {
        impl #impl_generics<__State: #state_mod_name::State> #struct_ident #ty_generics<__State> #where_clause {
            #( #require_methods )*
        }
    }
}
