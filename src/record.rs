use std::{fmt::Debug, intrinsics::type_id, marker::PhantomData, mem::forget};

pub auto trait True {}
pub struct IsNot<A, B>(PhantomData<(A, B)>);
impl<T> !True for IsNot<T, T> {}

pub trait Property: 'static {
    type Type;
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct P<T: Property>(pub T::Type);

impl<T: Property> Debug for P<T>
where
    T::Type: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

pub trait Record: Sized {
    type Top: Property;
    type Rest: Record;
    const IS_TERMINATOR: bool;
    fn _as_parts(&self) -> &(Self::Rest, P<Self::Top>);
    fn _as_mut_parts(&mut self) -> &mut (Self::Rest, P<Self::Top>);
    fn _into_parts(self) -> (Self::Rest, P<Self::Top>);
    fn _from_parts(rest: Self::Rest, top: P<Self::Top>) -> Self;
    fn insert<T: Property>(self, val: T::Type) -> (Self, P<T>) {
        (self, P(val))
    }
    fn insert_default<T: Property>(self, val: T::Type) -> (Self, P<T>) {
        if Self::IS_TERMINATOR {
            (self, P(val))
        } else if type_id::<T>() == type_id::<Self::Top>() {
            let (rest, top) = self._into_parts();
            let top_as_pt = unsafe { std::mem::transmute_copy::<P<Self::Top>, P<T>>(&top) };
            forget(top);
            let p_val = P(val);
            let p_val_as_p_top = unsafe { std::mem::transmute_copy::<P<T>, P<Self::Top>>(&p_val) };
            forget(p_val);
            let new_self = Self::_from_parts(rest, p_val_as_p_top);
            (new_self, top_as_pt)
        } else {
            let (rest, top) = self._into_parts();
            let (rest, p_val) = rest.insert_default(val);
            let new_self = Self::_from_parts(rest, top);
            (new_self, p_val)
        }
    }
    fn get_or<'a, T: Property>(&'a self, val: &'a T::Type) -> &'a T::Type {
        if Self::IS_TERMINATOR {
            val
        } else if type_id::<T>() == type_id::<Self::Top>() {
            let top = &self._as_parts().1 .0;
            let top_as_t = (top as *const <Self::Top as Property>::Type).cast::<T::Type>();
            unsafe { &*top_as_t }
        } else {
            self._as_parts().0.get_or::<T>(val)
        }
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

impl Property for () {
    type Type = ();
}

impl Record for () {
    type Rest = ();
    type Top = ();
    const IS_TERMINATOR: bool = true;
    fn _as_parts(&self) -> &(Self::Rest, P<Self::Top>) {
        unreachable!()
    }
    fn _as_mut_parts(&mut self) -> &mut (Self::Rest, P<Self::Top>) {
        unreachable!()
    }
    fn _into_parts(self) -> (Self::Rest, P<Self::Top>) {
        unreachable!()
    }
    fn _from_parts(_rest: Self::Rest, _top: P<Self::Top>) -> Self {
        unreachable!()
    }
}
impl<A: Record, B: Property> Record for (A, P<B>) {
    const IS_TERMINATOR: bool = false;
    type Rest = A;
    type Top = B;
    fn _as_parts(&self) -> &(Self::Rest, P<Self::Top>) {
        self
    }
    fn _as_mut_parts(&mut self) -> &mut (Self::Rest, P<Self::Top>) {
        self
    }
    fn _into_parts(self) -> (Self::Rest, P<Self::Top>) {
        self
    }
    fn _from_parts(rest: Self::Rest, top: P<Self::Top>) -> Self {
        (rest, top)
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

pub trait PartialTake<T>: Record {
    fn _partial_take(&mut self) -> T;
}

impl<T: Record> PartialTake<()> for T {
    fn _partial_take(&mut self) {}
}

impl<A, T, U> PartialTake<(T, P<A>)> for U
where
    A: Property,
    A::Type: Default,
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

    fn default_is_admin<T: Record>(val: T) -> (T, P<IsAdmin>) {
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

    fn generic<T: Has<Age> + Has<Name>>(inp: T) -> (String, u8, bool) {
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
