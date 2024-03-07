use cxx::{CxxString, UniquePtr};
use std::ops::Deref;
use std::path::PathBuf;

/// Newtype for more generically converting between foreign types.
///
/// # Example
/// ```text, no_run
/// impl From<Conv<UniquePtr<CxxString>>> for String {
///     fn from(value: Conv<UniquePtr<CxxString>>) -> Self {
///         value.take().to_string()
///     }
/// }
/// ...
/// // Invoke conversion.
/// let cxx_string: UniquePtr<CxxString> = ...;
/// let string: String = Conv(cxx_string).into();
/// ```
pub(crate) struct Conv<T>(pub(crate) T);

impl<T> Conv<T> {
    pub fn inner(&self) -> &T {
        &self.0
    }
    pub fn take(self) -> T {
        self.0
    }
}

impl<T> Deref for Conv<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner()
    }
}

impl From<Conv<UniquePtr<CxxString>>> for String {
    fn from(value: Conv<UniquePtr<CxxString>>) -> Self {
        value.take().to_string()
    }
}

impl From<Conv<UniquePtr<CxxString>>> for PathBuf {
    fn from(value: Conv<UniquePtr<CxxString>>) -> Self {
        value.take().to_string().into()
    }
}
