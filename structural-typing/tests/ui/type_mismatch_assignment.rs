use structural_typing::structural;

#[structural]
struct User {
    name: String,
}

fn main() {
    let user_with_name = User::empty().name("Alice".to_owned());
    let user_without_name: User<user::AllAbsent> = user_with_name;
}
