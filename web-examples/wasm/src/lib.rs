use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn do_thing() {
    let system = unsafe { fmod::studio::System::new() }.unwrap();
    let core_system = system.get_core_system().unwrap();
    let driver = core_system.get_driver().unwrap();
    let driver_info = core_system.get_driver_info(driver).unwrap();
    log(&format!("{driver_info:?}"));
}
