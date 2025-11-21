use std::collections::{HashMap, HashSet};
use syn::{GenericParam, Generics, Type};

pub struct GenericUsageAnalyzer {
    user_generics: HashSet<String>,
    current_field: Option<String>,
    usage_map: HashMap<String, Vec<String>>,
}

impl GenericUsageAnalyzer {
    fn new(generics: &Generics) -> Self {
        let user_generics = generics
            .params
            .iter()
            .filter_map(|param| match param {
                GenericParam::Type(type_param) => Some(type_param.ident.to_string()),
                _ => None,
            })
            .collect();

        Self {
            user_generics,
            current_field: None,
            usage_map: HashMap::new(),
        }
    }

    fn analyze_field(&mut self, field_name: &str, field_type: &Type) {
        self.current_field = Some(field_name.to_string());
        self.visit_type(field_type);
        self.current_field = None;
    }

    fn visit_type(&mut self, ty: &Type) {
        match ty {
            Type::Path(type_path) => {
                if let Some(ident) = type_path.path.get_ident() {
                    let ident_str = ident.to_string();
                    if self.user_generics.contains(&ident_str) {
                        if let Some(field_name) = &self.current_field {
                            self.usage_map
                                .entry(ident_str)
                                .or_default()
                                .push(field_name.clone());
                        }
                    }
                }

                for segment in &type_path.path.segments {
                    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                        for arg in &args.args {
                            if let syn::GenericArgument::Type(ty) = arg {
                                self.visit_type(ty);
                            }
                        }
                    }
                }
            }
            Type::Reference(type_ref) => {
                self.visit_type(&type_ref.elem);
            }
            Type::Ptr(type_ptr) => {
                self.visit_type(&type_ptr.elem);
            }
            Type::Array(type_array) => {
                self.visit_type(&type_array.elem);
            }
            Type::Slice(type_slice) => {
                self.visit_type(&type_slice.elem);
            }
            Type::Tuple(type_tuple) => {
                for elem in &type_tuple.elems {
                    self.visit_type(elem);
                }
            }
            Type::Paren(type_paren) => {
                self.visit_type(&type_paren.elem);
            }
            _ => {}
        }
    }
}

pub fn analyze_generic_usage(
    generics: &Generics,
    fields: &[(String, Type)],
) -> HashMap<String, Vec<String>> {
    let mut analyzer = GenericUsageAnalyzer::new(generics);

    for (field_name, field_type) in fields {
        analyzer.analyze_field(field_name, field_type);
    }

    analyzer.usage_map
}

pub fn identify_single_field_generics(usage_map: &HashMap<String, Vec<String>>) -> HashSet<String> {
    usage_map
        .iter()
        .filter_map(|(generic, fields)| {
            if fields.len() == 1 {
                Some(generic.clone())
            } else {
                None
            }
        })
        .collect()
}
