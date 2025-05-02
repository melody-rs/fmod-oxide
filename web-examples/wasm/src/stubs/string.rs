use std::ffi::c_int;

#[unsafe(no_mangle)]
extern "C" fn strncmp(mut str1: *mut i8, mut str2: *mut i8, mut n: usize) -> c_int {
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
