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
use fmod::c;
use fmod_studio_examples::media_path_for;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = unsafe {
        // Safety: we call this before calling any other functions and only in main, so this is safe
        fmod::studio::SystemBuilder::new()?
    };

    // The example Studio project is authored for 5.1 sound, so set up the system output mode to match
    builder
        .core_builder()
        .software_format(0, fmod::SpeakerMode::FivePointOne, 0)?;

    let system = builder.build(
        1024,
        fmod::studio::InitFlags::NORMAL,
        fmod::InitFlags::NORMAL,
    )?;

    system.load_bank_file(
        &media_path_for("Master.bank"),
        fmod::studio::LoadBankFlags::NORMAL,
    )?;
    system.load_bank_file(
        &media_path_for("Master.strings.bank"),
        fmod::studio::LoadBankFlags::NORMAL,
    )?;
    system.load_bank_file(
        &media_path_for("Vehicles.bank"),
        fmod::studio::LoadBankFlags::NORMAL,
    )?;

    let event_description = system.get_event(c!("event:/Vehicles/Ride-on Mower"))?;
    let event_instance = event_description.create_instance()?;

    event_instance.set_parameter_by_name(c!("RPM"), 650.0, false)?;
    event_instance.start()?;

    // Position two listeners
    system.set_listener_count(2)?;

    let mut active_listener = 0usize;
    let mut listener_distance = 8.0;
    let mut listener_weight = [1.0; 2];
    let mut listener_attributes = [fmod::Attributes3D::default(); 2];

    listener_attributes[0].forward.z = 1.0;
    listener_attributes[0].up.y = 1.0;
    listener_attributes[0].position.x = -listener_distance;

    listener_attributes[1].forward.z = 1.0;
    listener_attributes[1].up.y = 1.0;
    listener_attributes[1].position.x = listener_distance;

    system.set_listener_attributes(0, listener_attributes[0], None)?;
    system.set_listener_weight(0, listener_weight[0])?;

    system.set_listener_attributes(1, listener_attributes[1], None)?;
    system.set_listener_weight(1, listener_weight[1])?;

    let mut car_attributes = fmod::Attributes3D::default();
    car_attributes.forward.z = 1.0;
    car_attributes.up.y = 1.0;
    car_attributes.position.x = 0.0;
    car_attributes.position.z = 2.0;

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
                'w' => {
                    car_attributes.position.z += 1.0;
                    event_instance.set_3d_attributes(car_attributes)?;
                }
                'a' => {
                    car_attributes.position.x -= 1.0;
                    event_instance.set_3d_attributes(car_attributes)?;
                }
                's' => {
                    car_attributes.position.z -= 1.0;
                    event_instance.set_3d_attributes(car_attributes)?;
                }
                'd' => {
                    car_attributes.position.x += 1.0;
                    event_instance.set_3d_attributes(car_attributes)?;
                }
                '1' => {
                    active_listener += 1;
                    if active_listener > 2 {
                        active_listener = 0;
                    }
                }
                '2' => {
                    active_listener = active_listener.checked_sub(1).unwrap_or(2);
                }
                '3' => {
                    listener_distance = (listener_distance - 1.0).max(0.0);
                }
                '4' => {
                    listener_distance = (listener_distance + 1.0).max(0.0);
                }
                'q' => {
                    break 'main_loop;
                }
                _ => {}
            }
        }

        for (i, weight) in listener_weight.iter_mut().enumerate() {
            // 0 = left, 1 = right, 2 = both
            let target = (active_listener == i || active_listener == 2) as i32 as f32;
            let distance = target - *weight;
            // very rough estimate of 50ms per update, not properly timed
            let step = 50.0 / 1000.0;

            if (-step..step).contains(&distance) {
                *weight = target;
            } else if distance > 0.0 {
                *weight += step;
            } else {
                *weight -= step;
            }
        }

        listener_attributes[0].position.x = -listener_distance;
        listener_attributes[1].position.x = listener_distance;

        system.set_listener_attributes(0, listener_attributes[0], None)?;
        system.set_listener_weight(0, listener_weight[0])?;
        system.set_listener_attributes(1, listener_attributes[1], None)?;
        system.set_listener_weight(1, listener_weight[1])?;

        system.update()?;

        execute!(stdout, Clear(ClearType::All))?;

        execute!(stdout, MoveTo(0, 0))?;
        crossterm::terminal::disable_raw_mode()?;

        stdout.write_all(b"==================================================\n")?;
        stdout.write_all(b"Event 3D Multi-Listener Example.\n")?;
        stdout.write_all(b"Adapted from the official FMOD example\n")?;
        stdout.write_all(b"==================================================")?;

        if let Some((row, col)) = get_character_position(fmod::Vector::default()) {
            execute!(stdout, MoveTo(col, row))?;
            stdout.write_all(b"^")?;
        }

        if let Some((row, col)) = get_character_position(car_attributes.position) {
            execute!(stdout, MoveTo(col, row))?;
            stdout.write_all(b"o")?;
        }

        if let Some((row, col)) = get_character_position(fmod::Vector {
            x: -listener_distance,
            y: 0.0,
            z: 0.0,
        }) {
            execute!(stdout, MoveTo(col, row))?;
            stdout.write_all(symbol_for_weight(listener_weight[0]))?;
        }

        if let Some((row, col)) = get_character_position(fmod::Vector {
            x: listener_distance,
            y: 0.0,
            z: 0.0,
        }) {
            execute!(stdout, MoveTo(col, row))?;
            stdout.write_all(symbol_for_weight(listener_weight[1]))?;
        }

        execute!(stdout, MoveTo(0, 20))?;
        writeln!(stdout, "Left listener: {:.0}", listener_weight[0] * 100.)?;
        writeln!(stdout, "Right listener: {:.0}", listener_weight[1] * 100.)?;
        stdout.write_all(b"Use the arrow keys (W, A, S, D) to control the event position\n")?;
        stdout.write_all(b"Use 1 and 2 to toggle left/right/both listeners\n")?;
        stdout.write_all(b"Use 3 and 4 to move listeners closer or further apart\n")?;
        stdout.write_all(b"Press Q to quit")?;

        crossterm::terminal::enable_raw_mode()?;

        stdout.flush()?;

        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    // reset terminal
    crossterm::terminal::disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen, Show)?;

    unsafe {
        // Safety: we don't use any fmod api calls after this, so this is ok
        system.release()?;
    }

    Ok(())
}

fn symbol_for_weight(weight: f32) -> &'static [u8] {
    if weight > 0.95 {
        b"X"
    } else if weight > 0.05 {
        b"x"
    } else {
        b"."
    }
}

fn get_character_position(postion: fmod::Vector) -> Option<(u16, u16)> {
    let row = (-postion.z) as i16 + 8;
    let col = postion.x as i16 + 25;

    if row.is_positive() && row < 16 && col.is_positive() && col < 50 {
        Some((row as u16 + 4, col as u16))
    } else {
        None
    }
}
