#![feature(negative_impls)]
#![feature(auto_traits)]
#![warn(clippy::pedantic)]
#![allow(clippy::trait_duplication_in_bounds)]

mod property;

#[cfg(test)]
mod tests {
    use crate::property::{Access, Has, Property};

    struct Name<'a>(&'a str);
    struct Age(u8);
    struct Father<T>(T);

    trait NameOps<'a>: Has<Name<'a>> {
        fn shout_name(&self) -> String {
            self.get::<Name>().to_uppercase()
        }
    }

    trait PersonOps<'a>: Has<Name<'a>> + Has<Age> {
        fn say_hello(&self) -> String {
            let name = self.get::<Name>();
            let age = self.get::<Age>();
            format!("Hi! my name is {name} and I'm {age} years old.")
        }
    }

    #[test]
    fn getting() {
        let john = (Name("John"), Age(26));
        assert_eq!(john.shout_name(), "JOHN");
        assert_eq!(
            john.say_hello(),
            "Hi! my name is John and I'm 26 years old."
        );
        (Father((Name("Nate"), Age(35))), Name("Mike"), 123, Age(3))
            .get::<Father<_>>()
            .say_hello();
    }

    // Everything below here can be generated with a derive macro

    impl<'a, T: Has<Name<'a>>> NameOps<'a> for T {}
    impl<'a, T: Has<Name<'a>> + Has<Age>> PersonOps<'a> for T {}
    impl<'a> Property for Name<'a> {
        type Item = &'a str;
        fn get(&self) -> &Self::Item {
            &self.0
        }
    }

    impl Property for Age {
        type Item = u8;
        fn get(&self) -> &Self::Item {
            &self.0
        }
    }

    impl<T> Property for Father<T> {
        type Item = T;
        fn get(&self) -> &Self::Item {
            &self.0
        }
    }
}
