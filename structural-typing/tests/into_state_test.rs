use structural_typing::{structural, Access};

#[structural]
struct Person {
    name: String,
    age: u8,
    email: String,
}

type BasicInfo = Person<
    person_state::SetEmail<
        person_state::SetName<
            person_state::Empty
        >
    >
>;

#[test]
fn test_into_state_removes_fields() {
    let full: FullPerson = Person::empty()
        .name("Alice".into())
        .age(30)
        .email("alice@example.com".into());

    let basic: BasicInfo = full.into_state();

    assert_eq!(basic.name, "Alice");
    assert_eq!(basic.email, "alice@example.com");
}

#[test]
fn test_into_state_to_empty() {
    let full: FullPerson = Person::empty()
        .name("Bob".into())
        .age(25)
        .email("bob@example.com".into());

    let empty: Person<person_state::Empty> = full.into_state();

    assert_eq!(empty.name.get(), None::<&String>);
    assert_eq!(empty.age.get(), None::<&u8>);
    assert_eq!(empty.email.get(), None::<&String>);
}

#[test]
fn test_into_state_identity() {
    let person: BasicInfo = Person::empty()
        .name("Charlie".into())
        .email("charlie@example.com".into());

    let same: BasicInfo = person.into_state();

    assert_eq!(same.name, "Charlie");
    assert_eq!(same.email, "charlie@example.com");
}
