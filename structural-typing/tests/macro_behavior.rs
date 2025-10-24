use structural_typing::{presence::Present, structural};

#[structural]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct TestStruct {
    name: String,
    email: String,
    id: u64,
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
fn select_duplicate_first_wins() {
    type DuplicateName = test_struct::select!(name, name);
    let val: TestStruct<DuplicateName> = TestStruct::empty().name("Test".into());
    assert_eq!(val.name, "Test");
}

#[test]
fn select_duplicate_different_presence_first_wins() {
    type FirstPresent = test_struct::select!(name, ?name);
    let _: TestStruct<FirstPresent> = TestStruct::empty().name("Test".into());

    type FirstOptional = test_struct::select!(?name, name);
    let _: TestStruct<FirstOptional> = TestStruct::empty().maybe_name(Some("Test".into()));
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
fn modify_no_op() {
    type Same = test_struct::modify!(test_struct::AllAbsent);
    let val: TestStruct<Same> = TestStruct::empty();
    assert!(val.get_name().is_none());
}

#[test]
fn modify_conflicting_specs_first_wins() {
    type FirstWins = test_struct::modify!(test_struct::AllAbsent, +name, -name);
    let val: TestStruct<FirstWins> = TestStruct::empty().name("Test".into());
    assert_eq!(val.name, "Test");
}

#[test]
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
fn serde_with_modify() {
    type OnlyName = test_struct::modify!(test_struct::AllAbsent, +name);
    let val: TestStruct<OnlyName> = TestStruct::empty().name("Henry".into());

    let json = serde_json::to_string(&val).unwrap();
    assert_eq!(json, r#"{"name":"Henry"}"#);

    let full_json = r#"{"name":"Henry","email":"","id":0}"#;
    let deserialized: TestStruct<test_struct::AllPresent> =
        serde_json::from_str(full_json).unwrap();
    assert_eq!(deserialized.name, "Henry");
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
fn merge_with_modify() {
    type NameAndEmail = test_struct::modify!(test_struct::AllAbsent, +name, +email);
    type IdOnly = test_struct::modify!(test_struct::AllAbsent, +id);

    let partial1: TestStruct<NameAndEmail> = TestStruct::empty()
        .name("Bob".into())
        .email("bob@test.com".into());
    let partial2: TestStruct<IdOnly> = TestStruct::empty().id(456);

    let merged = partial1.merge(partial2);
    assert_eq!(merged.name, "Bob");
    assert_eq!(merged.email, "bob@test.com");
    assert_eq!(merged.id, 456);
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
