use std::sync::OnceLock;

use wasm_bindgen::prelude::*;

mod stubs;

use fmod::c;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn do_thing() {
    console_error_panic_hook::set_once();

    try_thing().unwrap();
}

static SYSTEM: OnceLock<fmod::studio::System> = OnceLock::new();

fn try_thing() -> Result<(), fmod::Error> {
    let system = unsafe { fmod::studio::System::new() }?;
    let core_system = system.get_core_system()?;
    let driver = core_system.get_driver()?;
    let driver_info = core_system.get_driver_info(driver)?;
    log(&format!("{driver_info:?}"));
    SYSTEM.set(system).unwrap();

    Ok(())
}

#[wasm_bindgen]
pub fn start_thing() {
    try_start_thing().unwrap();
}

fn try_start_thing() -> Result<(), fmod::Error> {
    let system = SYSTEM.get().unwrap();

    system.load_bank_file(c!("master"), fmod::studio::LoadBankFlags::NORMAL)?;
    system.load_bank_file(c!("strings"), fmod::studio::LoadBankFlags::NORMAL)?;
    system.load_bank_file(c!("vehicles"), fmod::studio::LoadBankFlags::NORMAL)?;

    let event_description = system.get_event(c!("event:/Vehicles/Ride-on Mower"))?;
    let event_instance = event_description.create_instance()?;

    event_instance.set_parameter_by_name(c!("RPM"), 650.0, false)?;
    event_instance.start()?;

    let mut attributes = fmod::Attributes3D::default();
    attributes.forward.z = 1.0;
    attributes.up.y = 1.0;

    system.set_listener_attributes(0, attributes, None)?;

    attributes.position.z = 2.0;
    event_instance.set_3d_attributes(attributes)?;

    system.update()?;

    Ok(())
}
