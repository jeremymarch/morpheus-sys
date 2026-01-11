include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
//include!("../bindings.rs");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_morpheus() {
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

    #[test]
    fn check_word() {
        use std::ffi::{CStr, CString, c_char};
        use std::ptr;
        let my_string = "fe/rw";
        let c_string = CString::new(my_string).unwrap();
        let ptr = c_string.into_raw();
        //let mut null_ptr: *mut c_char = ptr::null();
        unsafe {
            let raw_ptr: *mut c_char = philolog_morph(ptr, 0, ptr::null_mut());
            assert!(!raw_ptr.is_null());
            let c_str = CStr::from_ptr(raw_ptr);

            let owned_string = c_str.to_str().unwrap().to_owned();

            assert_eq!(
                owned_string,
                String::from(
                    "<word>\n<form xml:lang=\"grc-x-beta\">fe/rw</form>\n<entry>\n<dict>\n<hdwd xml:lang=\"grc-x-beta\">fe/rw</hdwd>\n<pofs order=\"1\">verb</pofs>\n</dict>\n<infl>\n<term xml:lang=\"grc-x-beta\"><stem>fer</stem><suff>w</suff></term>\n<pofs order=\"1\">verb</pofs>\n<mood>subjunctive</mood>\n<num>singular</num>\n<pers>1st</pers>\n<tense>present</tense>\n<voice>active</voice>\n<stemtype>w_stem</stemtype>\n</infl>\n<infl>\n<term xml:lang=\"grc-x-beta\"><stem>fer</stem><suff>w</suff></term>\n<pofs order=\"1\">verb</pofs>\n<mood>indicative</mood>\n<num>singular</num>\n<pers>1st</pers>\n<tense>present</tense>\n<voice>active</voice>\n<stemtype>w_stem</stemtype>\n</infl>\n</entry>\n</word>\n</words>\n"
                )
            );
            //free_cstring_in_c(raw_ptr as *mut c_void);
            // After calling, retake ownership of the pointer and deallocate it
            let _ = CString::from_raw(ptr);
        }
    }
}
