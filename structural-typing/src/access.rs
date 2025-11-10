//! Runtime field access for structural types.
use core::marker::PhantomData;

use crate::presence::InferPresence;

/// Uniform field access for Present, Optional, or Absent.
pub trait Access<T>: InferPresence<T> {
    /// Compile-time constant indicating if this field is absent.
    const IS_ABSENT: bool;

    /// Remainder type when extracting Self from Source.
    type RemainderFrom<Source: Access<T>>: Access<T>;

    /// Get field reference if present.
    fn get(&self) -> Option<&T>;
    /// Get mutable field reference if present.
    fn get_mut(&mut self) -> Option<&mut T>;
    /// Consume and convert to Option.
    fn into_option(self) -> Option<T>;
}

/// Helper function to check if a field is absent (used by serde).
pub fn is_absent<A: Access<T>, T>(_value: &A) -> bool {
    A::IS_ABSENT
}

impl<T> Access<T> for PhantomData<T> {
    const IS_ABSENT: bool = true;

    type RemainderFrom<Source: Access<T>> = Source;

    #[inline]
    fn get(&self) -> Option<&T> {
        None
    }
    #[inline]
    fn get_mut(&mut self) -> Option<&mut T> {
        None
    }
    #[inline]
    fn into_option(self) -> Option<T> {
        None
    }
}

impl<T: Sized> Access<T> for T {
    const IS_ABSENT: bool = false;

    type RemainderFrom<Source: Access<T>> = PhantomData<T>;

    #[inline]
    fn get(&self) -> Option<&T> {
        Some(self)
    }
    #[inline]
    fn get_mut(&mut self) -> Option<&mut T> {
        Some(self)
    }
    #[inline]
    fn into_option(self) -> Option<T> {
        Some(self)
    }
}

impl<T> Access<T> for Option<T> {
    const IS_ABSENT: bool = false;

    type RemainderFrom<Source: Access<T>> = PhantomData<T>;

    #[inline]
    fn get(&self) -> Option<&T> {
        self.as_ref()
    }
    #[inline]
    fn get_mut(&mut self) -> Option<&mut T> {
        self.as_mut()
    }
    #[inline]
    fn into_option(self) -> Option<T> {
        self
    }
}
