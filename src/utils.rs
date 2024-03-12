#![allow(unused)]
use crate::error::Result;
use pdal_sys::size_t;
use std::ffi::{c_char, c_int, CString};
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
