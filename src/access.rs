use std::marker::PhantomData;

pub trait Access<T> {
    fn get(&self) -> Option<&T>;
    fn get_mut(&mut self) -> Option<&mut T>;
    fn remove(self) -> Option<T>;
}

impl<T> Access<T> for PhantomData<T> {
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
