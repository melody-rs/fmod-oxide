use std::ffi::{c_double, c_int, c_void};
use std::fmt::Write;

use fmod::Utf8CStr;
use js_sys::Reflect;
use wasm_bindgen::prelude::*;

use std::cell::RefCell;
use std::collections::HashMap;

#[unsafe(no_mangle)]
extern "C" fn emscripten_get_now() -> c_double {
    let window = web_sys::window().unwrap();
    let performance = window.performance().unwrap();
    performance.now()
}

extern "C" {
    fn FMOD_JS_MixerSlowpathFunction() -> c_int;
    fn FMOD_JS_MixerFastpathFunction() -> c_int;
}

// This is how emscripten lets JS call wasm code. It's actually not too difficult to port over,
// however FMOD only uses it to wrap 2 functions. So it's not really worth it
fn cwrap(ident: JsValue, _: JsValue, _: JsValue, _: JsValue) -> JsValue {
    let ident = ident.as_string().unwrap();
    match ident.as_str() {
        "FMOD_JS_MixerFastpathFunction" => {
            Closure::<dyn Fn() -> _>::new(|| unsafe { FMOD_JS_MixerFastpathFunction() })
                .into_js_value()
        }
        "FMOD_JS_MixerSlowpathFunction" => {
            Closure::<dyn Fn() -> _>::new(|| unsafe { FMOD_JS_MixerSlowpathFunction() })
                .into_js_value()
        }
        _ => unimplemented!(),
    }
}

thread_local! {
    static ASM_FNS: RefCell<HashMap<*const i8, js_sys::Function>> = RefCell::default();
}

// Mostly based off of the generated fmod emscripten code
#[unsafe(no_mangle)]
extern "C" fn emscripten_asm_const_int(
    code: *const i8,
    sigs: *const i8,
    mut arg_buf: *mut c_void,
) -> c_int {
    let code = unsafe { Utf8CStr::from_ptr_unchecked(code) };
    let sigs = unsafe { Utf8CStr::from_ptr_unchecked(sigs) };

    assert!(arg_buf.addr() % 16 == 0);

    // this function is the only way FMOD calls into JS.
    // It's reliant on some emscripten machinery (Module) so we define it here if it isn't already.
    let window = web_sys::window().unwrap();
    match js_sys::Reflect::get(&window, &"Module".into()) {
        Ok(obj) | Err(obj) if obj.is_undefined() => {
            let module_obj = js_sys::Object::new();

            let cwrap = Closure::<dyn Fn(_, _, _, _) -> _>::new(cwrap).into_js_value();
            Reflect::set(&module_obj, &"cwrap".into(), &cwrap).unwrap();

            Reflect::set(&window, &"Module".into(), &module_obj).unwrap();
        }
        _ => {}
    }

    let function = ASM_FNS.with_borrow_mut(|fns| {
        fns.entry(code.as_ptr())
            .or_insert_with(|| {
                let mut function_args = String::new();
                for i in 0..sigs.len() {
                    write!(function_args, "${i},").unwrap();
                }
                js_sys::Function::new_with_args(&function_args, code)
            })
            .clone()
    });

    let args = js_sys::Array::new();
    for char in sigs.chars() {
        let wide = char != 'i' && char != 'p';
        let offset = if wide && arg_buf.addr() % 8 == 0 {
            4
        } else {
            0
        };
        arg_buf = unsafe { arg_buf.byte_add(offset) };

        let js_value = match char {
            'i' => {
                let value = unsafe { *arg_buf.cast::<i32>() };
                JsValue::from(value)
            }
            'p' => {
                let value = unsafe { *arg_buf.cast::<*mut ()>() };
                JsValue::from(value)
            }
            'd' | 'f' => {
                let value = unsafe { *arg_buf.cast::<f64>() };
                JsValue::from(value)
            }
            _ => unimplemented!(),
        };
        args.push(&js_value);

        let offset = if wide { 8 } else { 4 };
        arg_buf = unsafe { arg_buf.byte_add(offset) }
    }

    let result = function.apply(&JsValue::undefined(), &args).unwrap();
    result.unchecked_into_f64() as c_int // is this correct?
}
