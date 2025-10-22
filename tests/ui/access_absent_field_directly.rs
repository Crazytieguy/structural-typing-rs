use structural_typing::presence::{Absent, Presence};
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
}

fn main() {
    let user = User::empty();
    let _name_str: String = user.name;
}
