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
    let pair = Pair::<String, i32, _>::empty()
        .first("hello".to_owned())
        .second(42);

    assert_eq!(pair.first, "hello");
    assert_eq!(pair.second, 42);
}

#[test]
fn multiple_generics_partial() {
    let partial = Pair::<String, i32, _>::empty().first("world".to_owned());

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
    let bounded = Bounded::<String, _>::empty().value("test".to_owned());

    assert_eq!(bounded.value, "test");
}

// Test 3: Nested generics with extract
#[structural]
#[derive(Debug, PartialEq)]
struct Outer<T> {
    inner: T,
    label: String,
}

#[test]
fn nested_generic_extract() {
    #[structural]
    #[derive(Debug, PartialEq)]
    struct Address {
        city: String,
        country: String,
    }

    let full_address = Address::empty()
        .city("Tokyo".to_owned())
        .country("Japan".to_owned());

    let outer = Outer::<Address, _>::empty()
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
    let with_lifetime = WithLifetime::<'_, _>::empty().data(s.as_str());

    assert_eq!(with_lifetime.data, "hello");
}

// Test 5: Const generic (if edition 2024 supports it well)
#[structural]
#[derive(Debug)]
struct FixedArray<const N: usize> {
    data: [i32; N],
}

#[test]
fn const_generic() {
    let arr = FixedArray::<3, _>::empty().data([1, 2, 3]);

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
    let complex = Complex::<String, _>::empty().value("test".to_owned());

    assert_eq!(complex.value, "test");
}

// Test 7: Multiple generics with merge
#[test]
fn multiple_generics_merge() {
    let first_part = Pair::<String, i32, _>::empty().first("merged".to_owned());
    let second_part = Pair::<String, i32, _>::empty().second(99);

    let merged = first_part.merge(second_part);

    assert_eq!(merged.first, "merged");
    assert_eq!(merged.second, 99);
}

// Test 8: Generic with extract returning Remainder
#[test]
fn generic_extract_with_remainder() {
    let full = Pair::<String, i32, _>::empty()
        .first("hello".to_owned())
        .second(42);

    type FirstOnly = select!(pair: first);
    let (extracted, remainder): (Pair<String, i32, FirstOnly>, _) = full.extract();

    assert_eq!(extracted.first, "hello");
    assert_eq!(remainder.second, 42);
}
