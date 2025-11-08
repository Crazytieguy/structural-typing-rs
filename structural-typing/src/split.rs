//! Split structs into selected fields and remainder.
//!
//! Use `split()` when extraction always succeeds (e.g., `Present` → `Optional`).
//! Use `try_split()` when extraction may fail (e.g., `Optional` → `Present` can fail if `None`).

use std::marker::PhantomData;

use crate::presence::{Absent, Optional, Present, Presence};

/// Trait for compile-time checked splitting from one presence to another.
///
/// Returns both the selected value (in target presence state) and the remainder.
pub trait Split<To: Presence>: Presence {
    /// Split a value into selected and remainder parts.
    fn split<T>(value: Self::Output<T>) -> SplitResult<To, Self, T>
    where
        Self: Sized;
}

/// Return type for [`Split::split`].
///
/// Returns `(selected, remainder)` where `selected` has the target presence type `To`,
/// and `remainder` contains the fields not selected.
pub type SplitResult<To, From, T> = (
    <To as Presence>::Output<T>,
    <<To as Presence>::RemainderFrom<From> as Presence>::Output<T>,
);

/// Trait for runtime checked splitting that may fail.
///
/// Returns Ok with (selected, remainder) on success, or Err with original value on failure.
/// Automatically succeeds when `Split` is implemented (blanket impl); only fails when `Optional` → `Present` with `None`.
pub trait TrySplit<To: Presence>: Presence {
    /// Try to split a value into selected and remainder parts.
    ///
    /// # Errors
    ///
    /// Returns `Err` with the original value if the split cannot be performed
    /// (e.g., when trying to convert `Optional` to `Present` but the value is `None`).
    fn try_split<T>(value: Self::Output<T>) -> Result<SplitResult<To, Self, T>, Self::Output<T>>
    where
        Self: Sized;
}

impl<From: Presence, To: Presence> TrySplit<To> for From
where
    From: Split<To>,
{
    fn try_split<T>(value: From::Output<T>) -> Result<SplitResult<To, From, T>, Self::Output<T>> {
        Ok(From::split(value))
    }
}

impl Split<Absent> for Present {
    fn split<T>(value: T) -> (PhantomData<T>, T) {
        (PhantomData, value)
    }
}

impl Split<Present> for Present {
    fn split<T>(value: T) -> (T, PhantomData<T>) {
        (value, PhantomData)
    }
}

impl Split<Optional> for Present {
    fn split<T>(value: T) -> (Option<T>, PhantomData<T>) {
        (Some(value), PhantomData)
    }
}

impl Split<Absent> for Optional {
    fn split<T>(value: Option<T>) -> (PhantomData<T>, Option<T>) {
        (PhantomData, value)
    }
}

impl Split<Optional> for Optional {
    fn split<T>(value: Option<T>) -> (Option<T>, PhantomData<T>) {
        (value, PhantomData)
    }
}

impl TrySplit<Present> for Optional {
    fn try_split<T>(value: Option<T>) -> Result<(T, PhantomData<T>), Option<T>> {
        match value {
            Some(v) => Ok((v, PhantomData)),
            None => Err(None),
        }
    }
}

impl Split<Absent> for Absent {
    fn split<T>(_value: PhantomData<T>) -> (PhantomData<T>, PhantomData<T>) {
        (PhantomData, PhantomData)
    }
}

impl Split<Optional> for Absent {
    fn split<T>(_value: PhantomData<T>) -> (Option<T>, PhantomData<T>) {
        (None, PhantomData)
    }
}
