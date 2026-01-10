include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
//include!("../bindings.rs");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_test() {
        use std::ffi::CString;
        let my_string = "λόγος";
        let c_string = CString::new(my_string).unwrap();
        let ptr = c_string.into_raw();
        unsafe {
            let res = get_philolog(ptr);
            assert_eq!(res, 26);
            // After calling, retake ownership of the pointer and deallocate it
            let _ = CString::from_raw(ptr);
        }
    }
}
