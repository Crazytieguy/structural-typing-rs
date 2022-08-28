trait Access: Sized {
    fn get<'a, T: FromTuple<Self> + SelfAccess + 'a>(&'a self) -> &T::Item {
        T::from_tuple(self).get()
    }
}

impl<T> Access for (T,) {}
impl<T, U> Access for (T, U) {}
impl<T, U, A> Access for (T, U, A) {}

trait SelfAccess {
    type Item;
    fn get(&self) -> &Self::Item;
}

trait FromTuple<T> {
    fn from_tuple(tuple: &T) -> &Self;
}

impl<T> FromTuple<(T,)> for T {
    fn from_tuple(tuple: &(T,)) -> &Self {
        &tuple.0
    }
}

struct Name(String);

impl SelfAccess for Name {
    type Item = String;
    fn get(&self) -> &Self::Item {
        &self.0
    }
}

auto trait IsNotName {}
impl !IsNotName for Name {}

impl<A: IsNotName> FromTuple<(Name, A)> for Name {
    fn from_tuple(tuple: &(Name, A)) -> &Self {
        &tuple.0
    }
}

impl<A: IsNotName> FromTuple<(A, Name)> for Name {
    fn from_tuple(tuple: &(A, Name)) -> &Self {
        &tuple.1
    }
}

struct Age(u8);

impl SelfAccess for Age {
    type Item = u8;
    fn get(&self) -> &Self::Item {
        &self.0
    }
}

auto trait IsNotAge {}
impl !IsNotAge for Age {}

impl<A: IsNotAge> FromTuple<(Age, A)> for Age {
    fn from_tuple(tuple: &(Age, A)) -> &Self {
        &tuple.0
    }
}

impl<A: IsNotAge> FromTuple<(A, Age)> for Age {
    fn from_tuple(tuple: &(A, Age)) -> &Self {
        &tuple.1
    }
}

struct Friends(Vec<(Age, Name)>);

impl SelfAccess for Friends {
    type Item = Vec<(Age, Name)>;
    fn get(&self) -> &Self::Item {
        &self.0
    }
}

auto trait IsNotFriends {}
impl !IsNotFriends for Friends {}

impl<A: IsNotFriends, B: IsNotFriends> FromTuple<(A, B, Friends)> for Friends {
    fn from_tuple(tuple: &(A, B, Friends)) -> &Self {
        &tuple.2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn getting() {
        let yoav = (Name("Yoav".into()), Age(26));
        assert_eq!(yoav.get::<Name>(), "Yoav");
        assert_eq!(yoav.get::<Age>(), &26);
        let baby = (Age(5), "Random");
        assert_eq!(baby.get::<Age>(), &5);
    }

    trait NameOps: Access
    where
        Name: FromTuple<Self>,
    {
        fn shout_name(&self) -> String {
            self.get::<Name>().to_uppercase()
        }
    }

    impl<T: Access> NameOps for T where Name: FromTuple<T> {}

    trait Person: Access
    where
        Name: FromTuple<Self>,
        Age: FromTuple<Self>,
    {
        fn is_wise(&self) -> bool {
            self.get::<Name>().len() < 5 && self.get::<Age>() > &15
        }
    }

    impl<T: Access> Person for T
    where
        Name: FromTuple<T>,
        Age: FromTuple<T>,
    {
    }

    #[test]
    fn methods() {
        let yoav = (Name("Yoav".into()), Age(26));
        assert_eq!(yoav.shout_name(), "YOAV");
        assert!(yoav.is_wise());
        let aur = (Name("Aur".into()),);
        assert_eq!(aur.shout_name(), "AUR");
    }

    #[test]
    fn nested() {
        let person = (
            Name("Yoav".into()),
            Age(32),
            Friends(vec![(Age(31), Name("Aur".into()))]),
        );
        assert!(person.get::<Friends>()[0].is_wise());
    }
}
