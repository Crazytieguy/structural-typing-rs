//! Conversions between different presence states.

use core::marker::PhantomData;

/// Trait for converting between different presence-wrapped types.
///
/// This trait allows converting from one presence state to another:
/// - `T` → `T` (identity)
/// - `T` → `Option<T>` (wrap in Some)
/// - `T` → `PhantomData<T>` (discard value)
/// - `Option<T>` → `PhantomData<T>` (discard value)
/// - `PhantomData<T>` → `Option<T>` (None)
/// - `PhantomData<T>` → `PhantomData<T>` (identity)
pub trait PresenceConvert<T, U> {
    /// Convert from one presence state to another.
    fn presence_convert(self) -> U;
}

impl<T> PresenceConvert<T, T> for T {
    fn presence_convert(self) -> T {
        self
    }
}

impl<T> PresenceConvert<T, Option<T>> for T {
    fn presence_convert(self) -> Option<T> {
        Some(self)
    }
}

impl<T> PresenceConvert<T, PhantomData<T>> for T {
    fn presence_convert(self) -> PhantomData<T> {
        PhantomData
    }
}

impl<T> PresenceConvert<T, Option<T>> for Option<T> {
    fn presence_convert(self) -> Option<T> {
        self
    }
}

impl<T> PresenceConvert<T, PhantomData<T>> for Option<T> {
    fn presence_convert(self) -> PhantomData<T> {
        PhantomData
    }
}

impl<T> PresenceConvert<T, Option<T>> for PhantomData<T> {
    fn presence_convert(self) -> Option<T> {
        None
    }
}

impl<T> PresenceConvert<T, PhantomData<T>> for PhantomData<T> {
    fn presence_convert(self) -> PhantomData<T> {
        PhantomData
    }
}
