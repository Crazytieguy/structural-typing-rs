//! # Basic Example
//!
//! This example demonstrates the basic builder pattern and merging.

use structural_typing::structural;

#[structural]
struct Person {
    name: String,
    age: u8,
    height: f32,
}

fn main() {
    // Create a person with just a name
    let john = Person::empty().name("John".into());
    println!("Created person: {:?}", john.name);

    // Add age - returns a new type with age present
    let john_with_age = john.age(26);
    println!("Added age: {}", john_with_age.age);

    // Merge two partial persons
    let merged = Person::empty()
        .name("Alice".into())
        .merge(Person::empty().age(30));
    println!("Merged: name={}, age={}", merged.name, merged.age);
}
