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
    struct Age(u8);
    struct Father<T>(T);

    trait NameOps: Has<Name> {
        fn shout_name(&self) -> String {
            self.get::<Name>().to_uppercase()
        }
    }

    trait PersonOps: Has<Name> + Has<Age> {
        fn say_hello(&self) -> String {
            let name = self.get::<Name>();
            let age = self.get::<Age>();
            format!("Hi! my name is {name} and I'm {age} years old.")
        }
    }

    #[test]
    fn example() {
        let mut john = (Name("John".into()), Age(26));
        assert_eq!(john.shout_name(), "JOHN");
        john.get_mut::<Name>().push_str("son");
        assert_eq!(
            john.say_hello(),
            "Hi! my name is Johnson and I'm 26 years old."
        );
        (
            Father((Name("Nate".into()), Age(35))),
            Name("Mike".into()),
            123,
            Age(3),
        )
            .get::<Father<_>>()
            .say_hello();
    }

    // Everything below here could be generated from the above code using macros

    impl<T: Has<Name>> NameOps for T {}
    impl<T: Has<Name> + Has<Age>> PersonOps for T {}
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
    impl<T> Property for Father<T> {
        type Item = T;
        fn get(&self) -> &Self::Item {
            &self.0
        }
        fn get_mut(&mut self) -> &mut Self::Item {
            &mut self.0
        }
    }
}
