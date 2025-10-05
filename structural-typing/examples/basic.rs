use structural_typing::structural;

#[structural]
struct Person {
    name: String,
    age: u8,
    height: f32,
}

fn main() {
    let john = Person::empty().name("John".into());

    println!("Created person with name: {:?}", john.name);

    let john_with_age = john.age(26);

    println!("Added age: {}", john_with_age.age);

    let merged = Person::empty()
        .name("Alice".into())
        .merge(Person::empty().age(30));

    println!("Merged person: name={}, age={}", merged.name, merged.age);
}
