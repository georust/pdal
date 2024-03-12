#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use crate::{size_t, PDALFullVersionString};
    use std::ffi::{c_char, CString};

    #[test]
    fn test_read_stuff() {
        const BUF_SIZE: usize = 1000;
        let mut buffer = [0 as c_char; BUF_SIZE];
        let data = unsafe {
            let len = PDALFullVersionString(buffer.as_mut_ptr(), BUF_SIZE as size_t);
            assert!((len as usize) < BUF_SIZE);
            let byte_slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len as usize);
            CString::new(byte_slice).unwrap()
        };

        assert!(!data.is_empty())
    }
}
