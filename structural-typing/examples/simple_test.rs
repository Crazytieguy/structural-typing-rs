use structural_typing::{presence::Present, structural};

#[structural]
#[derive(Clone, Debug)]
struct Person {
    name: String,
    age: u8,
}

impl<F: person::Fields<name = Present>> Person<F> {
    fn greet(&self) -> String {
        format!("Hello, {}!", self.name)
    }
}

fn main() {
    let person = Person::empty().name("Alice".to_string());
    println!("{}", person.greet());
    assert_eq!(person.greet(), "Hello, Alice!");

    let person_with_age = person.age(30);
    assert_eq!(person_with_age.name, "Alice");
    assert_eq!(person_with_age.age, 30);

    println!("✓ Simple test passed!");
}
