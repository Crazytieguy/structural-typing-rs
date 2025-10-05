use structural_typing::structural;

#[structural]
#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u8,
}

#[test]
fn test_derive_debug() {
    let person = Person::empty().name("Alice".into()).age(30);
    let debug_str = format!("{:?}", person);
    assert!(debug_str.contains("Alice"));
}

#[test]
fn test_derive_clone() {
    let person = Person::empty().name("Bob".into()).age(25);
    let cloned = person.clone();
    assert_eq!(cloned.name, "Bob");
    assert_eq!(cloned.age, 25);
}
