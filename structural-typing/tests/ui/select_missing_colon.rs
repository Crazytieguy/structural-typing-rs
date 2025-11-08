use structural_typing::{select, structural};

#[structural]
struct User {
    name: String,
}

fn main() {
    type Invalid = select!(user name);
}
