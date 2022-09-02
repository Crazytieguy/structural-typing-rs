#![feature(negative_impls)]
#![feature(auto_traits)]
// TODO: #![deny(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::trait_duplication_in_bounds)]

mod record;
pub use record::{Has, Property, Record, WithDefault, P};

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use super::*;
    struct Name;
    impl Property for Name {
        type Type = String;
    }

    fn shout_name<T: Has<Name>>(person: &T) -> String {
        person.get::<Name>().to_uppercase()
    }

    #[test]
    fn simple_example() {
        let mut john = ().insert::<Name>("John".into());
        john.get_mut::<Name>().push_str("son");
        assert_eq!(shout_name(&john), "JOHNSON");
    }

    struct Age;
    impl Property for Age {
        type Type = u8;
    }
    // Fathers have to be people (in this case, have Name and Age)
    struct Father<T: Person>(PhantomData<T>);
    impl<T: Person> Property for Father<T> {
        type Type = T;
    }

    impl<T: Has<Name> + Has<Age>> Person for T {}
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
        let mike =
            ().insert::<Father<_>>(().insert::<Age>(35).insert::<Name>("Nate".into()))
                .insert::<Age>(3)
                .insert::<Name>("Mike".into());
        assert_eq!(
            mike.say_hello_with_father(),
            "Hi! my name is Mike and I'm 3 years old.\n\
            Hi! my name is Nate and I'm 35 years old."
        );
    }

    struct IsAdmin;
    impl Property for IsAdmin {
        type Type = bool;
    }

    fn access_allowed<T: WithDefault<IsAdmin>>(user: &T) -> bool {
        *user.get_or::<IsAdmin>(&false)
    }

    fn with_is_admin<T: WithDefault<IsAdmin>>(user: T) -> (T, P<IsAdmin>) {
        user.insert_default::<IsAdmin>(false)
    }

    #[test]
    fn defaults_example() {
        let user = ().insert::<Name>("Hope".into());
        assert!(!access_allowed(&user));
        let mut user = with_is_admin(user);
        assert!(!user.get::<IsAdmin>());
        *user.get_mut::<IsAdmin>() = true;
        let user = with_is_admin(user);
        assert!(user.get::<IsAdmin>());
    }
}
