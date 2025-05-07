use crossterm::{
    cursor::*,
    event::{Event, KeyCode, KeyEvent},
    execute,
    terminal::*,
};
use fmod_examples::media_path_for;
use std::io::prelude::*;

fn fetch_driver(system: fmod::System) -> Result<i32, Box<dyn std::error::Error>> {
    let driver_count = system.get_driver_count()?;

    if driver_count == 0 {
        system.set_output(fmod::OutputType::NoSound)?;
    }

    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;
    crossterm::terminal::enable_raw_mode()?;

    let mut selected_index = 0;
    'main_loop: loop {
        while crossterm::event::poll(std::time::Duration::from_micros(1000))? {
            let event = crossterm::event::read()?;

            let Event::Key(KeyEvent {
                code: KeyCode::Char(character),
                ..
            }) = event
            else {
                continue;
            };

            match character {
                'w' if selected_index != 0 => {
                    selected_index -= 1;
                }
                's' if selected_index != driver_count - 1 => {
                    selected_index += 1;
                }
                '1' | 'q' => break 'main_loop,
                _ => {}
            }
        }

        execute!(stdout, Clear(ClearType::All))?;
        execute!(stdout, MoveTo(0, 0))?;
        crossterm::terminal::disable_raw_mode()?;

        writeln!(stdout, "==================================================")?;
        writeln!(stdout, "Multiple System Example.")?;
        writeln!(stdout, "Adapted from the official FMOD example")?;
        writeln!(stdout, "==================================================")?;
        writeln!(stdout)?;
        writeln!(stdout, "Choose a device for system: {system:?}")?;
        writeln!(stdout)?;
        writeln!(stdout, "Use W and S to select.")?;
        writeln!(stdout, "Press 1 to confirm.")?;
        writeln!(stdout)?;
        for i in 0..driver_count {
            let (driver_name, _, _, _, _) = system.get_driver_info(i)?;
            writeln!(
                stdout,
                "[{}] - {i}. {driver_name}",
                if i == selected_index { 'X' } else { ' ' }
            )?;
        }

        crossterm::terminal::enable_raw_mode()?;

        stdout.flush()?;

        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    Ok(selected_index)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;
    crossterm::terminal::enable_raw_mode()?;

    // This does not follow the example verbatim- we don't provide a way to get access to a system before it is initialized.
    // Still works fine though!
    let system_a = unsafe { fmod::SystemBuilder::new()? }.build(32, fmod::InitFlags::NORMAL)?;
    let driver = fetch_driver(system_a)?;
    system_a.set_driver(driver)?;

    let system_b = unsafe { fmod::SystemBuilder::new()? }.build(32, fmod::InitFlags::NORMAL)?;
    let driver = fetch_driver(system_b)?;
    system_b.set_driver(driver)?;

    let sound_a = fmod::SoundBuilder::open(&media_path_for("drumloop.wav"))
        .with_mode(fmod::Mode::LOOP_OFF)
        .build(system_a)?;
    let sound_b = fmod::SoundBuilder::open(&media_path_for("jaguar.wav")).build(system_b)?;

    'main_loop: loop {
        while crossterm::event::poll(std::time::Duration::from_micros(1000))? {
            let event = crossterm::event::read()?;

            let Event::Key(KeyEvent {
                code: KeyCode::Char(character),
                ..
            }) = event
            else {
                continue;
            };

            match character {
                '1' => {
                    system_a.play_sound(sound_a, None, false)?;
                }
                '2' => {
                    system_b.play_sound(sound_b, None, false)?;
                }
                'q' => break 'main_loop,
                _ => {}
            }
        }

        system_a.update()?;
        system_b.update()?;

        let (playing_count_a, _) = system_a.get_playing_channels()?;
        let (playing_count_b, _) = system_b.get_playing_channels()?;

        execute!(stdout, Clear(ClearType::All))?;
        execute!(stdout, MoveTo(0, 0))?;
        crossterm::terminal::disable_raw_mode()?;

        writeln!(stdout, "==================================================")?;
        writeln!(stdout, "Multiple System Example.")?;
        writeln!(stdout, "Adapted from the official FMOD example")?;
        writeln!(stdout, "==================================================")?;
        writeln!(stdout)?;
        writeln!(stdout, "Press 1 to play a sound on device A")?;
        writeln!(stdout, "Press 2 to play a sound on device B")?;
        writeln!(stdout, "Press Q to quit")?;
        writeln!(stdout)?;
        writeln!(stdout, "Channels playing on A: {playing_count_a}")?;
        writeln!(stdout, "Channels playing on B: {playing_count_b}")?;

        crossterm::terminal::enable_raw_mode()?;

        stdout.flush()?;

        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    crossterm::terminal::disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen, Show)?;

    sound_a.release()?;
    sound_b.release()?;

    unsafe {
        system_a.close()?;
        system_a.release()?;

        system_b.close()?;
        system_b.release()?;
    }

    Ok(())
}
