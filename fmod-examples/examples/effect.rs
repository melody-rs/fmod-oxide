// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crossterm::{
    cursor::*,
    event::{Event, KeyCode, KeyEvent},
    execute,
    terminal::*,
};

use fmod_examples::media_path_for;
use std::io::Write;

fn bypass_char(bypass: bool) -> char {
    if bypass { ' ' } else { 'x' }
}

#[allow(deprecated)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let builder = unsafe {
        // Safety: we call this before calling any other functions and only in main, so this is safe
        fmod::SystemBuilder::new()?
    };
    let system = builder.build(100, fmod::InitFlags::NORMAL)?;

    let main_group = system.get_master_channel_group()?;

    let sound = fmod::SoundBuilder::open(&media_path_for("drumloop.wav")).build(system)?;
    let channel = system.play_sound(sound, None, false)?;

    let lowpass = system.create_dsp_by_type(fmod::DspType::Lowpass)?;
    let highpass = system.create_dsp_by_type(fmod::DspType::Highpass)?;
    let echo = system.create_dsp_by_type(fmod::DspType::Echo)?;
    let flange = system.create_dsp_by_type(fmod::DspType::Flange)?;

    main_group.add_dsp(0, lowpass)?;
    main_group.add_dsp(0, highpass)?;
    main_group.add_dsp(0, echo)?;
    main_group.add_dsp(0, flange)?;

    lowpass.set_bypass(true)?;
    highpass.set_bypass(true)?;
    echo.set_bypass(true)?;
    flange.set_bypass(true)?;

    // use alternate screen
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;
    crossterm::terminal::enable_raw_mode()?;

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
                ' ' => {
                    let paused = channel.get_paused()?;
                    channel.set_paused(!paused)?;
                }
                '1' => {
                    let bypass = lowpass.get_bypass()?;
                    lowpass.set_bypass(!bypass)?;
                }
                '2' => {
                    let bypass = highpass.get_bypass()?;
                    highpass.set_bypass(!bypass)?;
                }
                '3' => {
                    let bypass = echo.get_bypass()?;
                    echo.set_bypass(!bypass)?;
                }
                '4' => {
                    let bypass = flange.get_bypass()?;
                    flange.set_bypass(!bypass)?;
                }
                'q' => break 'main_loop,
                _ => {}
            }
        }

        let paused = channel.get_paused()?;
        let lowpass_bypass = lowpass.get_bypass()?;
        let highpass_bypass = highpass.get_bypass()?;
        let echo_bypass = echo.get_bypass()?;
        let flange_bypass = flange.get_bypass()?;

        system.update()?;

        execute!(stdout, Clear(ClearType::All))?;

        execute!(stdout, MoveTo(0, 0))?;
        crossterm::terminal::disable_raw_mode()?;

        stdout.write_all(b"==================================================\n")?;
        stdout.write_all(b"Effects Example.\n")?;
        stdout.write_all(b"Adapted from the official FMOD example\n")?;
        stdout.write_all(b"==================================================\n")?;
        stdout.write_all(b"\n")?;
        stdout.write_all(b"Press SPACE to pause/unpause sound\n")?;
        stdout.write_all(b"Press 1 to toggle dsplowpass effect\n")?;
        stdout.write_all(b"Press 2 to toggle dsphighpass effect\n")?;
        stdout.write_all(b"Press 3 to toggle dspecho effect\n")?;
        stdout.write_all(b"Press 4 to toggle dspflange effect\n")?;
        stdout.write_all(b"Press Q to quit\n")?;
        stdout.write_all(b"\n")?;
        writeln!(
            stdout,
            "{} : lowpass[{}] highpass[{}] echo[{}] flange[{}]",
            if paused { "Paused" } else { "Playing" },
            bypass_char(lowpass_bypass),
            bypass_char(highpass_bypass),
            bypass_char(echo_bypass),
            bypass_char(flange_bypass)
        )?;

        crossterm::terminal::enable_raw_mode()?;

        stdout.flush()?;

        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    // reset terminal
    crossterm::terminal::disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen, Show)?;

    main_group.remove_dsp(lowpass)?;
    main_group.remove_dsp(highpass)?;
    main_group.remove_dsp(echo)?;
    main_group.remove_dsp(flange)?;

    lowpass.release()?;
    highpass.release()?;
    echo.release()?;
    flange.release()?;

    sound.release()?;

    // Safety: we don't use any fmod api calls after this, so this is ok
    unsafe {
        system.close()?;
        system.release()?;
    }

    Ok(())
}
