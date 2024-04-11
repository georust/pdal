// MIT License
//
// Copyright (c) 2024 NUVIEW, Inc. <simeon.fitch@nuview.space>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software
// and associated documentation files (the "Software"), to deal in the Software without restriction,
// including without limitation the rights to use, copy, modify, merge, publish, distribute,
// sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
// BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT
// OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use std::fmt::{Debug, Formatter};
use std::ops::Deref;

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

#[allow(dead_code)]
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
        self.inner()
    }
}

/// Newtype wrapper for debug rendering utility.
pub(crate) struct Elided<'a, T>(pub &'a T);

impl<T: Debug> Debug for Elided<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        let max_len = if f.alternate() { 80 } else { 15 };
        let mut buf = String::new();
        write!(buf, "{:?}", self.0)?;

        if buf.len() > max_len {
            let h = max_len / 2;
            write!(f, "{} \u{2026} {}", &buf[..h], &buf[buf.len() - h..])
        } else {
            write!(f, "{:?}", self.0)
        }
    }
}
