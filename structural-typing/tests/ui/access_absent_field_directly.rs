use structural_typing::structural;

#[structural]
struct User {
    name: String,
}

fn main() {
    let user = user::empty();
    let _name_str: String = user.name;
}
