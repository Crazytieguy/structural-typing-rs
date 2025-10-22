use structural_typing::presence::{Absent, Present, Presence};
use std::marker::PhantomData;

#[allow(non_camel_case_types)]
mod user {
    use super::*;

    mod sealed {
        pub trait Sealed {}
    }

    pub trait Fields: sealed::Sealed {
        type name: Presence;
    }

    pub struct FieldSet<name: Presence>(PhantomData<name>);

    impl<name: Presence> sealed::Sealed for FieldSet<name> {}

    impl<name: Presence> Fields for FieldSet<name> {
        type name = name;
    }
}

struct User<F: user::Fields = user::FieldSet<Absent>> {
    pub name: <F::name as Presence>::Output<String>,
}

impl User {
    pub fn empty() -> Self {
        Self {
            name: PhantomData,
        }
    }

    pub fn name(self, name: String) -> User<user::FieldSet<Present>> {
        User { name }
    }
}

fn main() {
    let user_with_name = User::empty().name("Alice".into());
    let user_without_name: User<user::FieldSet<Absent>> = user_with_name;
}
