use std::ffi::{CStr, CString, c_char};
use std::ptr;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
//include!("../bindings.rs");

pub fn morpheus_check(input: &str, morphlib_path: Option<&str>) -> Option<String> {
    let input_c_string = CString::new(input).unwrap();
    let input_ptr = input_c_string.into_raw();
    let flags = 0;

    let morph_lib_path_ptr = if let Some(morphlib_unwrapped) = morphlib_path {
        let morphlib_c_string = CString::new(morphlib_unwrapped).unwrap();
        morphlib_c_string.into_raw()
    } else {
        ptr::null_mut()
    };

    let result_owned_string;
    unsafe {
        let result_ptr: *mut c_char = philolog_morph(input_ptr, flags, morph_lib_path_ptr);
        if result_ptr.is_null() {
            return None;
        }
        let result_c_str = CStr::from_ptr(result_ptr);

        if let Ok(res_str) = result_c_str.to_str() {
            result_owned_string = res_str.to_owned();
            libc::free(result_ptr as *mut libc::c_void);
        } else {
            libc::free(result_ptr as *mut libc::c_void);
            return None;
        }
    }
    Some(result_owned_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_morpheus() {
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
        let my_string = "fe/rw";
        let morphlib_path = None; //or e.g.: Some("morpheus/dist/stemlib");
        let res = morpheus_check(my_string, morphlib_path);

        assert_eq!(
            res.unwrap(),
            String::from(
                r##"<words>
<word>
<form xml:lang="grc-x-beta">fe/rw</form>
<entry>
<dict>
<hdwd xml:lang="grc-x-beta">fe/rw</hdwd>
<pofs order="1">verb</pofs>
</dict>
<infl>
<term xml:lang="grc-x-beta"><stem>fer</stem><suff>w</suff></term>
<pofs order="1">verb</pofs>
<mood>subjunctive</mood>
<num>singular</num>
<pers>1st</pers>
<tense>present</tense>
<voice>active</voice>
<stemtype>w_stem</stemtype>
</infl>
<infl>
<term xml:lang="grc-x-beta"><stem>fer</stem><suff>w</suff></term>
<pofs order="1">verb</pofs>
<mood>indicative</mood>
<num>singular</num>
<pers>1st</pers>
<tense>present</tense>
<voice>active</voice>
<stemtype>w_stem</stemtype>
</infl>
</entry>
</word>
</words>
"##
            )
        );
    }

    #[test]
    fn check_word_empty() {
        let my_string = "";
        let morphlib_path = None; //or e.g.: Some("morpheus/dist/stemlib");
        let res = morpheus_check(my_string, morphlib_path);

        assert_eq!(res, None);
    }

    #[test]
    fn check_word_invalid() {
        let my_string = "xx";
        let morphlib_path = None; //or e.g.: Some("morpheus/dist/stemlib");
        let res = morpheus_check(my_string, morphlib_path);

        assert_eq!(
            res.unwrap(),
            String::from(
                r##"<words>
<unknown xml:lang="grc-x-beta">xx</unknown>
</words>
"##
            )
        );
    }

    #[test]
    fn check_word_multiple() {
        let my_string = "fe/reis a)/gomen";
        let morphlib_path = None; //or e.g.: Some("morpheus/dist/stemlib");
        let res = morpheus_check(my_string, morphlib_path);

        assert_eq!(
            res.unwrap(),
            String::from(
                r##"<words>
<word>
<form xml:lang="grc-x-beta">fe/reis</form>
<entry>
<dict>
<hdwd xml:lang="grc-x-beta">fe/rw</hdwd>
<pofs order="1">verb</pofs>
</dict>
<infl>
<term xml:lang="grc-x-beta"><stem>fer</stem><suff>eis</suff></term>
<pofs order="1">verb</pofs>
<mood>indicative</mood>
<num>singular</num>
<pers>2nd</pers>
<tense>present</tense>
<voice>active</voice>
<stemtype>w_stem</stemtype>
</infl>
</entry>
</word>
<word>
<form xml:lang="grc-x-beta">a)/gomen</form>
<entry>
<dict>
<hdwd xml:lang="grc-x-beta">a)/gw</hdwd>
<pofs order="1">verb</pofs>
</dict>
<infl>
<term xml:lang="grc-x-beta"><stem>a)g</stem><suff>omen</suff></term>
<pofs order="1">verb</pofs>
<mood>indicative</mood>
<num>plural</num>
<pers>1st</pers>
<tense>imperfect</tense>
<voice>active</voice>
<dial>Doric Aeolic</dial>
<stemtype>w_stem</stemtype>
<derivtype>reg_conj</derivtype>
</infl>
<infl>
<term xml:lang="grc-x-beta"><stem>a)g</stem><suff>omen</suff></term>
<pofs order="1">verb</pofs>
<mood>indicative</mood>
<num>plural</num>
<pers>1st</pers>
<tense>present</tense>
<voice>active</voice>
<stemtype>w_stem</stemtype>
<derivtype>reg_conj</derivtype>
</infl>
<infl>
<term xml:lang="grc-x-beta"><stem>a)g</stem><suff>omen</suff></term>
<pofs order="1">verb</pofs>
<mood>indicative</mood>
<num>plural</num>
<pers>1st</pers>
<tense>imperfect</tense>
<voice>active</voice>
<dial>Homeric Ionic</dial>
<stemtype>w_stem</stemtype>
<derivtype>reg_conj</derivtype>
<morph>unaugmented</morph>
</infl>
</entry>
</word>
</words>
"##
            )
        );
    }
}
