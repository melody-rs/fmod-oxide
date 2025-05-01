mod emscripten_stubs;
use wasm_bindgen::prelude::*;

use fmod::c;

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

    let master = system
        .load_bank_file(c!("master"), fmod::studio::LoadBankFlags::NORMAL)
        .unwrap();

    let strings = system
        .load_bank_file(c!("strings"), fmod::studio::LoadBankFlags::NORMAL)
        .unwrap();

    for i in 0..strings.string_count().unwrap() {
        let (_, string) = strings.get_string_info(i).unwrap();
        log(&string)
    }

    for event in master.get_event_list().unwrap() {
        let path = event.get_path().unwrap();
        log(&path);
    }

    unsafe {
        system.release().unwrap();
    }
}
