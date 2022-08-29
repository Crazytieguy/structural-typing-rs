#![feature(negative_impls)]
#![feature(auto_traits)]
#![warn(clippy::pedantic)]
#![allow(clippy::trait_duplication_in_bounds)]

mod property;

/// Everything here is completely safe and can't panic at runtime
#[cfg(test)]
mod tests {
    use crate::property::{Access, Has, Property};

    struct Name(String);

    fn shout_name<T: Has<Name>>(person: &T) -> String {
        person.get::<Name>().to_uppercase()
    }

    #[test]
    fn simple_example() {
        let mut john = (Name("John".into()),);
        john.get_mut::<Name>().push_str("son");
        assert_eq!(shout_name(&john), "JOHNSON");
    }

    struct Age(u8);
    // Fathers have to be people (in this case, have Name and Age)
    struct Father<T: Person>(T);

    trait Person: Has<Name> + Has<Age> {
        fn say_hello(&self) -> String {
            let name = self.get::<Name>();
            let age = self.get::<Age>();
            format!("Hi! my name is {name} and I'm {age} years old.")
        }

        fn say_hello_with_father<T: Person>(&self) -> String
        where
            Self: Has<Father<T>>,
        {
            format!(
                "{}\n{}",
                self.say_hello(),
                self.get::<Father<_>>().say_hello()
            )
        }
    }

    #[test]
    fn nested_trait_example() {
        let mike = (
            Father((Name("Nate".into()), Age(35))),
            Age(3),
            Name("Mike".into()),
        );
        assert_eq!(
            mike.say_hello_with_father(),
            "Hi! my name is Mike and I'm 3 years old.\n\
            Hi! my name is Nate and I'm 35 years old."
        );
    }

    // Everything below here could be generated from the above code using macros

    impl<T: Has<Name> + Has<Age>> Person for T {}
    impl Property for Name {
        type Item = String;
        fn get(&self) -> &Self::Item {
            &self.0
        }
        fn get_mut(&mut self) -> &mut Self::Item {
            &mut self.0
        }
    }
    impl Property for Age {
        type Item = u8;
        fn get(&self) -> &Self::Item {
            &self.0
        }
        fn get_mut(&mut self) -> &mut Self::Item {
            &mut self.0
        }
    }
    impl<T: Person> Property for Father<T> {
        type Item = T;
        fn get(&self) -> &Self::Item {
            &self.0
        }
        fn get_mut(&mut self) -> &mut Self::Item {
            &mut self.0
        }
    }
}
