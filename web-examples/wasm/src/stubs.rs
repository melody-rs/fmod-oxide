use std::ffi::{c_int, c_uint, c_void};

mod emscripten;
mod math;
mod stdio;
mod stdlib;
mod string;

// These are functions emscripten would normally provide.
// We're not using emscripten so we have to provide them ourselves.
// This is, to say the least *really dangerous*.
// If a function signature doesn't match we get a crash at best!

// I based most of these off of the bundled musl that comes with emscripten.
// Anything that would require javascript though is based off of emscripten's runtime library.

// All these do nothing.

// This is supposed to error out.
#[unsafe(no_mangle)]
extern "C" fn __cxa_pure_virtual() {
    unimplemented!();
}

// We don't bother to handle exit conditions.
#[unsafe(no_mangle)]
extern "C" fn __cxa_atexit(
    _: unsafe extern "C" fn(*mut c_void),
    _: *mut c_void,
    _: *mut c_void,
) -> c_int {
    // can't debug log in this one?
    0
}

#[unsafe(no_mangle)]
extern "C" fn usleep(_: c_uint) -> c_int {
    unimplemented!()
}
