use structural_typing::structural;

#[structural]
struct User {
    name: String,
    email: String,
}

fn main() {
    // String literal instead of field name
    type Invalid = user::select!("name");
}
