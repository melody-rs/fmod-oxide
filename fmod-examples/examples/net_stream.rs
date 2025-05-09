// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crossterm::{
    cursor::{self, MoveTo},
    event::{Event, KeyCode, KeyEvent},
    execute,
    terminal::{self, Clear, ClearType},
};
use std::io::Write;

const TAG_COUNT: usize = 4;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let system_builder = unsafe {
        // Safety: we call this before calling any other functions and only in main, so this is safe
        fmod::SystemBuilder::new()?
    };

    let system = system_builder.build(1, fmod::InitFlags::NORMAL)?;
    // Increase the file buffer size a little bit to account for Internet lag.
    system.set_stream_buffer_size(64 * 1024, fmod::TimeUnit::RawBytes)?;

    let mut sound = fmod::SoundBuilder::open(fmod::c!(
        "http://live-radio01.mediahubaustralia.com/2TJW/mp3/"
    ))
    .with_mode(fmod::Mode::CREATE_STREAM | fmod::Mode::NONBLOCKING)
    .with_file_buffer_size(1024 * 16)
    .build(system)?;

    // use alternate screen
    let mut stdout = std::io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;
    crossterm::terminal::enable_raw_mode()?;

    let mut channel: Option<fmod::Channel> = None;
    let mut tag_strings: [String; TAG_COUNT] = std::array::from_fn(|_| String::new());
    let mut tag_index = 0;

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
                    if let Some(channel) = channel {
                        let paused = channel.get_paused()?;
                        channel.set_paused(!paused)?;
                    }
                }
                'q' => {
                    break 'main_loop;
                }
                _ => {}
            }
        }

        system.update()?;

        let (open_state, percent, starving, _) = sound.get_open_state()?;

        // Read any tags that have arrived, this could happen if a radio station switches to a new song.
        while let Ok(tag) = sound.get_tag(None, -1) {
            if let fmod::TagData::String(text) = tag.data {
                tag_strings[tag_index] = format!("{} = '{text}' ({} bytes)", tag.name, text.len());
                tag_index = (tag_index + 1) % TAG_COUNT;

                if matches!(tag.kind, fmod::TagType::Playlist) && tag.name == "FILE" {
                    sound.release()?;

                    let url = fmod::Utf8CString::new(text)?;
                    sound = fmod::SoundBuilder::open(&url)
                        .with_mode(fmod::Mode::CREATE_SAMPLE | fmod::Mode::NONBLOCKING)
                        .with_file_buffer_size(1024 * 16)
                        .build(system)?;
                }
            } else if matches!(tag.kind, fmod::TagType::Fmod) {
                // When a song changes, the sample rate may also change, so compensate here.
                if let Some(channel) = channel {
                    if tag.name == "Sample Rate Change" {
                        let fmod::TagData::Float(frequency) = tag.data else {
                            unreachable!()
                        };
                        channel.set_frequency(frequency as f32)?;
                    }
                }
            }
        }

        let mut paused = false;
        let mut playing = false;
        let mut position = 0;

        if let Some(channel) = channel {
            paused = channel.get_paused()?;
            playing = channel.is_playing()?;
            position = channel.get_position(fmod::TimeUnit::MS)?;
            // Silence the stream until we have sufficient data for smooth playback.
            channel.set_mute(starving)?;
        } else {
            // This may fail if the stream isn't ready yet, so don't check the error code.
            channel = system.play_sound(sound, None, false).ok();
        }

        let state = if matches!(open_state, fmod::OpenState::Buffering) {
            "Buffering..."
        } else if matches!(open_state, fmod::OpenState::Connecting) {
            "Connecting..."
        } else if paused {
            "Paused"
        } else if playing {
            "Playing"
        } else {
            "Stopped"
        };

        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;
        crossterm::terminal::disable_raw_mode()?;

        stdout.write_all(b"==================================================\n")?;
        stdout.write_all(b"Net Stream Example.\n")?;
        stdout.write_all(b"Adapted from the official FMOD example\n")?;
        stdout.write_all(b"==================================================")?;
        stdout.write_all(b"\n")?;
        stdout.write_all(b"Press 1 to toggle pause\n")?;
        stdout.write_all(b"Press Q to quit\n")?;
        stdout.write_all(b"\n")?;
        writeln!(
            stdout,
            "Time = {:0>2}:{:0>2}:{:0>2}",
            position / 1000,
            position / 60,
            position / 10 % 100
        )?;
        writeln!(
            stdout,
            "State = {state} {}",
            if starving { "(STARVING)" } else { "" }
        )?;
        writeln!(stdout, "Buffer Percentage = {percent}")?;
        stdout.write_all(b"Tags:\n")?;
        for tag in tag_strings[tag_index..].iter() {
            writeln!(stdout, "{tag}\n")?;
        }

        crossterm::terminal::enable_raw_mode()?;

        stdout.flush()?;

        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    // Stop the channel, then wait for it to finish opening before we release it.
    if let Some(channel) = channel {
        channel.stop()?;
    }

    let (mut open_state, _, _, _) = sound.get_open_state()?;
    while !matches!(
        open_state,
        fmod::OpenState::Ready | fmod::OpenState::Error(_)
    ) {
        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;
        crossterm::terminal::disable_raw_mode()?;

        stdout
            .write_all(b"Waiting for sound to finish opening before trying to release it....\n")?;

        crossterm::terminal::enable_raw_mode()?;
        stdout.flush()?;
        std::thread::sleep(std::time::Duration::from_millis(50));
        system.update()?;

        (open_state, _, _, _) = sound.get_open_state()?;
    }

    // reset terminal
    crossterm::terminal::disable_raw_mode()?;
    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;

    // Shut down
    sound.release()?;

    unsafe {
        // Safety: we don't use any fmod api calls after this, so this is ok
        system.close()?;
        system.release()?;
    }

    Ok(())
}
