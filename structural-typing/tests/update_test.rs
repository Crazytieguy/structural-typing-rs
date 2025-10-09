use structural_typing::structural;

#[structural]
struct Person {
    name: String,
    age: u8,
}

#[test]
fn test_update_overwrites_existing() {
    let mut person = Person::empty().name("Alice".into()).age(25);

    let update = Person::empty().name("Bob".into()).age(30);
    person.update(update);

    assert_eq!(person.name, "Bob");
    assert_eq!(person.age, 30);
}

#[test]
fn test_update_partial() {
    let mut person = Person::empty().name("Alice".into()).age(25);

    // Only update age, keep name
    let update = Person::empty().age(30);
    person.update(update);

    assert_eq!(person.name, "Alice");
    assert_eq!(person.age, 30);
}

#[test]
fn test_update_ignores_absent_fields() {
    let mut person = Person::empty().name("Alice".into()).age(25);

    // update doesn't have age set - should not change person's age
    let update = Person::empty().name("Bob".into());
    person.update(update);

    assert_eq!(person.name, "Bob");
    assert_eq!(person.age, 25);  // unchanged
}
