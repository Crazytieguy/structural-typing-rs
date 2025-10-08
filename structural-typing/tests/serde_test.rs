use structural_typing::structural;
use serde::{Serialize, Deserialize};

#[structural]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
}

#[test]
fn test_serialize() {
    let person = Person::empty()
        .name("Alice".into())
        .age(30);

    let json = serde_json::to_string(&person).unwrap();
    assert!(json.contains("Alice"));
    assert!(json.contains("30"));
}

#[test]
fn test_deserialize() {
    let json = r#"{"name":"Bob","age":25}"#;
    let person: FullPerson = serde_json::from_str(json).unwrap();

    assert_eq!(person.name, "Bob");
    assert_eq!(person.age, 25);
}

#[test]
fn test_roundtrip() {
    let original: FullPerson = Person::empty()
        .name("Charlie".into())
        .age(35);

    let json = serde_json::to_string(&original).unwrap();
    let deserialized: FullPerson = serde_json::from_str(&json).unwrap();

    assert_eq!(original, deserialized);
}

#[test]
fn test_partial_eq() {
    let p1 = Person::empty().name("Alice".into()).age(30);
    let p2 = Person::empty().name("Alice".into()).age(30);
    let p3 = Person::empty().name("Bob".into()).age(30);

    assert_eq!(p1, p2);
    assert_ne!(p1, p3);
}
