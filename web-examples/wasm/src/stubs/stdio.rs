use fmod::Utf8CStr;
use std::ffi::{c_int, c_long, c_void};
use std::io::{Cursor, Read, Seek};

struct File {
    cursor: Cursor<&'static [u8]>,
}

#[unsafe(no_mangle)]
extern "C" fn fclose(f: *mut c_void) -> c_int {
    drop(unsafe { Box::from_raw(f as *mut File) });
    0
}
#[unsafe(no_mangle)]
extern "C" fn fopen(path: *mut i8, _: *mut c_void) -> *mut c_void {
    let path = unsafe { Utf8CStr::from_ptr_unchecked(path) };

    let data: &[u8] = match path.as_str() {
        "master" => {
            include_bytes!("../../../../fmod-sys/fmod/linux/api/studio/examples/media/Master.bank")
        }
        "strings" => include_bytes!(
            "../../../../fmod-sys/fmod/linux/api/studio/examples/media/Master.strings.bank"
        ),
        _ => todo!(),
    };
    let boxed = Box::new(File {
        cursor: Cursor::new(data),
    });
    Box::into_raw(boxed).cast()
}

#[unsafe(no_mangle)]
extern "C" fn fseek(file: *mut c_void, offset: c_long, whence: c_int) -> c_int {
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
    unsafe {
        let file = &mut *file.cast::<File>();
        file.cursor.position() as _
    }
}

#[unsafe(no_mangle)]
extern "C" fn rewind(file: *mut c_void) {
    unsafe {
        let file = &mut *file.cast::<File>();
        file.cursor.rewind().unwrap();
    }
}

#[unsafe(no_mangle)]
extern "C" fn fread(ptr: *mut c_void, size: usize, n: usize, file: *mut c_void) -> usize {
    unsafe {
        let file = &mut *file.cast::<File>();
        let slice = std::slice::from_raw_parts_mut(ptr.cast(), size * n);
        file.cursor.read(slice).unwrap()
    }
}

#[unsafe(no_mangle)]
extern "C" fn feof(file: *mut c_void) -> c_int {
    unsafe {
        let file = &mut *file.cast::<File>();
        let is_eof = file.cursor.position() == file.cursor.get_ref().len() as u64;
        is_eof as c_int
    }
}

#[unsafe(no_mangle)]
extern "C" fn vsnprintf(_: *mut c_void, _: *mut c_void, _: *mut c_void) -> c_int {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn sscanf(_: *mut c_void, _: *mut c_void, _: *mut c_void) -> c_int {
    unimplemented!()
}
