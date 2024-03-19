#![allow(dead_code)]
#[cxx::bridge]
pub mod ffi {
    #[namespace = "pdal_sys"]   
    unsafe extern "C++" {
        include!("pdal-sys/src/options/options.hpp");
        type Options;
        fn new_options() -> UniquePtr<Options>;
        fn add(&self, name: &str, value: &str);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_new_options() {
        let mut o = super::ffi::new_options();
        o.add("foo", "bar");
    }
}