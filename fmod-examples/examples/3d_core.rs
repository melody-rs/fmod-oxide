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

const DISTANCE_FACTOR: f32 = 1.0;
const INSTANCE_UPDATE_TIME: f32 = 50.0;

const FORWARD: fmod::Vector = fmod::Vector {
    x: 0.0,
    y: 0.0,
    z: 1.0,
};
const UP: fmod::Vector = fmod::Vector {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let builder = unsafe {
        // Safety: we call this before calling any other functions and only in main, so this is safe
        fmod::SystemBuilder::new()?
    };
    let system = builder.build(100, fmod::InitFlags::NORMAL)?;

    system.set_3d_settings(1.0, DISTANCE_FACTOR, 1.0)?;

    let sound_1 = fmod::SoundBuilder::open(&media_path_for("drumloop.wav"))
        .with_mode(fmod::Mode::D3)
        .build(system)?;
    sound_1.set_3d_min_max_distance(0.5 * DISTANCE_FACTOR, 5000.0 * DISTANCE_FACTOR)?;
    sound_1.set_mode(fmod::Mode::LOOP_NORMAL)?;

    let sound_2 = fmod::SoundBuilder::open(&media_path_for("jaguar.wav"))
        .with_mode(fmod::Mode::D3)
        .build(system)?;
    sound_2.set_3d_min_max_distance(0.5 * DISTANCE_FACTOR, 5000.0 * DISTANCE_FACTOR)?;
    sound_2.set_mode(fmod::Mode::LOOP_NORMAL)?;

    let sound_3 = fmod::SoundBuilder::open(&media_path_for("swish.wav"))
        .with_mode(fmod::Mode::D2)
        .build(system)?;

    let channel_1 = {
        let pos = fmod::Vector {
            x: -10.0 * DISTANCE_FACTOR,
            y: 0.0,
            z: 0.0,
        };
        let vel = fmod::Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let channel = system.play_sound(sound_1, None, true)?;
        channel.set_3d_attributes(Some(pos), Some(vel))?;
        channel.set_paused(false)?;
        channel
    };

    let channel_2 = {
        let pos = fmod::Vector {
            x: 15.0 * DISTANCE_FACTOR,
            y: 0.0,
            z: 0.0,
        };
        let vel = fmod::Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let channel = system.play_sound(sound_2, None, true)?;
        channel.set_3d_attributes(Some(pos), Some(vel))?;
        channel.set_paused(false)?;
        channel
    };

    let mut listener_flag = true;
    let mut listener_pos = fmod::Vector {
        x: 0.0,
        y: 0.0,
        z: -DISTANCE_FACTOR,
    };

    let mut t = 0.0;
    let mut last_pos = fmod::Vector {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

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
                    let paused = channel_1.get_paused()?;
                    channel_1.set_paused(!paused)?;
                }
                '2' => {
                    let paused = channel_2.get_paused()?;
                    channel_2.set_paused(!paused)?;
                }
                '3' => {
                    system.play_sound(sound_3, None, false)?;
                }
                ' ' => {
                    listener_flag = !listener_flag;
                }
                'a' if !listener_flag => {
                    listener_pos.x -= 1.0 * DISTANCE_FACTOR;
                    if listener_pos.x < -24.0 * DISTANCE_FACTOR {
                        listener_pos.x = -24.0 * DISTANCE_FACTOR
                    }
                }
                'd' if !listener_flag => {
                    listener_pos.x += 1.0 * DISTANCE_FACTOR;
                    if listener_pos.x > 23.0 * DISTANCE_FACTOR {
                        listener_pos.x = 23.0 * DISTANCE_FACTOR
                    }
                }
                'q' => break 'main_loop,
                _ => {}
            }
        }

        {
            if listener_flag {
                listener_pos.x = (t * 0.05_f32).sin() * 24.0 * DISTANCE_FACTOR;
            }

            let velocity = fmod::Vector {
                x: (listener_pos.x - last_pos.x) * (1000.0 / INSTANCE_UPDATE_TIME),
                y: (listener_pos.y - last_pos.y) * (1000.0 / INSTANCE_UPDATE_TIME),
                z: (listener_pos.z - last_pos.z) * (1000.0 / INSTANCE_UPDATE_TIME),
            };
            last_pos = listener_pos;

            system.set_3d_listener_attributes(
                0,
                Some(listener_pos),
                Some(velocity),
                Some(FORWARD),
                Some(UP),
            )?;
            t += 30.0 * (1.0 / INSTANCE_UPDATE_TIME);
        }

        system.update()?;

        execute!(stdout, Clear(ClearType::All))?;

        execute!(stdout, MoveTo(0, 0))?;
        crossterm::terminal::disable_raw_mode()?;

        stdout.write_all(b"==================================================\n")?;
        stdout.write_all(b"3D Example.\n")?;
        stdout.write_all(b"Adapted from the official FMOD example\n")?;
        stdout.write_all(b"==================================================")?;
        stdout.write_all(b"\n")?;
        stdout.write_all(b"Press 1 to toggle sound 1 (16bit Mono 3D)\n")?;
        stdout.write_all(b"Press 2 to toggle sound 2 (8bit Mono 3D)\n")?;
        stdout.write_all(b"Press 3 to play a sound (16bit Stereo 2D)\n")?;
        stdout.write_all(b"Press A or D to move listener in still mode\n")?;
        stdout.write_all(b"Press SPACE to toggle listener auto movement\n")?;
        stdout.write_all(b"Press Q to quit\n")?;
        stdout.write_all(b"\n")?;

        let mut bytes = *b"|.............<1>......................<2>.......|";
        bytes[(listener_pos.x / DISTANCE_FACTOR + 25.0) as usize] = b'L';
        stdout.write_all(&bytes)?;

        crossterm::terminal::enable_raw_mode()?;

        stdout.flush()?;

        let dur = std::time::Duration::from_secs_f32(INSTANCE_UPDATE_TIME / 1000.0);
        std::thread::sleep(dur);
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
