use crate::error::Result;
use std::ffi::{c_char, CString};

#[derive(Debug, Clone)]
pub struct PdalJson(CString);

impl PdalJson {
    pub(crate) fn new(spec: &str) -> Result<Self> {
        let cstr = CString::new(spec)?;
        Ok(Self(cstr))
    }
    pub(crate) fn as_ptr(&self) -> *const c_char {
        self.0.as_ptr()
    }
}

impl From<&str> for PdalJson {
    fn from(value: &str) -> Self {
        Self(CString::new(value).unwrap_or_default())
    }
}

impl From<String> for PdalJson {
    fn from(value: String) -> Self {
        Self(CString::new(value).unwrap_or_default())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::testkit::read_test_file;

    #[test]
    fn test_bad_pdal_json() {
        let bad = PdalJson::new("{\0foobar}".into());
        assert!(bad.is_err());
    }
    #[test]
    fn test_good_pdal_json() {
        let json = read_test_file("stats.json");
        let good = PdalJson::new(&json);
        assert!(good.is_ok(), "{:?}", good);
    }
}
