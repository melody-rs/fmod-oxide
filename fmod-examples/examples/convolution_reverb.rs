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

    let reverb_group = system.create_channel_group(fmod::c!("reverb"))?;
    let main_group = system.create_channel_group(fmod::c!("main"))?;

    let reverb_unit = system.create_dsp_by_type(fmod::DspType::ConvolutionReverb)?;
    reverb_group.add_dsp(fmod::ffi::FMOD_CHANNELCONTROL_DSP_TAIL, reverb_unit)?;

    let ir_sound = fmod::SoundBuilder::open(&media_path_for("standrews.wav"))
        .with_mode(fmod::Mode::OPEN_ONLY)
        .build(system)?;

    let ir_data = unsafe { fmod::convolution_reverb::ImpulseResponse::from_sound(ir_sound) }?;
    reverb_unit.set_parameter(fmod::convolution_reverb::IR, &ir_data)?;
    reverb_unit.set_parameter(fmod::convolution_reverb::Dry, -80.0)?;

    ir_sound.release()?;

    let sound = fmod::SoundBuilder::open(&media_path_for("singing.wav"))
        .with_mode(fmod::Mode::D3 | fmod::Mode::LOOP_NORMAL)
        .build(system)?;
    let channel = system.play_sound(sound, Some(main_group), true)?;

    let channel_head = channel.get_dsp(fmod::ffi::FMOD_CHANNELCONTROL_DSP_HEAD as _)?;
    let reverb_connection = reverb_unit.add_input(channel_head, fmod::DspConnectionType::Send)?;

    channel.set_paused(false)?;

    let mut wet_volume = 1.0;
    let mut dry_volume = 1.0;

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
                'a' => {
                    wet_volume = (wet_volume - 0.05f32).max(0.0);
                }
                'd' => {
                    wet_volume = (wet_volume + 0.05f32).min(1.0);
                }
                's' => {
                    dry_volume = (dry_volume - 0.05f32).max(0.0);
                }
                'w' => {
                    dry_volume = (dry_volume + 0.05f32).min(1.0);
                }
                'q' => break 'main_loop,
                _ => {}
            }
        }

        system.update()?;

        reverb_connection.set_mix(wet_volume)?;
        main_group.set_volume(dry_volume)?;

        execute!(stdout, Clear(ClearType::All))?;

        execute!(stdout, MoveTo(0, 0))?;
        crossterm::terminal::disable_raw_mode()?;

        stdout.write_all(b"==================================================\n")?;
        stdout.write_all(b"Convolution Example.\n")?;
        stdout.write_all(b"Adapted from the official FMOD example\n")?;
        stdout.write_all(b"==================================================\n")?;
        stdout.write_all(b"Press W and S to change dry mix\n")?;
        stdout.write_all(b"Press A and D to change wet mix\n")?;
        writeln!(
            stdout,
            "wet mix [{wet_volume:.2}] dry mix [{dry_volume:.2}]"
        )?;
        stdout.write_all(b"Press Q to quit\n")?;
        stdout.write_all(b"\n")?;

        crossterm::terminal::enable_raw_mode()?;

        stdout.flush()?;

        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    // reset terminal
    crossterm::terminal::disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen, Show)?;

    sound.release()?;
    unsafe { main_group.release()? };
    reverb_group.remove_dsp(reverb_unit)?;
    reverb_unit.disconnect_all(true, true)?;
    reverb_unit.release()?;
    unsafe { reverb_group.release()? };

    // Safety: we don't use any fmod api calls after this, so this is ok
    unsafe {
        system.close()?;
        system.release()?;
    }

    Ok(())
}
