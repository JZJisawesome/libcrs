#![no_std]

//FIXME mark relevant functions as unsafe

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    todo!()//TODO
}

trait NiceIntoSlices {
    unsafe fn into_regular_str(&self) -> Result<&str, core::str::Utf8Error>;
    unsafe fn into_byte_slice(&self) -> &[u8];
}

impl NiceIntoSlices for *const core::ffi::c_char {
    unsafe fn into_regular_str(&self) -> Result<&str, core::str::Utf8Error> {
        debug_assert!(!self.is_null());
        let c_str = unsafe { core::ffi::CStr::from_ptr(*self) };
        c_str.to_str()
    }

    unsafe fn into_byte_slice(&self) -> &[u8] {
        debug_assert!(!self.is_null());
        let c_str = unsafe { core::ffi::CStr::from_ptr(*self) };
        c_str.to_bytes()
    }
}

#[no_mangle]
pub extern "C" fn puts(the_str: *const core::ffi::c_char) -> core::ffi::c_int {
    let regular_str = unsafe { the_str.into_regular_str() }.unwrap();//TODO proper except here
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

//FIXME change all of these to if...else statements

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

//TODO actually don't do an except, do or else "0" (or to be more efficient, do an if-let and skip parsing in the case the UTF8 is invalid)

#[no_mangle]
pub extern "C" fn atof(the_str: *const core::ffi::c_char) -> core::ffi::c_double {
    let regular_str = unsafe { the_str.into_regular_str() }.unwrap();//TODO proper except here
    regular_str.parse().unwrap_or_else(|_| 0.0)
}

#[no_mangle]
pub extern "C" fn atoi(the_str: *const core::ffi::c_char) -> core::ffi::c_int {
    let regular_str = unsafe { the_str.into_regular_str() }.unwrap();//TODO proper except here
    regular_str.parse().unwrap_or_else(|_| 0)
}

#[no_mangle]
pub extern "C" fn atol(the_str: *const core::ffi::c_char) -> core::ffi::c_long {
    let regular_str = unsafe { the_str.into_regular_str() }.unwrap();//TODO proper except here
    regular_str.parse().unwrap_or_else(|_| 0)
}

#[no_mangle]
pub extern "C" fn atoll(the_str: *const core::ffi::c_char) -> core::ffi::c_longlong {
    let regular_str = unsafe { the_str.into_regular_str() }.unwrap();//TODO proper except here
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

/*#[no_mangle]
pub extern "C" fn strtold(the_str: *const core::ffi::c_char, str_end: *const core::ffi::c_char) -> core::ffi::c_longdouble {
    todo!()
}*/

//TODO number -> string functions

//TODO other string functions

//FIXME we can't write strlen using anything in libcore since otherwise CStr and friends actually tries to call this function to use as its strlen implementation
#[no_mangle]
pub extern "C" fn strlen(mut the_str: *const core::ffi::c_char) -> core::ffi::c_ulong {
    debug_assert!(!the_str.is_null());
    
    //Note: We CANNOT use the following since it uses libcore, but libcore depends on strlen existing
    //let c_str = unsafe { core::ffi::CStr::from_ptr(the_str) };
    //c_str.to_bytes().len() as core::ffi::c_ulong
    
    //Really simple strlen implementation
    let mut count = 0;
    while unsafe { *the_str != 0 } {
        count += 1;
        the_str = unsafe { the_str.add(1) };
    }

    count
}

//TODO strlen_s

#[no_mangle]
pub extern "C" fn strchr(the_str: *const core::ffi::c_char, ch: core::ffi::c_int) -> *const core::ffi::c_char {
    debug_assert!(!the_str.is_null());
    let slice = unsafe { the_str.into_byte_slice() };
    if let Some(pos) = slice.iter().position(|&x| x == (ch as u8)) {
        unsafe { the_str.add(pos) }
    } else {
        core::ptr::null()
    }
}

#[no_mangle]
pub extern "C" fn memchr(ptr: *const core::ffi::c_void, ch: core::ffi::c_int, count: core::ffi::c_ulong) -> *const core::ffi::c_void {
    debug_assert!(!ptr.is_null());
    let actual_ptr = ptr as *const u8;
    let actual_ch = ch as u8;
    let slice = unsafe { core::slice::from_raw_parts(actual_ptr, count as usize) };
    if let Some(pos) = slice.iter().position(|&x| x == actual_ch) {
        unsafe { actual_ptr.add(pos) as *mut core::ffi::c_void }
    } else {
        core::ptr::null()
    }
}

#[no_mangle]
pub extern "C" fn memcmp(lhs: *const core::ffi::c_void, rhs: *const core::ffi::c_void, count: core::ffi::c_ulong) -> core::ffi::c_int {
    let mut actual_lhs = lhs as *const u8;
    let mut actual_rhs = rhs as *const u8;

    //Again, we can't use any libcore functions since they could call memcmp
    let mut current_count = 0;
    while current_count < count {
        if unsafe { *actual_lhs != *actual_rhs } {
            return unsafe { (*actual_lhs).wrapping_sub(*actual_rhs) as core::ffi::c_int };
        }

        actual_lhs = unsafe { actual_lhs.add(1) };
        actual_rhs = unsafe { actual_rhs.add(1) };
        current_count += 1;
    }
    0
}

#[no_mangle]
pub extern "C" fn memset(ptr: *mut core::ffi::c_void, value: core::ffi::c_int, count: core::ffi::c_ulong) -> *mut core::ffi::c_void {
    debug_assert!(!ptr.is_null());
    let mut actual_ptr = ptr as *mut u8;
    let actual_value = value as u8;

    //Again, we can't use any libcore functions since they could call memset
    //Thankfully, rustc auto-vectorizes this really nicely
    //If you use RUSTFLAGS="-C target-feature=+avx,+avx2" cargo build --release, it will even use AVX2 instructions
    //Or: RUSTFLAGS="-C target-feature=+avx,+avx2,+avx512f" cargo build --release
    //Then it's even faster!
    let mut current_count = 0;
    while current_count < count {
        unsafe { *actual_ptr = actual_value };
        actual_ptr = unsafe { actual_ptr.add(1) };
        current_count += 1;
    }
    ptr
}

#[no_mangle]
pub extern "C" fn memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, count: core::ffi::c_ulong) -> *mut core::ffi::c_void {
    debug_assert!(!dest.is_null());
    debug_assert!(!src.is_null());
    debug_assert_ne!(dest.cast_const(), src);
    let mut actual_dest = dest as *mut u8;
    let mut actual_src = src as *const u8;

    //Again, we can't use any libcore functions since they could call memcpy
    //Thankfully, rustc auto-vectorizes this really nicely
    //If you use RUSTFLAGS="-C target-feature=+avx,+avx2" cargo build --release, it will even use AVX2 instructions
    //Or: RUSTFLAGS="-C target-feature=+avx,+avx2,+avx512f" cargo build --release
    //Then it's even faster!
    let mut current_count = 0;
    while current_count < count {
        unsafe { *actual_dest = *actual_src };
        actual_dest = unsafe { actual_dest.add(1) };
        actual_src = unsafe { actual_src.add(1) };
        current_count += 1;
    }
    dest
}


#[no_mangle]
pub extern "C" fn nanf(arg: *const core::ffi::c_char) -> core::ffi::c_float {
    todo!()//We can't just do f32::NAN since we need to put in info based on arg
}

//#define FP_NORMAL    0
//#define FP_SUBNORMAL 1
//#define FP_ZERO      2
//#define FP_INFINITE  3
//#define FP_NAN       4
#[no_mangle]//TODO we would have to turn this into a C macro to make it work
pub extern "C" fn fpclassify(x: core::ffi::c_double) -> core::ffi::c_int {
    match x.classify() {
        core::num::FpCategory::Normal       => 0,
        core::num::FpCategory::Subnormal    => 1,
        core::num::FpCategory::Zero         => 2,
        core::num::FpCategory::Infinite     => 3,
        core::num::FpCategory::Nan          => 4,
    }
}

#[no_mangle]
pub extern "C" fn isfinite(x: core::ffi::c_double) -> core::ffi::c_int {
    if x.is_finite() {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn testing123() {
    todo!()
}

/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strlen_sanity() {
        assert_eq!(strlen(unsafe { b"Hello, world!\0".as_ptr() as *const i8 }), 13);
    }
}
