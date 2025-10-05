use std::marker::PhantomData;

/// Trait for uniform access to fields regardless of their presence state.
///
/// This trait allows you to access fields using `.get()`, `.get_mut()`, and `.remove()`
/// methods, which return `Option<T>` regardless of whether the field is known to be
/// present, absent, or optionally present at compile-time.
///
/// You must explicitly import this trait to use these methods:
/// ```rust
/// use structural_typing::Access;
/// ```
pub trait Access<T> {
    /// Returns a reference to the value if present.
    fn get(&self) -> Option<&T>;

    /// Returns a mutable reference to the value if present.
    fn get_mut(&mut self) -> Option<&mut T>;

    /// Consumes self and returns the value if present.
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
