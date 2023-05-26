
#![no_std]

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    todo!()//TODO
}

trait IntoStrRef {
    fn into_regular_str(&self) -> Result<&str, core::str::Utf8Error>;
}

impl IntoStrRef for *const core::ffi::c_char {
    fn into_regular_str(&self) -> Result<&str, core::str::Utf8Error> {
        //TODO what if the pointer is null?
        let c_str = unsafe { core::ffi::CStr::from_ptr(*self) };
        c_str.to_str()
    }
}

#[no_mangle]
pub extern "C" fn puts(the_str: *const core::ffi::c_char) -> core::ffi::c_int {
    let regular_str = the_str.into_regular_str().unwrap();//TODO proper except here
    //println!("{}", regular_str);//TODO does this work without std? (no)
    0
}

#[no_mangle]
pub extern "C" fn abs(n: core::ffi::c_int) -> core::ffi::c_int {
    n.abs()
}

//Cool, the compiler recognizes labs and llabs are the same on my linux system
//So both symbols go to the same spot in the binary!!!

#[no_mangle]
pub extern "C" fn labs(n: core::ffi::c_long) -> core::ffi::c_long {
    n.abs()
}

#[no_mangle]
pub extern "C" fn llabs(n: core::ffi::c_longlong) -> core::ffi::c_longlong {
    n.abs()
}

/*
#[no_mangle]
pub extern "C" fn imaxabs(n: core::ffi::c_intmax) -> core::ffi::c_intmax {
    n.abs()
}
*/

#[no_mangle]
pub extern "C" fn isalnum(ch: core::ffi::c_int) -> core::ffi::c_int {
    (ch as u8).is_ascii_alphanumeric() as core::ffi::c_int
}

#[no_mangle]
pub extern "C" fn isalpha(ch: core::ffi::c_int) -> core::ffi::c_int {
    (ch as u8).is_ascii_alphabetic() as core::ffi::c_int
}

#[no_mangle]
pub extern "C" fn islower(ch: core::ffi::c_int) -> core::ffi::c_int {
    (ch as u8).is_ascii_lowercase() as core::ffi::c_int
}

#[no_mangle]
pub extern "C" fn isupper(ch: core::ffi::c_int) -> core::ffi::c_int {
    (ch as u8).is_ascii_uppercase() as core::ffi::c_int
}

#[no_mangle]
pub extern "C" fn isdigit(ch: core::ffi::c_int) -> core::ffi::c_int {
    (ch as u8).is_ascii_digit() as core::ffi::c_int
}

#[no_mangle]
pub extern "C" fn isxdigit(ch: core::ffi::c_int) -> core::ffi::c_int {
    (ch as u8).is_ascii_hexdigit() as core::ffi::c_int
}

#[no_mangle]
pub extern "C" fn iscntrl(ch: core::ffi::c_int) -> core::ffi::c_int {
    (ch as u8).is_ascii_control() as core::ffi::c_int
}

#[no_mangle]
pub extern "C" fn isgraph(ch: core::ffi::c_int) -> core::ffi::c_int {
    todo!()
}

#[no_mangle]
pub extern "C" fn isspace(ch: core::ffi::c_int) -> core::ffi::c_int {
    todo!()
    //The below misses vertical tab
    //(ch as u8).is_ascii_whitespace() as core::ffi::c_int
}

#[no_mangle]
pub extern "C" fn isblank(ch: core::ffi::c_int) -> core::ffi::c_int {
    todo!()
}

#[no_mangle]
pub extern "C" fn isprint(ch: core::ffi::c_int) -> core::ffi::c_int {
    todo!()
}

#[no_mangle]
pub extern "C" fn ispunct(ch: core::ffi::c_int) -> core::ffi::c_int {
    (ch as u8).is_ascii_punctuation() as core::ffi::c_int
}

#[no_mangle]
pub extern "C" fn tolower(ch: core::ffi::c_int) -> core::ffi::c_int {
    (ch as u8).to_ascii_lowercase() as core::ffi::c_int
}

#[no_mangle]
pub extern "C" fn toupper(ch: core::ffi::c_int) -> core::ffi::c_int {
    (ch as u8).to_ascii_uppercase() as core::ffi::c_int
}

#[no_mangle]
pub extern "C" fn atof(the_str: *const core::ffi::c_char) -> core::ffi::c_double {
    let regular_str = the_str.into_regular_str().unwrap();//TODO proper except here
    regular_str.parse().unwrap_or_else(|_| 0.0)
}

#[no_mangle]
pub extern "C" fn atoi(the_str: *const core::ffi::c_char) -> core::ffi::c_int {
    let regular_str = the_str.into_regular_str().unwrap();//TODO proper except here
    regular_str.parse().unwrap_or_else(|_| 0)
}

#[no_mangle]
pub extern "C" fn atol(the_str: *const core::ffi::c_char) -> core::ffi::c_long {
    let regular_str = the_str.into_regular_str().unwrap();//TODO proper except here
    regular_str.parse().unwrap_or_else(|_| 0)
}

#[no_mangle]
pub extern "C" fn atoll(the_str: *const core::ffi::c_char) -> core::ffi::c_longlong {
    let regular_str = the_str.into_regular_str().unwrap();//TODO proper except here
    regular_str.parse().unwrap_or_else(|_| 0)
}

#[no_mangle]
pub extern "C" fn strtol(the_str: *const core::ffi::c_char, str_end: *const core::ffi::c_char, base: core::ffi::c_int) -> core::ffi::c_long {
    todo!()
    //TODO construct a str, then to c_long::from_str_radix()
}

#[no_mangle]
pub extern "C" fn strtoll(the_str: *const core::ffi::c_char, str_end: *const core::ffi::c_char, base: core::ffi::c_int) -> core::ffi::c_longlong {
    todo!()
    //TODO construct a str, then to c_longlong::from_str_radix()
}

#[no_mangle]
pub extern "C" fn strtoul(the_str: *const core::ffi::c_char, str_end: *const core::ffi::c_char, base: core::ffi::c_int) -> core::ffi::c_ulong {
    todo!()
    //TODO construct a str, then to c_ulong::from_str_radix()
}

#[no_mangle]
pub extern "C" fn strtoull(the_str: *const core::ffi::c_char, str_end: *const core::ffi::c_char, base: core::ffi::c_int) -> core::ffi::c_ulonglong {
    todo!()
    //TODO construct a str, then to c_ulonglong::from_str_radix()
}

#[no_mangle]
pub extern "C" fn strtof(the_str: *const core::ffi::c_char, str_end: *const core::ffi::c_char) -> core::ffi::c_float {
    todo!()
}

#[no_mangle]
pub extern "C" fn strtod(the_str: *const core::ffi::c_char, str_end: *const core::ffi::c_char) -> core::ffi::c_double {
    todo!()
}

#[no_mangle]
pub extern "C" fn strtold(the_str: *const core::ffi::c_char, str_end: *const core::ffi::c_char) -> core::ffi::c_longdouble {
    todo!()
}

//TODO number -> string functions

//TODO other string functions


