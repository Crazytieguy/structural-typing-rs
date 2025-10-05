mod types {
    use structural_typing::{structural, Access, Present};

    #[structural]
    pub struct Person {
        pub name: String,
        pub age: u8,
    }

    impl<S> Person<S>
    where
        S: person_state::State<Name = Present>,
    {
        pub fn shout_name(&self) -> String {
            // .get() works here because Access is explicitly imported
            match self.age.get() {
                Some(age) => format!("{} ({})", self.name.to_uppercase(), age),
                None => self.name.to_uppercase(),
            }
        }
    }
}

mod usage_without_import {
    use super::types::Person;
    use structural_typing::Present;

    // This function demonstrates that you NEED to import Access
    // when using .get() in a different module
    pub fn describe_person<S>(person: &Person<S>) -> String
    where
        S: super::types::person_state::State<Name = Present>,
    {
        // To use .get() here, we must import Access:
        use structural_typing::Access;

        match person.age.get() {
            Some(age) => format!("{} is {} years old", person.name, age),
            None => format!("{} (age unknown)", person.name),
        }
    }
}

mod usage_with_import {
    use super::types::Person;
    use structural_typing::{Access, Present};

    // Or import Access at the module level
    pub fn describe_person<S>(person: &Person<S>) -> String
    where
        S: super::types::person_state::State<Name = Present>,
    {
        match person.age.get() {
            Some(age) => format!("{} is {} years old", person.name, age),
            None => format!("{} (age unknown)", person.name),
        }
    }
}

#[test]
fn test_cross_module_requires_import() {
    let person = types::Person::empty()
        .name("Alice".into())
        .age(30);

    // Both approaches work as long as Access is imported somewhere
    let desc1 = usage_without_import::describe_person(&person);
    let desc2 = usage_with_import::describe_person(&person);

    assert_eq!(desc1, "Alice is 30 years old");
    assert_eq!(desc2, "Alice is 30 years old");
}

#[test]
fn test_methods_work_with_explicit_import() {
    let person = types::Person::empty()
        .name("Bob".into())
        .age(25);

    // This works because the types module imports Access
    assert_eq!(person.shout_name(), "BOB (25)");
}
