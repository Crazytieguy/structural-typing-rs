//! Demonstrates struct with generic type parameters

use structural_typing::{presence::Present, structural};

#[structural]
struct Address {
    city: String,
    country: String,
}

#[structural]
struct User<A: address::Fields> {
    name: String,
    email: String,
    address: Address<A>,
}

fn main() {
    // Generic struct with full nested type
    let address = Address::empty()
        .city("Tokyo".to_owned())
        .country("Japan".to_owned());

    let user = User::empty()
        .name("Alice".to_owned())
        .email("alice@example.com".to_owned())
        .address(address);

    assert_eq!(user.name, "Alice");
    assert_eq!(user.address.city, "Tokyo");

    // Generic struct with partial nested type
    let partial = Address::empty().city("London".to_owned());

    let user2 = User::empty()
        .name("Bob".to_owned())
        .email("bob@example.com".to_owned())
        .address(partial);

    assert_eq!(user2.email, "bob@example.com");
    assert_eq!(user2.address.city, "London");

    // Generic impl requiring specific fields
    fn get_email<A: address::Fields>(user: User<A, impl user::Fields<email = Present>>) -> String {
        user.email
    }

    assert_eq!(get_email(user2), "bob@example.com");
}
