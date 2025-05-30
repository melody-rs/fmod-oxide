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

use fmod::{Utf8CStr, c, studio::EventInstanceCallback};
use fmod_studio_examples::media_path_for;
use std::{io::Write, sync::Mutex};

pub struct ProgrammerSoundContext {
    core_system: fmod::System,
    studio_system: fmod::studio::System,
    dialogue_string: &'static Utf8CStr,
}

struct Callback;

impl EventInstanceCallback for Callback {
    fn create_programmer_sound(
        event: fmod::studio::EventInstance,
        sound_props: fmod::studio::ProgrammerSoundProperties<'_>,
    ) -> fmod::Result<()> {
        let context: &Mutex<ProgrammerSoundContext> = unsafe { &*event.get_userdata()?.cast() };
        let context = context.lock().unwrap();

        let sound_info = unsafe {
            context
                .studio_system
                .get_sound_info(context.dialogue_string)?
        };
        let sound = context.core_system.create_sound(&sound_info.builder)?;
        eprintln!("{sound_info:#?}");

        *sound_props.sound = sound;
        *sound_props.subsound_index = sound_info.subsound_index;

        Ok(())
    }

    fn destroy_programmer_sound(
        _: fmod::studio::EventInstance,
        sound_props: fmod::studio::ProgrammerSoundProperties<'_>,
    ) -> fmod::Result<()> {
        sound_props.sound.release()
    }
}

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
        &media_path_for("SFX.bank"),
        fmod::studio::LoadBankFlags::NORMAL,
    )?;

    let mut bank_index = 0;
    const BANKS: [&str; 3] = ["Dialogue_EN.bank", "Dialogue_JP.bank", "Dialogue_CN.bank"];
    let bank_path = &media_path_for(BANKS[bank_index]);

    let mut localized_bank =
        system.load_bank_file(bank_path, fmod::studio::LoadBankFlags::NORMAL)?;

    let event_description = system.get_event(c!("event:/Character/Dialogue"))?;
    let event_instance = event_description.create_instance()?;

    let mut dialogue_index = 0;
    const DIALOGUE: [&Utf8CStr; 3] = [c!("welcome"), c!("main menu"), c!("goodbye")];

    let programmer_sound_context = Mutex::new(ProgrammerSoundContext {
        core_system: system.get_core_system()?,
        studio_system: system,
        dialogue_string: DIALOGUE[dialogue_index],
    });
    let programmer_sound_context = &programmer_sound_context;
    let userdata = std::ptr::from_ref(programmer_sound_context)
        .cast::<std::ffi::c_void>()
        .cast_mut();

    event_instance.set_userdata(userdata)?;
    event_instance.set_callback::<Callback>(
        fmod::studio::EventCallbackMask::CREATE_PROGRAMMER_SOUND
            | fmod::studio::EventCallbackMask::DESTROY_PROGRAMMER_SOUND,
    )?;

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
                    localized_bank.unload()?;

                    bank_index = if bank_index < 2 { bank_index + 1 } else { 0 };
                    let bank_path = &media_path_for(BANKS[bank_index]);

                    localized_bank =
                        system.load_bank_file(bank_path, fmod::studio::LoadBankFlags::NORMAL)?;
                }
                '2' => {
                    dialogue_index = if dialogue_index < 2 {
                        dialogue_index + 1
                    } else {
                        0
                    };
                    programmer_sound_context.lock().unwrap().dialogue_string =
                        DIALOGUE[dialogue_index];
                }
                ' ' => {
                    event_instance.start()?;
                }
                'q' => {
                    break 'main_loop;
                }
                _ => {}
            }
        }

        system.update()?;

        execute!(stdout, Clear(ClearType::All))?;

        execute!(stdout, MoveTo(0, 0))?;
        crossterm::terminal::disable_raw_mode()?;

        stdout.write_all(b"==================================================\n")?;
        stdout.write_all(b"Programmer Sound Example.\n")?;
        stdout.write_all(b"Adapted from the official FMOD example\n")?;
        stdout.write_all(b"==================================================")?;
        stdout.write_all(b"\n")?;

        stdout.write_all(b"Press 1 to change language\n")?;
        stdout.write_all(b"Press 2 to change dialogue\n")?;
        stdout.write_all(b"Press SPACE to play the event\n")?;
        stdout.write_all(b"\n")?;

        stdout.write_all(b"Language:\n")?;
        writeln!(
            stdout,
            " {} English",
            if bank_index == 0 { ">" } else { " " }
        )?;
        writeln!(
            stdout,
            " {} Japanese",
            if bank_index == 1 { ">" } else { " " }
        )?;
        writeln!(
            stdout,
            " {} Chinese",
            if bank_index == 2 { ">" } else { " " }
        )?;
        stdout.write_all(b"\n")?;

        stdout.write_all(b"Dialogue:\n")?;
        writeln!(
            stdout,
            " {} Welcome to the FMOD Studio tutorial",
            if dialogue_index == 0 { ">" } else { " " }
        )?;
        writeln!(
            stdout,
            " {} This is the main menu",
            if dialogue_index == 1 { ">" } else { " " }
        )?;
        writeln!(
            stdout,
            " {} Goodbye",
            if dialogue_index == 2 { ">" } else { " " }
        )?;
        stdout.write_all(b"\n")?;

        stdout.write_all(b"Press Q to quit\n")?;

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
