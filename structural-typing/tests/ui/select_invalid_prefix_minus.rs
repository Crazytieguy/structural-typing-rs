use structural_typing::structural;

#[structural]
struct User {
    name: String,
    email: String,
}

fn main() {
    type Invalid = user::select!(-email);
}
