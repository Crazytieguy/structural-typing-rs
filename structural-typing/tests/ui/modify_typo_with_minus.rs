use structural_typing::structural;

#[structural]
struct User {
    name: String,
    email: String,
}

fn main() {
    // Typo with - operator
    type Invalid = user::modify!(user::AllPresent, -emial);
}
