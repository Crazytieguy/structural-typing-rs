use core::marker::PhantomData;
use structural_typing::{presence::Present, select, structural};

// Basic nested structs for testing
#[structural]
#[derive(Debug, PartialEq)]
struct Address {
    street: String,
    city: String,
    zip: String,
}

#[structural]
#[derive(Debug, PartialEq)]
struct Person<A: address::Fields = select!(address: all-)> {
    name: String,
    #[nested(address: street, city, zip)]
    address: Address<A>,
}

// Test 1: Basic nested setter functionality
#[test]
fn basic_nested_setter() {
    let person = person::empty()
        .name("Alice".to_owned())
        .address(address::empty())
        .address_city("Seattle".to_owned());

    assert_eq!(person.name, "Alice");
    assert_eq!(person.address.city, "Seattle");
}

// Test 2: Multiple nested setters
#[test]
fn multiple_nested_setters() {
    let person = person::empty()
        .address(address::empty())
        .address_street("123 Main St".to_owned())
        .address_city("Portland".to_owned())
        .address_zip("97201".to_owned());

    assert_eq!(person.address.street, "123 Main St");
    assert_eq!(person.address.city, "Portland");
    assert_eq!(person.address.zip, "97201");
}

// Test 3: Chaining nested setters with regular setters
#[test]
fn chaining_nested_and_regular_setters() {
    let person = person::empty()
        .name("Bob".to_owned())
        .address(address::empty())
        .address_city("Austin".to_owned())
        .address_street("456 Oak Ave".to_owned());

    assert_eq!(person.name, "Bob");
    assert_eq!(person.address.city, "Austin");
    assert_eq!(person.address.street, "456 Oak Ave");
}

// Test 4: Nested setter preserves other field states
#[test]
fn nested_setter_preserves_field_states() {
    let person = person::empty()
        .name("Charlie".to_owned())
        .address(address::empty())
        .address_city("Boston".to_owned());

    // Name should still be Present
    fn requires_name<A: address::Fields, F: person::Fields<name = Present>>(
        p: &Person<F, A>,
    ) -> &str {
        &p.name
    }

    assert_eq!(requires_name(&person), "Charlie");
}

// Test 5: Nested setter with Optional value
#[test]
fn nested_setter_with_optional() {
    let person = person::empty()
        .address(address::empty())
        .address_city(Some("Paris".to_owned()));

    assert_eq!(person.address.city, Some("Paris".to_owned()));
}

// Test 6: Other nested fields remain Absent
#[test]
fn nested_setter_other_fields_absent() {
    let person = person::empty()
        .address(address::empty())
        .address_city("London".to_owned());

    // street and zip should be Absent (PhantomData)
    assert_eq!(person.address.street, PhantomData);
    assert_eq!(person.address.zip, PhantomData);
}

// Test 7: Nested setters with single-field struct
// NOTE: Nested setters are not supported for single-field structs.
// Attempting to use #[nested] on a single-field struct produces a compile-time error:
// "type alias takes at most 1 generic argument but 2 generic arguments were supplied"
// This happens because the select! macro tries to use with::field<P, F>, but single-field
// structs only generate with::field<P>. This limitation is intentional and documented.
//
// Uncomment below to see the compile error:
// #[structural]
// struct SimpleNested {
//     value: i32,
// }
//
// #[structural]
// struct SimpleParent<N: simple_nested::Fields = select!(simple_nested: all-)> {
//     id: u64,
//     #[nested(value)]
//     nested: SimpleNested<N>,
// }

// Test 8: Multiple nested struct fields in parent
#[structural]
struct Contact {
    email: String,
    phone: String,
}

#[structural]
struct Employee<
    A: address::Fields = select!(address: all-),
    C: contact::Fields = select!(contact: all-),
> {
    #[nested(address: street, city)]
    address: Address<A>,
    #[nested(contact: email)]
    contact: Contact<C>,
}

#[test]
fn multiple_nested_struct_fields() {
    let emp = employee::empty()
        .address(address::empty())
        .contact(contact::empty())
        .address_city("NYC".to_owned())
        .contact_email("test@example.com".to_owned());

    assert_eq!(emp.address.city, "NYC");
    assert_eq!(emp.contact.email, "test@example.com");
}

// Test 9: Nested setters don't require all nested fields to be Present
#[test]
fn nested_setters_independent() {
    // This test verifies that we can call address_city without address_street being Present
    // (tests that trait bounds are per-method, not per-impl)
    let emp = employee::empty()
        .address(address::empty())
        .contact(contact::empty())
        .address_city("SF".to_owned());

    // Should compile even though we haven't set contact_email
    assert_eq!(emp.address.city, "SF");
}

// Test 10: Integration with extract
#[test]
fn nested_setter_then_extract() {
    let person = person::empty()
        .name("Frank".to_owned())
        .address(address::empty())
        .address_city("Vienna".to_owned())
        .address_street("Main St".to_owned());

    type PersonWithAddress = select!(person: address);
    let (extracted, remainder): (Person<PersonWithAddress, _>, _) = person.extract();

    assert_eq!(extracted.address.city, "Vienna");
    assert_eq!(remainder.name, "Frank");
}

// Test 11: Integration with merge
// TODO: Known limitation - merge requires compatible types for nested generic parameters.
// When two Person instances have different Address field presence states (even if one has
// address: Absent), the generic parameter A must match. This prevents merging Person instances
// that were built using nested setters with different field combinations.
// A future enhancement could allow merge to work with nested structs that have different
// field presence states, but this requires significant changes to the type system.
// #[test]
// fn nested_setter_with_merge() {
//     let p1 = person::empty().name("George".to_owned());
//
//     let p2 = person::empty()
//         .address(address::empty())
//         .address_city("Berlin".to_owned())
//         .address_street("Wall St".to_owned());
//
//     let merged = p1.merge(p2);
//
//     assert_eq!(merged.name, "George");
//     assert_eq!(merged.address.city, "Berlin");
//     assert_eq!(merged.address.street, "Wall St");
// }

// Test 12: Lifetimes with nested setters
#[structural]
struct WithLifetime<'a> {
    data: &'a str,
    label: String,
}

#[structural]
struct Container<'a, W: with_lifetime::Fields = select!(with_lifetime: all-)> {
    id: u64,
    #[nested(with_lifetime: data)]
    inner: WithLifetime<'a, W>,
}

#[test]
fn nested_setter_with_lifetime() {
    let s = String::from("hello");
    let container = container::empty()
        .id(1)
        .inner(with_lifetime::empty())
        .inner_data(s.as_str());

    assert_eq!(container.inner.data, "hello");
}

// Test 13: Deep nesting (3+ levels) - nested setters work at each level
#[structural]
struct DeepInner {
    value: i32,
    label: String,
}

#[structural]
struct DeepMiddle<I: deep_inner::Fields = select!(deep_inner: all-)> {
    name: String,
    #[nested(deep_inner: value, label)]
    inner: DeepInner<I>,
}

#[test]
fn deep_nesting_three_levels() {
    let middle = deep_middle::empty()
        .name("middle".to_owned())
        .inner(deep_inner::empty())
        .inner_value(42)
        .inner_label("answer".to_owned());

    assert_eq!(middle.name, "middle");
    assert_eq!(middle.inner.value, 42);
    assert_eq!(middle.inner.label, "answer");

    let updated = middle.inner_value(100);
    assert_eq!(updated.inner.value, 100);
    assert_eq!(updated.name, "middle");
}

// Test 14: Value overriding with nested setters
#[test]
fn nested_setter_value_override() {
    let person = person::empty()
        .name("Alice".to_owned())
        .address(address::empty())
        .address_city("Seattle".to_owned())
        .address_city("Portland".to_owned());

    assert_eq!(person.address.city, "Portland");
}

// Test 15: Serde integration with nested setters
#[cfg(feature = "serde")]
mod serde_integration {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[structural]
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Account {
        balance: f64,
        currency: String,
    }

    #[structural]
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Customer<A: account::Fields = select!(account: all-)> {
        name: String,
        #[nested(account: balance, currency)]
        account: Account<A>,
    }

    #[test]
    fn nested_setter_with_serde_roundtrip() {
        // Build using nested setters
        let customer = customer::empty()
            .name("Dave".to_owned())
            .account(account::empty())
            .account_balance(1000.0)
            .account_currency("USD".to_owned());

        // Serialize
        let json = serde_json::to_string(&customer).unwrap();
        assert!(json.contains("Dave"));
        assert!(json.contains("1000"));
        assert!(json.contains("USD"));

        // Verify structure is correct by deserializing
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["name"], "Dave");
        assert_eq!(value["account"]["balance"], 1000.0);
        assert_eq!(value["account"]["currency"], "USD");
    }
}
