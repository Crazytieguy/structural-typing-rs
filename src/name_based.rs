struct Name(String);

pub trait GetName {
    fn name(&self) -> &String;
}

auto trait IsNotName {}
impl !IsNotName for Name {}

impl<T: IsNotName> GetName for (Name, T) {
    fn name(&self) -> &String {
        &self.0 .0
    }
}

impl<T: IsNotName> GetName for (T, Name) {
    fn name(&self) -> &String {
        &self.1 .0
    }
}

struct Age(u8);

pub trait GetAge {
    fn age(&self) -> &u8;
}

auto trait IsNotAge {}
impl !IsNotAge for Age {}

impl<T: IsNotAge> GetAge for (Age, T) {
    fn age(&self) -> &u8 {
        &self.0 .0
    }
}

impl<T: IsNotAge> GetAge for (T, Age) {
    fn age(&self) -> &u8 {
        &self.1 .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn getting() {
        let t = (Name("Yoav".to_owned()), Age(26));
        assert_eq!(t.name(), "Yoav");
        assert_eq!(t.age(), &26);
        let t = (Age(5), "Random");
        assert_eq!(t.age(), &5);
    }

    trait Named: GetName {
        fn shout_name(&self) -> String {
            self.name().to_uppercase()
        }
    }

    impl<T: GetName> Named for T {}

    trait NameAndAge: GetName + GetAge {
        fn is_wise(&self) -> bool {
            self.name().len() > 3 && self.age() > &15
        }
    }

    impl<T: GetName + GetAge> NameAndAge for T {}

    #[test]
    fn interfaceing_works() {
        let yoav = (Name("Yoav".to_owned()), Age(26));
        assert_eq!(yoav.shout_name(), "YOAV");
        assert!(yoav.is_wise());
        let aur = (Name("Aur".to_owned()), "Some other property");
        assert_eq!(aur.shout_name(), "AUR");
    }
}
