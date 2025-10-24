use structural_typing::structural;

#[structural]
struct User {
    name: String,
    email: String,
}

fn main() {
    // One valid field, one typo - should error on the typo
    type Invalid = user::select!(name, emial);
}
