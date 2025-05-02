use std::alloc::Layout;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{c_double, c_int, c_uint, c_ulong, c_void};

use crate::log;

thread_local! {
  static POINTERS: RefCell<HashMap<*mut c_void, Layout>> = RefCell::default();
}

#[unsafe(no_mangle)]
extern "C" fn malloc(size: c_uint) -> *mut c_void {
    let layout = Layout::from_size_align(size as _, 1).unwrap();
    let ptr = unsafe { std::alloc::alloc(layout).cast() };
    POINTERS.with_borrow_mut(|map| {
        map.insert(ptr, layout);
    });
    ptr
}

#[unsafe(no_mangle)]
extern "C" fn realloc(pointer: *mut c_void, size: c_uint) -> *mut c_void {
    POINTERS.with_borrow_mut(|map| {
        let layout = map.remove(&pointer).unwrap();
        let new_ptr = unsafe { std::alloc::realloc(pointer.cast(), layout, size as _).cast() };
        let layout = Layout::from_size_align(size as _, 1).unwrap();
        map.insert(new_ptr, layout);
        new_ptr
    })
}

#[unsafe(no_mangle)]
extern "C" fn free(pointer: *mut c_void) {
    POINTERS.with_borrow_mut(|map| {
        let layout = map.remove(&pointer).unwrap();
        unsafe { std::alloc::dealloc(pointer.cast(), layout) }
    });
}

// Apparently this is the mangled symbol of C++'s `operator delete`
// (No idea why new isn't in here, though.)
#[unsafe(no_mangle)]
extern "C" fn _ZdlPv(ptr: *mut c_void) {
    log("delete []");
    free(ptr);
}

#[unsafe(no_mangle)]
extern "C" fn strtod(_: *mut i8, _: *mut i8) -> c_double {
    log("strtod");
    0.0
}

#[unsafe(no_mangle)]
extern "C" fn atoi(_: *mut i8) -> c_int {
    log("atoi");
    0
}

#[unsafe(no_mangle)]
extern "C" fn strtoul(_: *mut c_void, _: *mut c_void) -> c_ulong {
    log("strtoul");
    0
}

#[unsafe(no_mangle)]
extern "C" fn qsort(_: *mut c_void, _: usize, _: usize, _: unsafe extern "C" fn()) {
    log("qsort");
}
