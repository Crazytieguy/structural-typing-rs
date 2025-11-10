use core::marker::PhantomData;

use structural_typing::{presence::Present, select, structural};

#[structural]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct TestStruct {
    name: String,
    email: String,
    id: u64,
}

#[structural]
#[derive(Debug)]
struct RawIdConfig {
    r#type: String,
    r#match: bool,
    normal: u32,
}

#[structural]
struct NoClone {
    value: String,
    id: u64,
}

#[structural]
struct SingleField {
    data: String,
}

#[test]
fn select_basic() {
    type NameOnly = select!(test_struct: name);
    let val: TestStruct<NameOnly> = TestStruct::empty().name("Alice".to_owned());
    assert_eq!(val.name, "Alice");
}

#[test]
fn select_multiple() {
    type NameAndEmail = select!(test_struct: name, email);
    let val: TestStruct<NameAndEmail> = TestStruct::empty()
        .name("Bob".to_owned())
        .email("bob@test.com".to_owned());
    assert_eq!(val.name, "Bob");
    assert_eq!(val.email, "bob@test.com");
}

#[test]
fn select_optional() {
    type NameAndMaybeEmail = select!(test_struct: name, ?email);
    let val: TestStruct<NameAndMaybeEmail> = TestStruct::empty()
        .name("Charlie".to_owned())
        .email(Some("charlie@test.com".to_owned()));
    assert_eq!(val.name, "Charlie");
    assert_eq!(val.email, Some("charlie@test.com".to_string()));
}

#[test]
fn select_empty_all_absent() {
    let val = TestStruct::empty();
    assert!(val.get_name().is_none());
    assert!(val.get_email().is_none());
    assert!(val.get_id().is_none());
}

#[test]
fn select_all_optional() {
    type AllOptional = select!(test_struct: ?all);
    let val: TestStruct<AllOptional> = TestStruct::empty()
        .name(Some("Alice".to_owned()))
        .email(None)
        .id(Some(123));
    assert_eq!(val.name, Some("Alice".to_owned()));
    assert_eq!(val.email, None);
    assert_eq!(val.id, Some(123));
}

#[test]
fn modify_add_fields() {
    type NameAndEmail = select!(test_struct: name, email);
    let val: TestStruct<NameAndEmail> = TestStruct::empty()
        .name("Dave".to_owned())
        .email("dave@test.com".to_owned());
    assert_eq!(val.name, "Dave");
    assert_eq!(val.email, "dave@test.com");
}

#[test]
fn modify_remove_fields() {
    type OnlyName = select!(test_struct: name);
    let val: TestStruct<OnlyName> = TestStruct::empty().name("Eve".to_owned());
    assert_eq!(val.name, "Eve");
    assert!(val.get_email().is_none());
    assert!(val.get_id().is_none());
}

#[test]
fn modify_make_optional() {
    type NameAndMaybeEmail = select!(test_struct: name, ?email);
    let val: TestStruct<NameAndMaybeEmail> =
        TestStruct::empty().name("Frank".to_owned()).email(None);
    assert_eq!(val.name, "Frank");
    assert_eq!(val.email, None);
}

#[test]
#[cfg(feature = "serde")]
fn serde_with_select() {
    type NameAndEmail = select!(test_struct: name, email);
    let val: TestStruct<NameAndEmail> = TestStruct::empty()
        .name("Grace".to_owned())
        .email("grace@test.com".to_owned());

    let json = serde_json::to_string(&val).unwrap();
    assert!(json.contains(r#""name":"Grace"#));
    assert!(json.contains(r#""email":"grace@test.com"#));
    assert!(!json.contains("id"), "Absent field should not appear");

    let full_json = r#"{"name":"Grace","email":"grace@test.com","id":0}"#;
    let deserialized: TestStruct = serde_json::from_str(full_json).unwrap();
    assert_eq!(deserialized.name, "Grace");
    assert_eq!(deserialized.email, "grace@test.com");
}

#[test]
#[cfg(feature = "serde")]
fn serde_deserialize_with_missing_optional_fields() {
    type NameEmailAndOptionalId = select!(test_struct: name, email, ?id);
    let json_without_id = r#"{"name":"Bob","email":"bob@test.com"}"#;
    let deserialized: TestStruct<NameEmailAndOptionalId> =
        serde_json::from_str(json_without_id).unwrap();
    assert_eq!(deserialized.name, "Bob");
    assert_eq!(deserialized.email, "bob@test.com");
    assert_eq!(deserialized.id, None);
}

#[test]
fn merge_with_select() {
    type NameOnly = select!(test_struct: name);
    type IdOnly = select!(test_struct: id);

    let name_val: TestStruct<NameOnly> = TestStruct::empty().name("Alice".to_owned());
    let id_val: TestStruct<IdOnly> = TestStruct::empty().id(123);

    let merged = name_val.merge(id_val);
    assert_eq!(merged.name, "Alice");
    assert_eq!(merged.id, 123);
}

#[test]
fn split_with_select() {
    let full = TestStruct::empty()
        .name("Charlie".to_owned())
        .email("charlie@test.com".to_owned())
        .id(789);

    let (selected, remainder) = full.extract::<select!(test_struct: name, id)>();
    assert_eq!(selected.name, "Charlie");
    assert_eq!(selected.id, 789);
    assert!(selected.get_email().is_none());

    assert_eq!(remainder.email, "charlie@test.com");
    assert!(remainder.get_name().is_none());
    assert!(remainder.get_id().is_none());
}

#[test]
fn try_extract_failure() {
    // Fails when Optional is None but Present required
    let optional_email: TestStruct<select!(test_struct: name, ?email)> =
        TestStruct::empty().name("Test".to_owned()).email(None);
    let result = optional_email.try_extract::<select!(test_struct: name, email)>();
    assert!(
        result.is_err(),
        "try_extract should fail when Optional field is None but target needs Present"
    );
}

#[test]
fn try_extract_returns_exact_original() {
    // Returns exact original on failure
    let original: TestStruct<select!(test_struct: name, ?email, id)> = TestStruct::empty()
        .name("Alice".to_owned())
        .email(None)
        .id(123);

    let cloned = original.clone();
    let result = original.try_extract::<select!(test_struct: name, email, id)>();

    assert!(result.is_err());
    let returned = result.unwrap_err();
    assert_eq!(
        returned, cloned,
        "try_extract should return exact original on failure"
    );
}

#[test]
fn try_extract_failure_at_different_positions() {
    // Failure: Optional in middle
    let partial: TestStruct<select!(test_struct: name, ?email, id)> = TestStruct::empty()
        .name("Bob".to_owned())
        .email(None)
        .id(456);

    let cloned = partial.clone();
    let result = partial.try_extract::<select!(test_struct: name, email, id)>();

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), cloned);

    // Failure: Optional at end
    let partial2: TestStruct<select!(test_struct: name, email, ?id)> = TestStruct::empty()
        .name("Charlie".to_owned())
        .email("charlie@test.com".to_owned())
        .id(None);

    let cloned2 = partial2.clone();
    let result2 = partial2.try_extract::<select!(test_struct: name, email, id)>();

    assert!(result2.is_err());
    assert_eq!(result2.unwrap_err(), cloned2);
}

#[test]
fn try_extract_without_clone() {
    // Works without Clone
    let ncs = NoClone::empty().value("test".to_owned()).id(42);

    match ncs.try_extract::<select!(no_clone: value)>() {
        Ok((selected, remainder)) => {
            assert_eq!(selected.value, "test");
            assert_eq!(remainder.id, 42);
        }
        Err(_) => panic!("Expected success"),
    }

    // Failure without Clone
    let ncs2 = NoClone::empty().value(None).id(99);

    match ncs2.try_extract::<select!(no_clone: value)>() {
        Ok(_) => panic!("Expected error when Optional is None"),
        Err(returned) => {
            assert_eq!(returned.get_value(), None);
            assert_eq!(returned.id, 99);
        }
    }
}

#[test]
fn try_extract_multiple_optional_fields() {
    // Multiple Optional: only second None
    let partial: TestStruct<select!(test_struct: ?name, ?email, id)> = TestStruct::empty()
        .name(Some("Alice".to_owned()))
        .email(None)
        .id(123);

    let cloned = partial.clone();
    let result = partial.try_extract::<select!(test_struct: name, email)>();

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), cloned);
}

#[test]
fn try_extract_success_then_merge_and_reverse_split() {
    let original = TestStruct::empty()
        .name("Bob".to_owned())
        .email("bob@test.com".to_owned())
        .id(456);

    let original_cloned = original.clone();

    // Convert email: Present → Optional
    let result = original.try_extract::<select!(test_struct: name, ?email)>();
    assert!(result.is_ok());
    let (selected, remainder) = result.unwrap();

    let expected_selected = TestStruct::empty()
        .name(original_cloned.name.clone())
        .email(Some(original_cloned.email.clone()));
    assert_eq!(selected, expected_selected);

    // Merge back (email now Optional)
    let reconstructed = selected.merge(remainder);
    let expected_reconstructed = TestStruct::empty()
        .name(original_cloned.name.clone())
        .email(Some(original_cloned.email.clone()))
        .id(original_cloned.id);
    assert_eq!(reconstructed, expected_reconstructed);

    // Convert back: Optional → Present
    let result2 = reconstructed.try_extract::<select!(test_struct: email)>();
    assert!(result2.is_ok());
    let (selected2, remainder2) = result2.unwrap();

    let expected_selected2 = TestStruct::empty().email(original_cloned.email.clone());
    let expected_remainder2 = TestStruct::empty()
        .name(original_cloned.name.clone())
        .id(original_cloned.id);
    assert_eq!(selected2, expected_selected2);
    assert_eq!(remainder2, expected_remainder2);
}

#[test]
fn bounded_impl_with_select() {
    type NameOnly = select!(test_struct: name);
    let val: TestStruct<NameOnly> = TestStruct::empty().name("Diana".to_owned());
    assert_eq!(val.greet(), "Hello, Diana!");
}

#[test]
fn bounded_impl_with_modify() {
    type NameAndEmail = select!(test_struct: name, email);
    let val: TestStruct<NameAndEmail> = TestStruct::empty()
        .name("Eve".to_owned())
        .email("eve@test.com".to_owned());
    assert_eq!(val.email_subject(), "Welcome, Eve! <eve@test.com>");
}

#[test]
fn get_field_mut() {
    // Direct type alias (not select!)
    type NameOnly = test_struct::with::name::Present;
    let mut val: TestStruct<NameOnly> = TestStruct::empty().name("Test".to_owned());

    if let Some(name) = val.get_name_mut() {
        *name = "Modified".to_owned();
    }
    assert_eq!(val.name, "Modified");
}

#[test]
fn unset_field() {
    let full = TestStruct::empty().name("Test".to_owned());
    let val = full.name(PhantomData);
    assert!(val.get_name().is_none());
}

#[test]
fn merge_conflict_resolution() {
    let user1 = TestStruct::empty().name("Alice".to_owned()).id(111);
    let user2 = TestStruct::empty().name("Bob".to_owned()).id(222);
    let merged = user1.merge(user2);
    // user2 wins
    assert_eq!(merged.name, "Bob");
    assert_eq!(merged.id, 222);
}

#[test]
fn raw_identifiers() {
    let cfg = RawIdConfig::empty()
        .r#type("test".to_owned())
        .r#match(true)
        .normal(42);

    assert_eq!(cfg.r#type, "test");
    assert!(cfg.r#match);
    assert_eq!(cfg.normal, 42);
}

#[test]
fn raw_identifiers_extract() {
    let cfg = RawIdConfig::empty()
        .r#type("test".to_owned())
        .r#match(true)
        .normal(42);

    let (extracted, remainder) = cfg.extract::<select!(raw_id_config: r#type, r#match)>();
    assert_eq!(extracted.r#type, "test");
    assert!(extracted.r#match);
    assert_eq!(remainder.normal, 42);
}

#[test]
fn raw_identifiers_try_extract() {
    let cfg = RawIdConfig::empty()
        .r#type(Some("optional".to_owned()))
        .r#match(Some(false))
        .normal(99);

    let result = cfg.try_extract::<select!(raw_id_config: r#type, r#match)>();
    assert!(result.is_ok());
    let (extracted, remainder) = result.unwrap();
    assert_eq!(extracted.r#type, "optional");
    assert!(!extracted.r#match);
    assert_eq!(remainder.normal, 99);

    let partial = RawIdConfig::empty()
        .r#type(Some("value".to_owned()))
        .r#match(None)
        .normal(77);

    let result = partial.try_extract::<select!(raw_id_config: r#type, r#match)>();
    assert!(result.is_err());
}

#[test]
fn select_with_module_path() {
    type NameOnly = select!(crate::test_struct: name);
    let val: TestStruct<NameOnly> = TestStruct::empty().name("Alice".to_owned());
    assert_eq!(val.name, "Alice");
}

#[test]
fn single_field_struct() {
    type DataPresent = select!(single_field: data);
    let val: SingleField<DataPresent> = SingleField::empty().data("test".to_owned());
    assert_eq!(val.data, "test");

    type DataOptional = select!(single_field: ?data);
    let val2: SingleField<DataOptional> = SingleField::empty().data(Some("test".to_owned()));
    assert_eq!(val2.data, Some("test".to_owned()));

    let val3: SingleField<DataOptional> = SingleField::empty().data(None);
    assert_eq!(val3.data, None);
}

#[test]
fn select_with_trailing_comma() {
    type NameOnly = select!(test_struct: name,);
    let val: TestStruct<NameOnly> = TestStruct::empty().name("Alice".to_owned());
    assert_eq!(val.name, "Alice");

    type NameAndEmail = select!(test_struct: name, email,);
    let val2: TestStruct<NameAndEmail> = TestStruct::empty()
        .name("Bob".to_owned())
        .email("bob@test.com".to_owned());
    assert_eq!(val2.name, "Bob");
    assert_eq!(val2.email, "bob@test.com");

    type OptionalName = select!(test_struct: ?name,);
    let val3: TestStruct<OptionalName> = TestStruct::empty().name(Some("Charlie".to_owned()));
    assert_eq!(val3.name, Some("Charlie".to_owned()));
}

#[test]
fn generic_extract_to_absent() {
    fn remove_email<F: test_struct::Fields>(data: TestStruct<F>) -> TestStruct<test_struct::with::email::Absent<F>> {
        let (extracted, _) = data.extract::<test_struct::with::email::Absent<F>>();
        extracted
    }

    let complete = TestStruct::empty()
        .name("Alice".to_owned())
        .email("alice@example.com".to_owned())
        .id(123);

    let without_email = remove_email(complete);
    assert_eq!(without_email.name, "Alice");
    assert_eq!(without_email.id, 123);
}

#[test]
fn generic_extract_to_optional() {
    fn extract_name_as_optional<F: test_struct::Fields>(
        data: TestStruct<F>,
    ) -> TestStruct<select!(test_struct: ?name)> {
        let (extracted, _) = data.extract::<select!(test_struct: ?name)>();
        extracted
    }

    let complete = TestStruct::empty()
        .name("Bob".to_owned())
        .email("bob@example.com".to_owned())
        .id(456);

    let with_optional_name = extract_name_as_optional(complete);
    assert_eq!(with_optional_name.name, Some("Bob".to_owned()));
}

impl<F: test_struct::Fields<name = Present>> TestStruct<F> {
    pub fn greet(&self) -> String {
        format!("Hello, {}!", self.name)
    }
}

impl<F: test_struct::Fields<name = Present, email = Present>> TestStruct<F> {
    pub fn email_subject(&self) -> String {
        format!("Welcome, {}! <{}>", self.name, self.email)
    }
}
