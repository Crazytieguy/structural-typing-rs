use structural_typing::{presence::Present, structural};

#[structural]
struct User {
    name: String,
}

impl<F: user::Fields<name = Present>> User<F> {
    pub fn greet(&self) -> String {
        format!("Hello, {}!", self.name)
    }
}

fn main() {
    let user = user::empty();
    user.greet();
}
