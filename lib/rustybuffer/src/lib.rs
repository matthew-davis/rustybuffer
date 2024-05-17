use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn hello(name: *const std::ffi::c_char) {
    let name_cstr = unsafe { CStr::from_ptr(name) };
    let name = name_cstr.to_str().unwrap();
    println!("Hello {}!", name);
}