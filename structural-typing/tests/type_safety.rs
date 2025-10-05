use structural_typing::{Present, structural};

#[structural]
struct Person {
    name: String,
    age: u8,
    height: f32,
}

impl<S> Person<S>
where
    S: person_state::State<Name = Present>,
{
    fn shout_name(&self) -> String {
        self.name.to_uppercase()
    }
}

impl<S> Person<S>
where
    S: person_state::State<Name = Present, Age = Present>,
{
    fn say_hello(&self) -> String {
        format!("Hi! I'm {} and I'm {} years old", self.name, self.age)
    }
}

#[test]
fn test_type_safety() {
    let john = Person::empty().name("John".into());

    assert_eq!(john.shout_name(), "JOHN");

    let john = john.age(26);

    john.shout_name();

    assert_eq!(john.say_hello(), "Hi! I'm John and I'm 26 years old");
}

#[test]
fn test_merge() {
    let josef = Person::empty().name("Josef".into());

    let josef = josef.merge(Person::empty().age(37));
    assert_eq!(josef.name, "Josef");
    assert_eq!(josef.age, 37);

    let josefine = josef.merge(Person::empty().name("Josefine".into()));
    assert_eq!(josefine.name, "Josefine");
    assert_eq!(josefine.age, 37);
}

#[test]
fn test_access_trait() {
    use structural_typing::Access;

    let person = Person::empty().name("Alice".into());

    assert_eq!(person.name.get(), Some(&"Alice".to_string()));
    assert_eq!(person.age.get(), Option::<&u8>::None);

    let person = person.age(30);

    assert_eq!(person.name.get(), Some(&"Alice".to_string()));
    assert_eq!(person.age.get(), Some(&30));
}

#[test]
fn test_require() {
    let person = Person::empty();

    let person_with_name = person.require_name();
    assert!(person_with_name.is_none());

    let person = Person::empty().name("Bob".into());
    let person_with_name = person.require_name();
    assert!(person_with_name.is_some());
}
