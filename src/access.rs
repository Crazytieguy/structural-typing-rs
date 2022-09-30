use std::marker::PhantomData;

pub trait Access<T> {
    type OptionOrSelf: Access<T>;
    type Or<U: Access<T>>: Access<T>;
    fn get(&self) -> Option<&T>;
    fn get_mut(&mut self) -> Option<&mut T>;
    fn remove(self) -> Option<T>;
    fn or<U: Access<T>>(self, other: U) -> Self::Or<U>;
    fn or_else<U: Access<T>>(self, f: impl FnOnce() -> U) -> Self::Or<U>;
    fn _option_or_self(self, option: Option<T>) -> Self::OptionOrSelf;
    fn _option_or_else_self(f: impl FnOnce() -> Self, option: Option<T>) -> Self::OptionOrSelf;
}

impl<T> Access<T> for PhantomData<T> {
    type OptionOrSelf = Option<T>;
    type Or<U: Access<T>> = U;
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
    fn or<U: Access<T>>(self, other: U) -> Self::Or<U> {
        other
    }
    fn or_else<U: Access<T>>(self, f: impl FnOnce() -> U) -> Self::Or<U> {
        f()
    }
    fn _option_or_self(self, option: Option<T>) -> Self::OptionOrSelf {
        option
    }
    fn _option_or_else_self(_f: impl FnOnce() -> Self, option: Option<T>) -> Self::OptionOrSelf {
        option
    }
}

impl<T: Sized> Access<T> for T {
    type OptionOrSelf = Self;
    type Or<U: Access<T>> = T;
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
    fn or<U: Access<T>>(self, _other: U) -> Self::Or<U> {
        self
    }
    fn or_else<U: Access<T>>(self, _f: impl FnOnce() -> U) -> Self::Or<U> {
        self
    }
    fn _option_or_self(self, option: Option<T>) -> Self::OptionOrSelf {
        option.unwrap_or(self)
    }
    fn _option_or_else_self(f: impl FnOnce() -> Self, option: Option<T>) -> Self::OptionOrSelf {
        option.unwrap_or_else(f)
    }
}

impl<T> Access<T> for Option<T> {
    type OptionOrSelf = Self;
    type Or<U: Access<T>> = U::OptionOrSelf;
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
    fn or<U: Access<T>>(self, other: U) -> Self::Or<U> {
        other._option_or_self(self)
    }
    fn or_else<U: Access<T>>(self, f: impl FnOnce() -> U) -> Self::Or<U> {
        U::_option_or_else_self(f, self)
    }
    fn _option_or_self(self, option: Option<T>) -> Self::OptionOrSelf {
        option.or(self)
    }
    fn _option_or_else_self(f: impl FnOnce() -> Self, option: Option<T>) -> Self::OptionOrSelf {
        option.or_else(f)
    }
}
