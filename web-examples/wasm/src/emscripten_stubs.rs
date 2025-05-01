use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{c_double, c_float, c_int, c_long, c_ulong};
use std::io::{Cursor, Read, Seek};
use std::{alloc::Layout, ffi::c_void, os::raw::c_uint};

use fmod::Utf8CStr;

use super::log;

thread_local! {
  static POINTERS: RefCell<HashMap<*mut c_void, Layout>> = RefCell::default();
}

// These are functions emscripten would normally provide.
// We're not using emscripten so we have to provide them ourselves.
// This is, to say the least *really dangerous*.
// If a function signature doesn't match we get a crash at best!

#[unsafe(no_mangle)]
extern "C" fn malloc(size: c_uint) -> *mut c_void {
    log("malloc");
    let layout = Layout::from_size_align(size as _, 1).unwrap();
    let ptr = unsafe { std::alloc::alloc(layout).cast() };
    POINTERS.with_borrow_mut(|map| {
        map.insert(ptr, layout);
    });
    ptr
}

#[unsafe(no_mangle)]
extern "C" fn realloc(pointer: *mut c_void, size: c_uint) -> *mut c_void {
    log("realloc");
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
    log("free");
    POINTERS.with_borrow_mut(|map| {
        let layout = map.remove(&pointer).unwrap();
        unsafe { std::alloc::dealloc(pointer.cast(), layout) }
    });
}

#[unsafe(no_mangle)]
extern "C" fn _ZdlPv(ptr: *mut c_void) {
    log("delete []");
    free(ptr);
}
#[unsafe(no_mangle)]
extern "C" fn __cxa_pure_virtual() {
    log("__cxa_pure_virtual");
}
#[unsafe(no_mangle)]
extern "C" fn qsort(_: *mut c_void, _: usize, _: usize, _: unsafe extern "C" fn()) {
    log("qsort");
}
#[unsafe(no_mangle)]
extern "C" fn __cxa_atexit(
    _: unsafe extern "C" fn(*mut c_void),
    _: *mut c_void,
    _: *mut c_void,
) -> c_int {
    // log("__cxa_atexit");
    0
}

struct File {
    cursor: Cursor<&'static [u8]>,
}

#[unsafe(no_mangle)]
extern "C" fn fclose(f: *mut c_void) -> c_int {
    log("fclose");
    drop(unsafe { Box::from_raw(f as *mut File) });
    0
}
#[unsafe(no_mangle)]
extern "C" fn fopen(path: *mut i8, _: *mut c_void) -> *mut c_void {
    let path = unsafe { Utf8CStr::from_ptr_unchecked(path) };

    log("fopen");

    let data: &[u8] = match path.as_str() {
        "master" => {
            include_bytes!("../../../fmod-sys/fmod/linux/api/studio/examples/media/Master.bank")
        }
        "strings" => include_bytes!(
            "../../../fmod-sys/fmod/linux/api/studio/examples/media/Master.strings.bank"
        ),
        _ => todo!(),
    };
    let boxed = Box::new(File {
        cursor: Cursor::new(data),
    });
    Box::into_raw(boxed).cast()
}
#[unsafe(no_mangle)]
extern "C" fn strncmp(_: *mut c_void, _: *mut c_void, _: usize) -> c_int {
    log("strncmp");
    0
}
#[unsafe(no_mangle)]
extern "C" fn atoi(_: *mut c_void) -> c_int {
    log("atoi");
    0
}
#[unsafe(no_mangle)]
extern "C" fn strtoul(_: *mut c_void, _: *mut c_void) -> c_ulong {
    log("strtoul");
    0
}
#[unsafe(no_mangle)]
extern "C" fn vsnprintf(_: *mut c_void, _: *mut c_void, _: *mut c_void) -> c_int {
    log("vsnprintf");
    0
}
#[unsafe(no_mangle)]
extern "C" fn fseek(file: *mut c_void, offset: c_long, whence: c_int) -> c_int {
    log("fseek");
    unsafe {
        let file = &mut *file.cast::<File>();
        let _ = match whence {
            0 => file.cursor.seek(std::io::SeekFrom::Start(offset as _)),
            1 => file.cursor.seek(std::io::SeekFrom::Current(offset as _)),
            2 => file.cursor.seek(std::io::SeekFrom::End(offset as _)),
            _ => unimplemented!(),
        };
        0
    }
}
#[unsafe(no_mangle)]
extern "C" fn ftell(file: *mut c_void) -> c_long {
    log("ftell");
    unsafe {
        let file = &mut *file.cast::<File>();
        file.cursor.position() as _
    }
}
#[unsafe(no_mangle)]
extern "C" fn rewind(file: *mut c_void) {
    log("rewind");
    unsafe {
        let file = &mut *file.cast::<File>();
        file.cursor.rewind().unwrap();
    }
}
#[unsafe(no_mangle)]
extern "C" fn fread(ptr: *mut c_void, size: usize, n: usize, file: *mut c_void) -> usize {
    log("fread");
    unsafe {
        let file = &mut *file.cast::<File>();
        let slice = std::slice::from_raw_parts_mut(ptr.cast(), size * n);
        file.cursor.read(slice).unwrap()
    }
}
#[unsafe(no_mangle)]
extern "C" fn feof(file: *mut c_void) -> c_int {
    log("feof");
    unsafe {
        let file = &mut *file.cast::<File>();
        let is_eof = file.cursor.position() == file.cursor.get_ref().len() as u64;
        is_eof as c_int
    }
}
#[unsafe(no_mangle)]
extern "C" fn emscripten_get_now() -> c_double {
    log("emscripten_get_now");
    0.0
}
#[unsafe(no_mangle)]
extern "C" fn usleep(_: c_uint) -> c_int {
    log("usleep");
    0
}
#[unsafe(no_mangle)]
extern "C" fn emscripten_asm_const_int(code: *const i8, sigs: *const i8, _: *mut c_void) -> c_int {
    let code = unsafe { Utf8CStr::from_ptr_unchecked(code) };
    let sigs = unsafe { Utf8CStr::from_ptr_unchecked(sigs) };

    log(code);
    log(sigs);

    0
}
#[unsafe(no_mangle)]
extern "C" fn modff(_: c_float, _: *mut c_void) -> c_float {
    log("modff");
    0.0
}
#[unsafe(no_mangle)]
extern "C" fn sscanf(_: *mut c_void, _: *mut c_void, _: *mut c_void) -> c_int {
    log("sscanf");
    0
}
#[unsafe(no_mangle)]
extern "C" fn strtod(_: *mut c_void, _: *mut c_void) -> c_double {
    log("strtod");
    0.0
}
