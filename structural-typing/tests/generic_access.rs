use structural_typing::{structural, Access, Present};

#[structural]
struct Person {
    name: String,
    age: u8,
    height: f32,
}

impl<S> Person<S>
where
    S: person_state::State<Name = Present>,
{
    fn describe(&self) -> String {
        // We know name is Present, but age could be Present, Absent, or Optional
        // We should be able to call .get() on age regardless
        match self.age.get() {
            Some(age) => format!("{} is {} years old", self.name, age),
            None => format!("{} (age unknown)", self.name),
        }
    }
}

#[test]
fn test_generic_access() {
    let person_with_age = Person::empty()
        .name("Alice".into())
        .age(30);

    assert_eq!(person_with_age.describe(), "Alice is 30 years old");

    let person_without_age = Person::empty()
        .name("Bob".into());

    assert_eq!(person_without_age.describe(), "Bob (age unknown)");
}
