#![feature(libc)]
#![feature(cstr_to_str)]
#![feature(cstr_memory)]

extern crate libc;

use std::ffi::{CStr, CString};
#[no_mangle]
pub extern fn ruby_reverse(s: *const libc::c_char) -> *const libc::c_char {
    let rust_cstr = unsafe { CStr::from_ptr(s) };
    let str = rust_cstr.to_str().unwrap();
    let string: String = str.chars().rev().collect();
    let cstring = CString::new(string).unwrap();
    cstring.into_raw()
}

#[no_mangle]
pub extern fn concat(s1: *const libc::c_char, s2: *const libc::c_char) -> *const libc::c_char {
    let s1_and_str = ruby_string_to_ref(s1);
    let s2_and_str = ruby_string_to_ref(s2);
    let s1_string = s1_and_str.to_string();
    let string: String = s1_string + s2_and_str;
    string_to_ruby_string(string)
}

fn ruby_string_to_ref<'a>(r_string: *const libc::c_char) -> &'a str {
    unsafe { CStr::from_ptr(r_string) }.to_str().unwrap()
}

fn string_to_ruby_string(string: String) -> *const libc::c_char {
    CString::new(string).unwrap().into_raw()
}
