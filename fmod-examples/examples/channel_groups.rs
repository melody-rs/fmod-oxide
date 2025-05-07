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

    const SOUND_NAMES: [&str; 6] = [
        "drumloop.wav",
        "jaguar.wav",
        "swish.wav",
        "c.ogg",
        "d.ogg",
        "e.ogg",
    ];
    let sounds = SOUND_NAMES
        .iter()
        .map(|n| {
            fmod::SoundBuilder::open(&media_path_for(n))
                .with_mode(fmod::Mode::LOOP_NORMAL)
                .build(system)
        })
        .collect::<fmod::Result<Vec<_>>>()?;

    let group_a = system.create_channel_group(fmod::c!("Group A"))?;
    let group_b = system.create_channel_group(fmod::c!("Group B"))?;
    let main_group = system.get_master_channel_group()?;

    main_group.add_group(group_a, true)?;
    main_group.add_group(group_b, true)?;

    sounds
        .iter()
        .enumerate()
        .try_for_each(|(i, &sound)| -> fmod::Result<()> {
            let channel = system.play_sound(sound, None, true)?;
            let group = if i < 3 { group_a } else { group_b };
            channel.set_channel_group(group)?;
            channel.set_paused(false)?;
            Ok(())
        })?;

    group_a.set_volume(0.5)?;
    group_b.set_volume(0.5)?;

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
                '1' => {
                    let mute = group_a.get_mute()?;
                    group_a.set_mute(!mute)?;
                }
                '2' => {
                    let mute = group_b.get_mute()?;
                    group_b.set_mute(!mute)?;
                }
                '3' => {
                    let mute = main_group.get_mute()?;
                    main_group.set_mute(!mute)?;
                }
                'q' => break 'main_loop,
                _ => {}
            }
        }

        system.update()?;

        execute!(stdout, Clear(ClearType::All))?;

        execute!(stdout, MoveTo(0, 0))?;
        crossterm::terminal::disable_raw_mode()?;

        let (channels_playing, _) = system.get_playing_channels()?;

        stdout.write_all(b"==================================================\n")?;
        stdout.write_all(b"Channel Groups Example.\n")?;
        stdout.write_all(b"Adapted from the official FMOD example\n")?;
        stdout.write_all(b"==================================================")?;
        stdout.write_all(b"\n")?;
        stdout.write_all(b"Group A : drumloop.wav, jaguar.wav, swish.wav\n")?;
        stdout.write_all(b"Group B : c.ogg, d.ogg, e.ogg\n")?;
        stdout.write_all(b"\n")?;
        stdout.write_all(b"Press 1 to mute/unmute group A\n")?;
        stdout.write_all(b"Press 2 to mute/unmute group B\n")?;
        stdout.write_all(b"Press 3 to mute/unmute main group\n")?;
        stdout.write_all(b"Press Q to quit\n")?;
        stdout.write_all(b"\n")?;
        writeln!(stdout, "Channels Playing {channels_playing:0>2}",)?;

        crossterm::terminal::enable_raw_mode()?;

        stdout.flush()?;

        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    let mut pitch = 1.0;
    let mut volume = 1.0;
    for _ in 0..200 {
        main_group.set_pitch(pitch)?;
        main_group.set_volume(volume)?;

        volume -= 1.0 / 200.0;
        pitch -= 0.5 / 200.0;

        system.update()?;

        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    for sound in sounds {
        sound.release()?;
    }

    unsafe { group_a.release()? };
    unsafe { group_b.release()? };

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
