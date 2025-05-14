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

    let sound = fmod::SoundBuilder::open(&media_path_for("drumloop.wav"))
        .with_mode(fmod::Mode::LOOP_NORMAL)
        .build(system)?;
    let channel = system.play_sound(sound, None, false)?;

    let low_pass = system.create_dsp_by_type(fmod::DspType::Lowpass)?;
    low_pass.set_parameter(fmod::lowpass::Cutoff, 1000.0)?;
    low_pass.set_parameter(fmod::lowpass::Resonance, 4.0)?;

    let high_pass = system.create_dsp_by_type(fmod::DspType::Lowpass)?;
    high_pass.set_parameter(fmod::highpass::Cutoff, 4000.0)?;
    high_pass.set_parameter(fmod::highpass::Resonance, 4.0)?;

    let main_group = system.get_master_channel_group()?;

    let head = main_group.get_dsp(fmod::ChannelControl::DSP_HEAD)?;

    let (channel_mixer, _) = head.get_input(0)?;
    head.disconnect_from(Some(channel_mixer), None)?;

    let low_pass_connection = head.add_input(low_pass, fmod::DspConnectionType::Standard)?;
    let high_pass_connection = head.add_input(high_pass, fmod::DspConnectionType::Standard)?;

    low_pass.add_input(channel_mixer, fmod::DspConnectionType::Standard)?;
    high_pass.add_input(channel_mixer, fmod::DspConnectionType::Standard)?;

    {
        let lowpass_matrix = [[1.0, 0.0], [0.0, 0.0]];
        let highpass_matrix = [[0.0, 0.0], [0.0, 1.0]];

        channel_mixer.set_channel_format(
            fmod::ChannelMask::empty(),
            0,
            fmod::SpeakerMode::Stereo,
        )?;

        low_pass_connection.set_mix_matrix(lowpass_matrix)?;
        high_pass_connection.set_mix_matrix(highpass_matrix)?;
    }

    low_pass.set_bypass(true)?;
    high_pass.set_bypass(true)?;

    low_pass.set_active(true)?;
    high_pass.set_active(true)?;

    // use alternate screen
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;
    crossterm::terminal::enable_raw_mode()?;

    let mut pan = 0.0;

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
                    let bypass = low_pass.get_bypass()?;
                    low_pass.set_bypass(!bypass)?;
                }
                '2' => {
                    let bypass = high_pass.get_bypass()?;
                    high_pass.set_bypass(!bypass)?;
                }
                'a' => {
                    pan = (pan - 0.1f32).max(-1.0);
                    channel.set_pan(pan)?;
                }
                'd' => {
                    pan = (pan + 0.1f32).min(1.0);
                    channel.set_pan(pan)?;
                }
                'q' => break 'main_loop,
                _ => {}
            }
        }

        system.update()?;

        execute!(stdout, Clear(ClearType::All))?;

        execute!(stdout, MoveTo(0, 0))?;
        crossterm::terminal::disable_raw_mode()?;

        let low_pass_bypass = low_pass.get_bypass()?;
        let high_pass_bypass = high_pass.get_bypass()?;

        stdout.write_all(b"==================================================\n")?;
        stdout.write_all(b"Convolution Example.\n")?;
        stdout.write_all(b"Adapted from the official FMOD example\n")?;
        stdout.write_all(b"==================================================\n")?;
        stdout.write_all(b"\n")?;
        stdout.write_all(b"Press 1 to toggle lowpass (left speaker)\n")?;
        stdout.write_all(b"Press 2 to toggle highpass (right speaker)\n")?;
        stdout.write_all(b"Press A or D to pan sound\n")?;
        writeln!(
            stdout,
            "Lowpass (left) is {}",
            if low_pass_bypass {
                "inactive"
            } else {
                "active"
            }
        )?;
        writeln!(
            stdout,
            "Highpass (right) is {}",
            if high_pass_bypass {
                "inactive"
            } else {
                "active"
            }
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
    low_pass.release()?;
    high_pass.release()?;

    // Safety: we don't use any fmod api calls after this, so this is ok
    unsafe {
        system.close()?;
        system.release()?;
    }

    Ok(())
}
