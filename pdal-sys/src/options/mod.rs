#![allow(dead_code)]
#[cxx::bridge]
pub mod ffi {
    #[namespace = "pdal_sys"]   
    unsafe extern "C++" {
        include!("pdal-sys/src/options/options.hpp");
        type Options;
        fn create_options() -> UniquePtr<Options>;
        fn add(self: Pin<&mut Options>, name: &str, value: &str);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_new_options() {
        let mut o = super::ffi::create_options();
        o.pin_mut().add("foo", "bar");
    }
}