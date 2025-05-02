use wasm_bindgen::prelude::*;

mod stubs;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn do_thing() {
    console_error_panic_hook::set_once();

    let system = unsafe { fmod::studio::System::new() }.unwrap();
    let core_system = system.get_core_system().unwrap();
    let driver = core_system.get_driver().unwrap();
    let driver_info = core_system.get_driver_info(driver).unwrap();
    log(&format!("{driver_info:?}"));

    system.update().unwrap();

    unsafe {
        system.release().unwrap();
    }
}
