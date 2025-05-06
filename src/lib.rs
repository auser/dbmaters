use std::ffi::{CStr, CString};
use std::os::raw::c_char;

pub mod error;
pub use error::*;

extern "C" {
    fn CreateAndMigrate(path: GoString) -> *const c_char;
}

#[repr(C)]
struct GoString {
    a: *const c_char,
    b: i64,
}

pub fn create_and_migrate(path: &str) -> Result<String, DbMatersError> {
    let c_path = CString::new(path).expect("Failed to convert path to CString");
    let ptr = c_path.as_ptr();

    let go_str = GoString {
        a: ptr,
        b: path.len() as i64,
    };

    let result = unsafe { CreateAndMigrate(go_str) };
    let c_str = unsafe { CStr::from_ptr(result) };

    let string = c_str
        .to_str()
        .expect("Failed to convert CStr to String from library");

    match string.is_empty() {
        true => Err(DbMatersError::new("Failed to create and migrate database")),
        false => Ok(string.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_migrate() {
        let path = "./test_db";
        let result = create_and_migrate(path);
        assert!(result.is_ok());
    }
}
