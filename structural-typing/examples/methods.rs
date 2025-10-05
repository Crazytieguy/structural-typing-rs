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
    fn shout_name(&self) -> String {
        let uppercase = self.name.to_uppercase();
        match self.age.get() {
            Some(age) => format!("{uppercase} ({age})"),
            None => uppercase,
        }
    }
}

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
    let john = Person::empty().name("John".into());

    println!("Can call shout_name: {}", john.shout_name());

    let john = john.age(26);

    println!("Now can call say_hello: {}", john.say_hello());

    println!("shout_name behavior changed: {}", john.shout_name());
}
