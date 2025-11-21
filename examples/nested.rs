//! Demonstrates struct with generic type parameters

use structural_typing::{presence::Present, select, structural};

#[structural]
struct Address {
    city: String,
    country: String,
}

#[structural]
struct User<A: address::Fields = select!(address: all-)> {
    name: String,
    email: String,
    address: Address<A>,
}

fn main() {
    // Generic struct with full nested type
    let address = address::empty()
        .city("Tokyo".to_owned())
        .country("Japan".to_owned());

    let user = user::empty()
        .name("Alice".to_owned())
        .email("alice@example.com".to_owned())
        .address(address);

    assert_eq!(user.name, "Alice");
    assert_eq!(user.address.city, "Tokyo");

    // Generic can change type!
    let partial = address::empty().city("London".to_owned());

    let user = user.address(partial);

    assert_eq!(user.address.city, "London");

    // Generic impl requiring specific fields
    fn get_email<U: user::Fields<email = Present>, A: address::Fields>(user: User<U, A>) -> String {
        user.email
    }

    assert_eq!(get_email(user), "alice@example.com");
}
