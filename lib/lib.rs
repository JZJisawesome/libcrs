
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

#[no_mangle]
pub extern "C" fn puts(the_str: *const core::ffi::c_char) -> core::ffi::c_int {
    //TODO what if the_str is null?
    let c_str = unsafe { core::ffi::CStr::from_ptr(the_str) };
    //let regular_str = c_str.to_str().unwrap();//TODO proper except here
    //println!("{}", regular_str);//TODO does this work without std? (no)
    0
}

#[no_mangle]
pub extern "C" fn abs(int: core::ffi::c_int) -> core::ffi::c_int {
    int.abs()
}

//Cool, the compiler recognizes labs and llabs are the same on my linux system
//So both symbols go to the same spot in the binary!!!

#[no_mangle]
pub extern "C" fn labs(long: core::ffi::c_long) -> core::ffi::c_long {
    long.abs()
}

#[no_mangle]
pub extern "C" fn llabs(long_long: core::ffi::c_longlong) -> core::ffi::c_longlong {
    long_long.abs()
}

/*
#[no_mangle]
pub extern "C" fn imaxabs(intmax: core::ffi::c_intmax) -> core::ffi::c_intmax {
    intmax.abs()
}
*/
