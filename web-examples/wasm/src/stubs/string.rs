use fmod::Utf8CStr;
use std::ffi::c_int;

use crate::log;

#[unsafe(no_mangle)]
extern "C" fn strncmp(mut str1: *mut i8, mut str2: *mut i8, mut n: usize) -> c_int {
    log("strncmp");

    unsafe {
        let str1 = Utf8CStr::from_ptr_unchecked(str1);
        let str2 = Utf8CStr::from_ptr_unchecked(str2);
        log(str1);
        log(str2);
    }

    // I think this is correct?
    if n == 0 {
        return 0;
    }

    while unsafe { *str1 != 0 && *str2 != 0 } && n != 0 && unsafe { *str1 == *str2 } {
        str1 = unsafe { str1.add(1) };
        str2 = unsafe { str2.add(1) };
        n -= 1;
    }
    (unsafe { *str1 - *str2 }) as c_int
}
