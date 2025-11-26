//! Serde integration helpers.

use core::fmt;

/// Error returned when deserializing a structural type fails due to a missing required field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissingFieldError {
    field: &'static str,
}

impl MissingFieldError {
    /// Create a new error for a missing field.
    #[must_use]
    pub const fn new(field: &'static str) -> Self {
        Self { field }
    }

    /// Get the name of the missing field.
    #[must_use]
    pub const fn field(&self) -> &'static str {
        self.field
    }
}

impl fmt::Display for MissingFieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "missing field `{}`", self.field)
    }
}
