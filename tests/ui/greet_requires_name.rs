use structural_typing::presence::{Absent, Present, Presence};
use std::marker::PhantomData;

mod user {
    use super::*;

    mod sealed {
        pub trait Sealed {}
    }

    #[allow(non_camel_case_types)]
    pub trait Fields: sealed::Sealed {
        type name: Presence;
    }

    #[allow(non_camel_case_types)]
    pub struct FieldSet<name: Presence>(PhantomData<name>);

    #[allow(non_camel_case_types)]
    impl<name: Presence> sealed::Sealed for FieldSet<name> {}

    #[allow(non_camel_case_types)]
    impl<name: Presence> Fields for FieldSet<name> {
        type name = name;
    }
}

struct User<F: user::Fields = user::FieldSet<Absent>> {
    pub name: <F::name as Presence>::Output<String>,
}

impl<F: user::Fields<name = Present>> User<F> {
    pub fn greet(&self) -> String {
        format!("Hello, {}!", self.name)
    }
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
    user.greet();
}
