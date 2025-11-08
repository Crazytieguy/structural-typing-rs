//! Runtime field access for structural types.
use std::marker::PhantomData;

/// Trait for uniformly accessing fields that may be Present, Optional, or Absent.
pub trait Access<T> {
    /// Compile-time constant indicating if this field is absent.
    const IS_ABSENT: bool;

    /// Get a reference to the field value if present.
    fn get(&self) -> Option<&T>;
    /// Get a mutable reference to the field value if present.
    fn get_mut(&mut self) -> Option<&mut T>;
}

/// Helper function to check if a field is absent (used by serde).
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
}
