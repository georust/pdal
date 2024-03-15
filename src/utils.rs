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

#![allow(unused)]
use crate::error::Result;
use pdal_sys::size_t;
use std::ffi::{c_char, c_int, CString};
use std::fmt::{Debug, Formatter};
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

impl TryFrom<Conv<CString>> for String {
    type Error = crate::error::Error;

    fn try_from(value: Conv<CString>) -> std::result::Result<Self, Self::Error> {
        Ok(value.to_str()?.into())
    }
}

impl TryFrom<Conv<CString>> for PathBuf {
    type Error = crate::error::Error;

    fn try_from(value: Conv<CString>) -> std::result::Result<Self, Self::Error> {
        Ok(value.to_str()?.into())
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

pub(crate) type CharBufFetch =
    unsafe extern "C" fn(buf: *mut ::std::ffi::c_char, size: size_t) -> size_t;

/// Fetch a string from a C function that writes to a buffer.
pub(crate) fn fetch_string_with_buffer<const BUF_SIZE: usize>(f: CharBufFetch) -> Result<CString> {
    let mut buffer = [0 as c_char; BUF_SIZE];
    let value = unsafe {
        let len = f(buffer.as_mut_ptr(), BUF_SIZE as size_t);
        if len as usize >= BUF_SIZE {
            return Err("buffer size too small".into());
        }
        let byte_slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len as usize);
        CString::new(byte_slice).unwrap()
    };
    Ok(value)
}

pub(crate) type Handle = *mut ::std::ffi::c_void;
pub(crate) type MemberCharBufFetch<H> =
    unsafe extern "C" fn(handle: H, *mut ::std::ffi::c_char, size: size_t) -> size_t;

/// Fetch a handle-associated string from a C function that writes to a buffer.
pub(crate) fn fetch_string_from_handle_with_buffer<const BUF_SIZE: usize, H>(
    handle: H,
    f: MemberCharBufFetch<H>,
) -> Result<CString> {
    let mut buffer = [0 as c_char; BUF_SIZE];
    let value = unsafe {
        let len = f(handle, buffer.as_mut_ptr(), BUF_SIZE as size_t);
        if len as usize >= BUF_SIZE {
            return Err("buffer size too small".into());
        }
        let byte_slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len as usize);
        CString::new(byte_slice).unwrap()
    };
    Ok(value)
}

pub(crate) type IntFetch = unsafe extern "C" fn() -> c_int;
pub(crate) fn fetch_int(f: IntFetch) -> Result<i32> {
    let value = unsafe { f() };
    Ok(value as i32)
}
