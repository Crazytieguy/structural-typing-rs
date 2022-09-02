use std::{fmt::Debug, marker::PhantomData};

pub auto trait True {}
pub struct IsNot<A, B>(PhantomData<(A, B)>);
impl<T> !True for IsNot<T, T> {}

pub trait Property {
    type Type;
}

pub struct P<T: Property>(pub T::Type);
impl<T: Property> Clone for P<T>
where
    T::Type: Clone,
{
    fn clone(&self) -> Self {
        P(self.0.clone())
    }
}

impl<T: Property> Copy for P<T> where T::Type: Copy {}

impl<T: Property> Default for P<T>
where
    T::Type: Default,
{
    fn default() -> Self {
        P(Default::default())
    }
}

impl<T: Property> Debug for P<T>
where
    T::Type: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Property for () {
    type Type = ();
}

pub trait Record: Sized {
    fn insert<T: Property>(self, val: T::Type) -> (Self, P<T>) {
        (self, P(val))
    }
    fn insert_default<T: Property>(self, val: T::Type) -> (Self, P<T>)
    where
        Self: WithDefault<T>,
    {
        self._insert_default(val)
    }
    fn get_or<'a, T: Property>(&'a self, val: &'a T::Type) -> &'a T::Type
    where
        Self: WithDefault<T>,
    {
        self._get_or(val)
    }
    fn get<A: Property>(&self) -> &A::Type
    where
        Self: Has<A>,
    {
        Has::<A>::_get(self)
    }
    fn get_mut<A: Property>(&mut self) -> &mut A::Type
    where
        Self: Has<A>,
    {
        Has::<A>::_get_mut(self)
    }
    fn partial_take<T: Record>(&mut self) -> T
    where
        Self: PartialTake<T>,
    {
        self._partial_take()
    }
    fn remove<A: Property>(self) -> (<Self as Has<A>>::Rem, A::Type)
    where
        Self: Has<A>,
    {
        Has::<A>::_remove(self)
    }
}

impl Record for () {}
impl<A: Record, B: Property> Record for (A, P<B>) {}

pub trait WithDefault<T: Property>: Record {
    fn _insert_default(self, val: T::Type) -> (Self, P<T>);
    fn _get_or<'a>(&'a self, val: &'a T::Type) -> &'a T::Type;
}

impl<A: Property> WithDefault<A> for () {
    fn _insert_default(self, val: A::Type) -> (Self, P<A>) {
        (self, P(val))
    }
    fn _get_or<'a>(&'a self, val: &'a A::Type) -> &'a A::Type {
        val
    }
}

impl<T, A, B> WithDefault<A> for (T, P<B>)
where
    T: WithDefault<A>,
    A: Property,
    B: Property,
    IsNot<A, ()>: True,
    IsNot<A, B>: True,
{
    fn _insert_default(self, val: A::Type) -> (Self, P<A>) {
        let (inner_self, a) = self.0._insert_default(val);
        ((inner_self, self.1), a)
    }
    fn _get_or<'a>(&'a self, val: &'a A::Type) -> &'a A::Type {
        self.0._get_or(val)
    }
}

impl<T, A> WithDefault<A> for (T, P<A>)
where
    T: Record,
    A: Property,
{
    fn _insert_default(self, val: <A as Property>::Type) -> (Self, P<A>) {
        ((self.0, P(val)), self.1)
    }
    fn _get_or<'a>(&'a self, _val: &'a <A as Property>::Type) -> &'a <A as Property>::Type {
        &self.1 .0
    }
}

pub trait Has<T: Property>: Record {
    type Rem: Record;
    fn _get(&self) -> &T::Type;
    fn _get_mut(&mut self) -> &mut T::Type;
    fn _remove(self) -> (Self::Rem, T::Type);
}

impl<A, B> Has<B> for (A, P<B>)
where
    B: Property,
    A: Record,
{
    type Rem = A;
    fn _get(&self) -> &B::Type {
        &self.1 .0
    }
    fn _get_mut(&mut self) -> &mut B::Type {
        &mut self.1 .0
    }
    fn _remove(self) -> (Self::Rem, B::Type) {
        (self.0, self.1 .0)
    }
}

impl<A, T, B> Has<A> for (T, P<B>)
where
    A: Property,
    B: Property,
    T: Has<A>,
    IsNot<A, B>: True,
{
    type Rem = (T::Rem, P<B>);
    fn _get(&self) -> &A::Type {
        self.0._get()
    }
    fn _get_mut(&mut self) -> &mut A::Type {
        self.0._get_mut()
    }
    fn _remove(self) -> (Self::Rem, A::Type) {
        let (rest, a) = self.0._remove();
        ((rest, self.1), a)
    }
}

pub trait PartialTake<T: Record>: Record {
    fn _partial_take(&mut self) -> T;
}

impl<T: Record> PartialTake<()> for T {
    fn _partial_take(&mut self) {}
}

impl<A, T, U> PartialTake<(T, P<A>)> for U
where
    A: Property,
    A::Type: Default,
    T: Record,
    U: Has<A> + PartialTake<T>,
{
    fn _partial_take(&mut self) -> (T, P<A>) {
        (
            self._partial_take(),
            P(std::mem::take(Has::<A>::_get_mut(self))),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Name;

    impl Property for Name {
        type Type = String;
    }

    struct Age;
    impl Property for Age {
        type Type = u8;
    }

    struct IsAdmin;
    impl Property for IsAdmin {
        type Type = bool;
    }

    struct Height;
    impl Property for Height {
        type Type = f64;
    }

    #[test]
    fn concrete_has() {
        let record = ().insert::<Age>(15).insert::<Name>("Hi".into()).insert::<IsAdmin>(true);
        let (record, age) = record.remove::<Age>();
        assert_eq!(age, 15);
        assert!(record.get::<IsAdmin>());
        let (record, is_admin) = record.remove::<IsAdmin>();
        let ((), name) = record.remove();
        assert_eq!(name, "Hi");
        assert!(is_admin);
        assert_eq!(age, 15);
    }

    fn default_is_admin<T: WithDefault<IsAdmin>>(val: T) -> (T, P<IsAdmin>) {
        let val = val.insert_default::<IsAdmin>(false);
        val.get::<IsAdmin>();
        val
    }

    #[test]
    fn test_insert_default() {
        let yes_admin = default_is_admin(().insert::<Name>("Yoav".into()).insert::<IsAdmin>(true));
        assert!(yes_admin.get::<IsAdmin>());
        let not_admin = default_is_admin(().insert::<Name>("Yoav".into()));
        assert!(!not_admin.get::<IsAdmin>());
    }

    #[test]
    fn concrete_partial_take() {
        let mut t =
            ().insert::<IsAdmin>(true)
                .insert::<Age>(5)
                .insert::<Name>("Hi".into())
                .insert::<Height>(1.78);

        let (((((), P(height)), P(name)), P(age)), P(is_admin)) =
            t.partial_take::<(((((), P<Height>), P<Name>), P<Age>), P<IsAdmin>)>();
        assert_eq!(age, 5);
        assert_eq!(name, "Hi");
        assert!(is_admin);
        assert!((height - 1.78).abs() < f64::EPSILON);
    }

    fn generic<T: Has<Age> + Has<Name> + WithDefault<IsAdmin>>(inp: T) -> (String, u8, bool) {
        inp.get::<Age>();
        let mut inp = inp.insert_default::<IsAdmin>(false);
        let ((((), P(name)), P(age)), P(is_admin)) =
            inp.partial_take::<((((), P<Name>), P<Age>), P<IsAdmin>)>();
        (name, age, is_admin)
    }

    #[test]
    fn test_generic() {
        let (name, age, is_admin) = generic(
            ().insert::<Age>(70)
                .insert::<IsAdmin>(true)
                .insert::<Name>("Bob".into()),
        );
        assert_eq!(name, "Bob");
        assert_eq!(age, 70);
        assert!(is_admin);
    }
}
