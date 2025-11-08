use structural_typing::structural;

#[structural]
struct User {
    name: String,
}

fn takes_empty(_user: User) {}

fn main() {
    let user_with_name = User::empty().name("Alice".to_owned());
    takes_empty(user_with_name);
}
