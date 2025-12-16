use std::ffi::{CString};
use std::fs;

///Fails if interior NULs exist inside the file since CStr/CString is NUL terminated
pub fn load_file_as_cstr(path: &str) -> Result<CString, Box<dyn std::error::Error>> {
    Ok(CString::new(fs::read(path)?)?)
}