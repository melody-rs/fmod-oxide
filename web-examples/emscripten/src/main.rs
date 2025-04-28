fn main() {
    let system = unsafe { fmod::studio::System::new() }.unwrap();
    let core_system = system.get_core_system().unwrap();
    let driver = core_system.get_driver().unwrap();
    let driver_info = core_system.get_driver_info(driver).unwrap();
    println!("{driver_info:?}");
}
