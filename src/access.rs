use std::marker::PhantomData;

pub trait Access<T> {
    const IS_ABSENT: bool;

    fn get(&self) -> Option<&T>;
    fn get_mut(&mut self) -> Option<&mut T>;
    fn remove(self) -> Option<T>;
}

pub fn is_absent<A: Access<T>, T>(_value: &A) -> bool {
    A::IS_ABSENT
}

impl<T> Access<T> for PhantomData<T> {
    const IS_ABSENT: bool = true;

    #[inline]
    fn get(&self) -> Option<&T> {
        None
    }
    #[inline]
    fn get_mut(&mut self) -> Option<&mut T> {
        None
    }
    #[inline]
    fn remove(self) -> Option<T> {
        None
    }
}

impl<T: Sized> Access<T> for T {
    const IS_ABSENT: bool = false;

    #[inline]
    fn get(&self) -> Option<&T> {
        Some(self)
    }
    #[inline]
    fn get_mut(&mut self) -> Option<&mut T> {
        Some(self)
    }
    #[inline]
    fn remove(self) -> Option<T> {
        Some(self)
    }
}

impl<T> Access<T> for Option<T> {
    const IS_ABSENT: bool = false;

    #[inline]
    fn get(&self) -> Option<&T> {
        self.as_ref()
    }
    #[inline]
    fn get_mut(&mut self) -> Option<&mut T> {
        self.as_mut()
    }
    #[inline]
    fn remove(self) -> Option<T> {
        self
    }
}
