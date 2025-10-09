//! # Methods Example
//!
//! This example demonstrates implementing methods that require specific fields.
//! Methods can require different combinations of fields to be present.

use structural_typing::{structural, Access, Present};

#[structural]
struct Person {
    name: String,
    age: u8,
    height: f32,
}

// Method that requires ONLY name - can check age optionally using Access trait
impl<S> Person<S>
where
    S: person_state::State<Name = Present>,
{
    fn shout_name(&self) -> String {
        let uppercase = self.name.to_uppercase();
        // Use Access trait to check if age is present
        match self.age.get() {
            Some(age) => format!("{uppercase} ({age})"),
            None => uppercase,
        }
    }
}

// Method that requires BOTH name AND age
impl<S> Person<S>
where
    S: person_state::State<Name = Present, Age = Present>,
{
    fn say_hello(&self) -> String {
        format!(
            "Hi! I'm {name} and I'm {age} years old",
            name = self.name,
            age = self.age
        )
    }
}

fn main() {
    // Create person with only name
    let john = Person::empty().name("John".into());
    println!("With name: {}", john.shout_name());
    // john.say_hello(); // Won't compile - needs age!

    // Add age - now we can call say_hello
    let john = john.age(26);
    println!("With age: {}", john.say_hello());

    // shout_name behavior changes when age is present
    println!("Shout name: {}", john.shout_name());
}
