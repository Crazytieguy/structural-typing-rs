use structural_typing::{structural, Access, Present};

#[structural]
struct Data {
    required: String,
    optional: i32,
}

impl<S> Data<S>
where
    S: data_state::State<Required = Present>,
{
    fn describe(&self) -> String {
        // Try using Access::get() without using the trait
        match self.optional.get() {
            Some(val) => format!("{}: {}", self.required, val),
            None => self.required.clone(),
        }
    }
}

#[test]
fn test_access_with_explicit_import() {
    let data = Data::empty().required("test".into()).optional(42);
    assert_eq!(data.describe(), "test: 42");

    let data_without_optional = Data::empty().required("minimal".into());
    assert_eq!(data_without_optional.describe(), "minimal");
}
