use std::ffi::c_float;

#[unsafe(no_mangle)]
extern "C" fn modff(arg: c_float, int_part: *mut c_float) -> c_float {
    unsafe { *int_part = arg.trunc() }
    arg.fract()
}
