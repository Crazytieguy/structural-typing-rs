use structural_typing::{select, structural};

// Test 1: Multiple generic parameters
#[structural]
#[derive(Debug, PartialEq)]
struct Pair<T, U> {
    first: T,
    second: U,
}

#[test]
fn multiple_generics_basic() {
    let pair = pair::empty::<String, i32>()
        .first("hello".to_owned())
        .second(42);

    assert_eq!(pair.first, "hello");
    assert_eq!(pair.second, 42);
}

#[test]
fn multiple_generics_partial() {
    let partial = pair::empty::<String, i32>().first("world".to_owned());

    assert_eq!(partial.first, "world");
}

// Test 2: Generic bounds
#[structural]
#[derive(Debug)]
struct Bounded<T> {
    value: T,
}

#[test]
fn generic_with_bounds() {
    let bounded = bounded::empty::<String>().value("test".to_owned());

    assert_eq!(bounded.value, "test");
}

// Test 3: Nested generics with extract
#[structural]
#[derive(Debug, PartialEq)]
struct Outer<T> {
    inner: T,
    label: String,
}

#[structural]
#[derive(Debug, PartialEq)]
struct Address {
    city: String,
    country: String,
}

#[test]
fn nested_generic_extract() {
    type FullAddress = select!(address: city, country);
    let full_address: Address<FullAddress> = address::empty()
        .city("Tokyo".to_owned())
        .country("Japan".to_owned());

    let outer = outer::empty::<Address<FullAddress>>()
        .inner(full_address)
        .label("location".to_owned());

    assert_eq!(outer.inner.city, "Tokyo");
    assert_eq!(outer.label, "location");
}

// Test 4: Generic with lifetime
#[structural]
#[derive(Debug)]
struct WithLifetime<'a> {
    data: &'a str,
}

#[test]
fn generic_lifetime() {
    let s = String::from("hello");
    let with_lifetime = with_lifetime::empty().data(s.as_str());

    assert_eq!(with_lifetime.data, "hello");
}

// Test 5: Const generic
#[structural]
#[derive(Debug)]
struct FixedArray<const N: usize> {
    data: [i32; N],
}

#[test]
fn const_generic() {
    let arr = fixed_array::empty::<3>().data([1, 2, 3]);

    assert_eq!(arr.data, [1, 2, 3]);
}

// Test 6: Complex where clause
#[structural]
#[derive(Debug)]
struct Complex<T>
where
    T: Clone + std::fmt::Debug,
{
    value: T,
}

#[test]
fn complex_where_clause() {
    let complex = complex::empty::<String>().value("test".to_owned());

    assert_eq!(complex.value, "test");
}

// Test 7: Multiple generics with merge
#[test]
fn multiple_generics_merge() {
    let first_part = pair::empty::<String, i32>().first("merged".to_owned());
    let second_part = pair::empty::<String, i32>().second(99);

    let merged = first_part.merge(second_part);

    assert_eq!(merged.first, "merged");
    assert_eq!(merged.second, 99);
}

// Test 8: Generic with extract returning Remainder
#[test]
fn generic_extract_with_remainder() {
    let full = pair::empty::<String, i32>()
        .first("hello".to_owned())
        .second(42);

    type FirstOnly = select!(pair: first);
    let (extracted, remainder): (Pair<FirstOnly, String, i32>, _) = full.extract();

    assert_eq!(extracted.first, "hello");
    assert_eq!(remainder.second, 42);
}

// Test 9: Type-changing setter with nested generics
#[structural]
struct Item<T> {
    data: T,
}

#[structural]
struct Store<I: item::Fields, T> {
    item_field: Item<I, T>,
}

#[test]
fn type_changing_setter_nested() {
    // Start with Item<String>
    let item_str = item::empty::<String>().data("hello".to_owned());
    let store = store::empty::<item::with::all, String>().item_field(item_str);

    // Verify original type
    assert_eq!(store.item_field.data, "hello");

    // Change to Item<i32> via setter - this tests the flexible generic setter
    let item_int = item::empty::<i32>().data(42);
    let store = store.item_field(item_int);

    // Verify type changed
    assert_eq!(store.item_field.data, 42);
}
