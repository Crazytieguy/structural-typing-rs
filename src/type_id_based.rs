use core::intrinsics::type_id;
use std::any::Any;
struct If<const COND: bool>;
trait True {}
impl True for If<true> {}

pub trait Has<const N: usize> {
    const TYPE_IDS: [u64; N];
}

const fn has<const N: usize, U: Has<N>, T: 'static>() -> bool {
    match N {
        1 => type_id::<T>() == U::TYPE_IDS[0],
        2 => type_id::<T>() == U::TYPE_IDS[0] || type_id::<T>() == U::TYPE_IDS[1],
        3 => {
            type_id::<T>() == U::TYPE_IDS[0]
                || type_id::<T>() == U::TYPE_IDS[1]
                || type_id::<T>() == U::TYPE_IDS[2]
        }
        4 => {
            type_id::<T>() == U::TYPE_IDS[0]
                || type_id::<T>() == U::TYPE_IDS[1]
                || type_id::<T>() == U::TYPE_IDS[2]
                || type_id::<T>() == U::TYPE_IDS[3]
        }
        _ => false,
    }
}

impl<A: 'static> const Has<1> for (A,) {
    const TYPE_IDS: [u64; 1] = [type_id::<A>()];
}

impl<A: 'static, B: 'static> const Has<2> for (A, B) {
    const TYPE_IDS: [u64; 2] = [type_id::<A>(), type_id::<B>()];
}

impl<A: 'static, B: 'static, C: 'static> const Has<3> for (A, B, C) {
    const TYPE_IDS: [u64; 3] = [type_id::<A>(), type_id::<B>(), type_id::<C>()];
}

impl<A: 'static, B: 'static, C: 'static, D: 'static> const Has<4> for (A, B, C, D) {
    const TYPE_IDS: [u64; 4] = [
        type_id::<A>(),
        type_id::<B>(),
        type_id::<C>(),
        type_id::<D>(),
    ];
}

pub trait SelfAccess: 'static {
    type Item;
    fn get(&self) -> &Self::Item;
}

pub trait AccessibleFrom<const N: usize, T> {}

impl<T: 'static, U: Has<N>, const N: usize> AccessibleFrom<N, U> for T where
    If<{ has::<N, U, T>() }>: True
{
}

pub trait Access<const N: usize>
where
    Self: Sized + Has<N> + 'static,
{
    fn get<T>(&self) -> &T::Item
    where
        T: SelfAccess + 'static + AccessibleFrom<N, Self>;
}

unsafe fn get_transmute<T: SelfAccess, U: SelfAccess>(val: &T) -> &U::Item {
    debug_assert_eq!(type_id::<T>(), type_id::<U>());
    &*(val.get() as *const dyn Any).cast::<U::Item>()
}

impl<A: 'static + SelfAccess> Access<1> for (A,) {
    fn get<T>(&self) -> &T::Item
    where
        T: SelfAccess + 'static + AccessibleFrom<1, Self>,
    {
        if type_id::<T>() == type_id::<A>() {
            unsafe { get_transmute::<A, T>(&self.0) }
        } else {
            unreachable!()
        }
    }
}

impl<A: 'static + SelfAccess, B: 'static + SelfAccess> Access<2> for (A, B) {
    fn get<T>(&self) -> &T::Item
    where
        T: SelfAccess + 'static + AccessibleFrom<2, Self>,
    {
        if type_id::<T>() == type_id::<A>() {
            unsafe { get_transmute::<A, T>(&self.0) }
        } else if type_id::<T>() == type_id::<B>() {
            unsafe { get_transmute::<B, T>(&self.1) }
        } else {
            unreachable!()
        }
    }
}

impl<A: 'static + SelfAccess, B: 'static + SelfAccess, C: 'static + SelfAccess> Access<3>
    for (A, B, C)
{
    fn get<T>(&self) -> &T::Item
    where
        T: SelfAccess + 'static + AccessibleFrom<3, Self>,
    {
        if type_id::<T>() == type_id::<A>() {
            unsafe { get_transmute::<A, T>(&self.0) }
        } else if type_id::<T>() == type_id::<B>() {
            unsafe { get_transmute::<B, T>(&self.1) }
        } else if type_id::<T>() == type_id::<C>() {
            unsafe { get_transmute::<C, T>(&self.2) }
        } else {
            unreachable!()
        }
    }
}

impl<
        A: 'static + SelfAccess,
        B: 'static + SelfAccess,
        C: 'static + SelfAccess,
        D: 'static + SelfAccess,
    > Access<4> for (A, B, C, D)
{
    fn get<T>(&self) -> &T::Item
    where
        T: SelfAccess + 'static + AccessibleFrom<4, Self>,
    {
        if type_id::<T>() == type_id::<A>() {
            unsafe { get_transmute::<A, T>(&self.0) }
        } else if type_id::<T>() == type_id::<B>() {
            unsafe { get_transmute::<B, T>(&self.1) }
        } else if type_id::<T>() == type_id::<C>() {
            unsafe { get_transmute::<C, T>(&self.2) }
        } else if type_id::<T>() == type_id::<D>() {
            unsafe { get_transmute::<D, T>(&self.3) }
        } else {
            unreachable!()
        }
    }
}

pub trait Insert<T, U> {
    fn insert(self, val: T) -> U;
}

impl<T> Insert<T, (T,)> for () {
    fn insert(self, val: T) -> (T,) {
        (val,)
    }
}

impl<A, T> Insert<T, (A, T)> for (A,) {
    fn insert(self, val: T) -> (A, T) {
        (self.0, val)
    }
}

impl<A, B, T> Insert<T, (A, B, T)> for (A, B) {
    fn insert(self, val: T) -> (A, B, T) {
        (self.0, self.1, val)
    }
}

impl<A, B, C, T> Insert<T, (A, B, C, T)> for (A, B, C) {
    fn insert(self, val: T) -> (A, B, C, T) {
        (self.0, self.1, self.2, val)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct Name(&'static str);
    struct Age(u8);
    struct Phone(&'static str);

    impl SelfAccess for Name {
        type Item = &'static str;
        fn get(&self) -> &Self::Item {
            &self.0
        }
    }

    impl SelfAccess for Age {
        type Item = u8;
        fn get(&self) -> &Self::Item {
            &self.0
        }
    }

    impl SelfAccess for Phone {
        type Item = &'static str;
        fn get(&self) -> &Self::Item {
            &self.0
        }
    }

    trait NameOps<const N: usize>: Has<N> + Sized + Access<N>
    where
        Name: AccessibleFrom<N, Self>,
    {
        fn shout_name(&self) -> String {
            self.get::<Name>().to_uppercase()
        }
    }

    impl<T: Has<N> + Sized + Access<N>, const N: usize> NameOps<N> for T where
        Name: AccessibleFrom<N, Self>
    {
    }

    trait PersonOps<const N: usize>: Has<N> + Sized + Access<N>
    where
        Name: AccessibleFrom<N, Self>,
        Age: AccessibleFrom<N, Self>,
    {
        fn is_wise(&self) -> bool {
            self.get::<Name>().len() < 5 && self.get::<Age>() > &15
        }
    }

    impl<T: Has<N> + Sized + Access<N>, const N: usize> PersonOps<N> for T
    where
        Name: AccessibleFrom<N, Self>,
        Age: AccessibleFrom<N, Self>,
    {
    }

    #[test]
    fn getting() {
        let yoav = (Name("Yoav"), Age(26));
        assert_eq!(*yoav.get::<Name>(), "Yoav");
        assert_eq!(yoav.get::<Age>(), &26);
        let yoav = (Age(26), Name("Yoav"));
        assert_eq!(*yoav.get::<Name>(), "Yoav");
        assert_eq!(yoav.get::<Age>(), &26);
        assert_eq!(yoav.shout_name(), "YOAV");
        assert!(yoav.is_wise());
        let yoav = ().insert(Name("Yoav"));
        assert_eq!(yoav.shout_name(), "YOAV");
        let yoav = yoav.insert(Age(18)).insert(Phone("053-821-6931"));
        assert!(yoav.is_wise());
    }
}
