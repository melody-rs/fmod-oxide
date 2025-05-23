use crate as fmod;
use std::sync::LazyLock;

pub static SYSTEM: LazyLock<studio::System> = LazyLock::new(|| {
    fn try_make_system() -> fmod::Result<studio::System> {
        fmod::debug::initialize(
            fmod::debug::DebugFlags::WARNING,
            fmod::debug::DebugMode::TTY,
        )?;

        // # Safety
        // LazyLock synchronizes this for us.
        let builder = unsafe { studio::SystemBuilder::new() }?;
        builder.build(
            1024,
            fmod::studio::InitFlags::NORMAL,
            fmod::InitFlags::NORMAL,
        )
    }

    try_make_system().expect("failed to create system")
});

#[test]
fn init_system() {
    LazyLock::force(&SYSTEM);
}

#[test]
fn get_core() -> fmod::Result<()> {
    let _ = SYSTEM.get_core_system()?;

    Ok(())
}
