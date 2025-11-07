use structural_typing::{presence::Present, structural};

#[structural]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct TestStruct {
    name: String,
    email: String,
    id: u64,
}

#[structural]
struct RawIdConfig {
    r#type: String,
    r#match: bool,
    normal: u32,
}

#[test]
fn select_basic() {
    type NameOnly = test_struct::select!(name);
    let val: TestStruct<NameOnly> = TestStruct::empty().name("Alice".into());
    assert_eq!(val.name, "Alice");
}

#[test]
fn select_multiple() {
    type NameAndEmail = test_struct::select!(name, email);
    let val: TestStruct<NameAndEmail> = TestStruct::empty()
        .name("Bob".into())
        .email("bob@test.com".into());
    assert_eq!(val.name, "Bob");
    assert_eq!(val.email, "bob@test.com");
}

#[test]
fn select_optional() {
    type NameAndMaybeEmail = test_struct::select!(name, ?email);
    let val: TestStruct<NameAndMaybeEmail> = TestStruct::empty()
        .name("Charlie".into())
        .maybe_email(Some("charlie@test.com".into()));
    assert_eq!(val.name, "Charlie");
    assert_eq!(val.email, Some("charlie@test.com".to_string()));
}

#[test]
fn select_empty_all_absent() {
    type AllAbsent = test_struct::select!();
    let val: TestStruct<AllAbsent> = TestStruct::empty();
    assert!(val.get_name().is_none());
    assert!(val.get_email().is_none());
    assert!(val.get_id().is_none());
}

#[test]
fn modify_add_fields() {
    type NameAndEmail = test_struct::modify!(test_struct::AllAbsent, +name, +email);
    let val: TestStruct<NameAndEmail> = TestStruct::empty()
        .name("Dave".into())
        .email("dave@test.com".into());
    assert_eq!(val.name, "Dave");
    assert_eq!(val.email, "dave@test.com");
}

#[test]
fn modify_remove_fields() {
    type OnlyName = test_struct::modify!(test_struct::AllPresent, -email, -id);
    let val: TestStruct<OnlyName> = TestStruct::empty().name("Eve".into());
    assert_eq!(val.name, "Eve");
    assert!(val.get_email().is_none());
    assert!(val.get_id().is_none());
}

#[test]
fn modify_make_optional() {
    type NameAndMaybeEmail = test_struct::modify!(test_struct::AllPresent, ?email, -id);
    let val: TestStruct<NameAndMaybeEmail> = TestStruct::empty()
        .name("Frank".into())
        .maybe_email(None);
    assert_eq!(val.name, "Frank");
    assert_eq!(val.email, None);
}

#[test]
#[cfg(feature = "serde")]
fn serde_with_select() {
    type NameAndEmail = test_struct::select!(name, email);
    let val: TestStruct<NameAndEmail> = TestStruct::empty()
        .name("Grace".into())
        .email("grace@test.com".into());

    let json = serde_json::to_string(&val).unwrap();
    assert!(json.contains(r#""name":"Grace"#));
    assert!(json.contains(r#""email":"grace@test.com"#));
    assert!(!json.contains("id"), "Absent field should not appear");

    let full_json = r#"{"name":"Grace","email":"grace@test.com","id":0}"#;
    let deserialized: TestStruct<test_struct::AllPresent> =
        serde_json::from_str(full_json).unwrap();
    assert_eq!(deserialized.name, "Grace");
    assert_eq!(deserialized.email, "grace@test.com");
}

#[test]
fn merge_with_select() {
    type NameOnly = test_struct::select!(name);
    type IdOnly = test_struct::select!(id);

    let name_val: TestStruct<NameOnly> = TestStruct::empty().name("Alice".into());
    let id_val: TestStruct<IdOnly> = TestStruct::empty().id(123);

    let merged = name_val.merge(id_val);
    assert_eq!(merged.name, "Alice");
    assert_eq!(merged.id, 123);
}

#[test]
fn project_with_select() {
    let full: TestStruct<test_struct::AllPresent> = TestStruct::empty()
        .name("Charlie".into())
        .email("charlie@test.com".into())
        .id(789);

    let projected = full.project::<test_struct::select!(name, id)>();
    assert_eq!(projected.name, "Charlie");
    assert_eq!(projected.id, 789);
    assert!(projected.get_email().is_none());
}

#[test]
fn try_project_failure() {
    // try_project fails when source doesn't have required Present field
    let optional_email: TestStruct<test_struct::select!(name, ?email)> =
        TestStruct::empty().name("Test".into()).maybe_email(None);
    let result = optional_email.try_project::<test_struct::select!(name, email)>();
    assert!(result.is_none(), "try_project should fail when Optional field is None but target needs Present");
}

#[test]
fn bounded_impl_with_select() {
    type NameOnly = test_struct::select!(name);
    let val: TestStruct<NameOnly> = TestStruct::empty().name("Diana".into());
    assert_eq!(val.greet(), "Hello, Diana!");
}

#[test]
fn bounded_impl_with_modify() {
    type NameAndEmail = test_struct::modify!(test_struct::AllAbsent, +name, +email);
    let val: TestStruct<NameAndEmail> = TestStruct::empty()
        .name("Eve".into())
        .email("eve@test.com".into());
    assert_eq!(val.email_subject(), "Welcome, Eve! <eve@test.com>");
}

#[test]
fn get_field_mut() {
    type NameOnly = test_struct::select!(name);
    let mut val: TestStruct<NameOnly> = TestStruct::empty().name("Test".into());

    if let Some(name) = val.get_name_mut() {
        *name = "Modified".into();
    }
    assert_eq!(val.name, "Modified");
}

#[test]
fn unset_field() {
    let val = TestStruct::empty().name("Test".into()).unset_name();
    assert!(val.get_name().is_none());
}

#[test]
fn merge_conflict_resolution() {
    let user1 = TestStruct::empty().name("Alice".into()).id(111);
    let user2 = TestStruct::empty().name("Bob".into()).id(222);
    let merged = user1.merge(user2);
    // Second argument (user2) wins
    assert_eq!(merged.name, "Bob");
    assert_eq!(merged.id, 222);
}

#[test]
fn raw_identifiers() {
    let cfg = RawIdConfig::empty()
        .r#type("test".into())
        .r#match(true)
        .normal(42);

    assert_eq!(cfg.r#type, "test");
    assert!(cfg.r#match);
    assert_eq!(cfg.normal, 42);
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
