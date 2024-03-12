use std::ffi::{c_char, CString};

#[derive(Debug, Clone)]
pub struct PdalJson(CString);

impl PdalJson {
    pub(crate) fn new(spec: &str) -> Self {
        let cstr = CString::new(spec).unwrap_or_default();
        Self(cstr)
    }
    pub(crate) fn as_ptr(&self) -> *const c_char {
        self.0.as_ptr()
    }
}

impl From<&str> for PdalJson {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for PdalJson {
    fn from(value: String) -> Self {
        Self::new(&value)
    }
}
