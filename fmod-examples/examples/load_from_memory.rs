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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let builder = unsafe {
        // Safety: we call this before calling any other functions and only in main, so this is safe
        fmod::SystemBuilder::new()?
    };
    let system = builder.build(100, fmod::InitFlags::NORMAL)?;

    let data = std::fs::read(media_path_for("drumloop.wav").as_str())?;
    let sound_builder =
        unsafe { fmod::SoundBuilder::open_memory(&data) }.with_mode(fmod::Mode::LOOP_OFF);
    let sound_1 = system.create_sound(&sound_builder)?;

    let data = std::fs::read(media_path_for("jaguar.wav").as_str())?;
    let sound_builder = unsafe { fmod::SoundBuilder::open_memory(&data) };
    let sound_2 = system.create_sound(&sound_builder)?;

    let data = std::fs::read(media_path_for("swish.wav").as_str())?;
    let sound_builder = unsafe { fmod::SoundBuilder::open_memory(&data) };
    let sound_3 = system.create_sound(&sound_builder)?;

    // use alternate screen
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;
    crossterm::terminal::enable_raw_mode()?;

    let mut channel = None;

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
                    channel = Some(system.play_sound(sound_1, None, false)?);
                }
                '2' => {
                    channel = Some(system.play_sound(sound_2, None, false)?);
                }
                '3' => {
                    channel = Some(system.play_sound(sound_3, None, false)?);
                }
                'q' => break 'main_loop,
                _ => {}
            }
        }

        system.update()?;

        execute!(stdout, Clear(ClearType::All))?;

        execute!(stdout, MoveTo(0, 0))?;
        crossterm::terminal::disable_raw_mode()?;

        let mut position = 0;
        let mut length_ms = 0;
        let mut playing = false;
        let mut paused = false;
        if let Some(channel) = channel {
            if let Ok(p) = channel.is_playing() {
                playing = p;
            }
            if let Ok(p) = channel.get_paused() {
                paused = p;
            }
            if let Ok(p) = channel.get_position(fmod::TimeUnit::MS) {
                position = p
            }
            if let Some(sound) = channel.get_current_sound().ok().flatten() {
                if let Ok(len) = sound.get_length(fmod::TimeUnit::MS) {
                    length_ms = len
                }
            }
        }

        let (channels_playing, _) = system.get_playing_channels()?;

        stdout.write_all(b"==================================================\n")?;
        stdout.write_all(b"Load From Memory Example.\n")?;
        stdout.write_all(b"Adapted from the official FMOD example\n")?;
        stdout.write_all(b"==================================================")?;
        stdout.write_all(b"\n")?;
        stdout.write_all(b"Press 1 to play a mono sound (drumloop)\n")?;
        stdout.write_all(b"Press 2 to play a mono sound (jaguar)\n")?;
        stdout.write_all(b"Press 3 to play a stereo sound (swish)\n")?;
        stdout.write_all(b"Press Q to quit\n")?;
        stdout.write_all(b"\n")?;
        writeln!(
            stdout,
            "Time {:0>2}:{:0>2}:{:0>2}/{:0>2}:{:0>2}:{:0>2}: {}",
            position / 1000 / 60,
            position / 1000 % 60,
            position / 10 % 100,
            length_ms / 1000 / 60,
            length_ms / 1000 % 60,
            length_ms / 10 % 100,
            if paused {
                "Paused"
            } else if playing {
                "Playing"
            } else {
                "Stopped"
            }
        )?;
        writeln!(stdout, "Channels Playing {channels_playing:0>2}",)?;

        crossterm::terminal::enable_raw_mode()?;

        stdout.flush()?;

        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    sound_1.release()?;
    sound_2.release()?;
    sound_3.release()?;

    // reset terminal
    crossterm::terminal::disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen, Show)?;

    // Safety: we don't use any fmod api calls after this, so this is ok
    unsafe {
        system.close()?;
        system.release()?;
    }

    Ok(())
}
