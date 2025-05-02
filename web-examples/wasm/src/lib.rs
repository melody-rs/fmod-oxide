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

    log("system");
    let system = unsafe { fmod::studio::System::new() }.unwrap();
    log("core system");
    let core_system = system.get_core_system().unwrap();
    log("driver");
    let driver = core_system.get_driver().unwrap();
    log("driver info");
    let driver_info = core_system.get_driver_info(driver).unwrap();
    log(&format!("{driver_info:?}"));

    system.update().unwrap();

    unsafe {
        system.release().unwrap();
    }
}
