use std::alloc::Layout;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{c_double, c_int, c_uint, c_ulong, c_void};

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
    free(ptr);
}

#[unsafe(no_mangle)]
extern "C" fn strtod(_: *mut i8, _: *mut i8) -> c_double {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn atoi(_: *mut i8) -> c_int {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn strtoul(_: *mut c_void, _: *mut c_void) -> c_ulong {
    todo!()
}

// based on https://en.wikipedia.org/wiki/Heapsort
fn swap_range(slice: &mut [u8], a: usize, b: usize, size: usize) {
    for i in 0..size {
        slice.swap(a * size + i, b * size + i);
    }
}

type CmpFn = unsafe extern "C" fn(*mut c_void, *mut c_void) -> c_int;
fn cmp_index(slice: &mut [u8], cmp: CmpFn, a: usize, b: usize, size: usize) -> std::cmp::Ordering {
    let a_real = a * size;
    let b_real = b * size;
    let [r1, r2] = slice
        .get_disjoint_mut([a_real..a_real + size, b_real..b_real + size])
        .unwrap();
    let result = unsafe { cmp(r1.as_mut_ptr().cast(), r2.as_mut_ptr().cast()) };
    match result {
        ..=-1 => std::cmp::Ordering::Less,
        0 => std::cmp::Ordering::Equal,
        1.. => std::cmp::Ordering::Greater,
    }
}

#[unsafe(no_mangle)]
extern "C" fn qsort(ptr: *mut c_void, memb_count: usize, memb_size: usize, cmp: CmpFn) {
    let slice: &mut [u8] =
        unsafe { std::slice::from_raw_parts_mut(ptr.cast(), memb_count * memb_size) };

    let mut start = memb_count / 2;
    let mut end = memb_count;
    while end > 1 {
        if start > 0 {
            start -= 1;
        } else {
            end -= 1;
            swap_range(slice, end, 0, memb_size);
        }

        let mut root = start;
        while 2 * root + 1 < end {
            let mut child = 2 * root + 1;
            if child + 1 < end && cmp_index(slice, cmp, child, child + 1, memb_size).is_lt() {
                child += 1;
            }

            if cmp_index(slice, cmp, root, child, memb_size).is_lt() {
                swap_range(slice, root, child, memb_size);
                root = child;
            } else {
                break;
            }
        }
    }
}
