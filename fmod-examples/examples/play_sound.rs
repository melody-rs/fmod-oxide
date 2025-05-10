use crossterm::{
    cursor::*,
    event::{Event, KeyCode, KeyEvent},
    execute,
    terminal::*,
};
use fmod_examples::media_path_for;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let system = unsafe { fmod::SystemBuilder::new()? }.build(32, fmod::InitFlags::NORMAL)?;

    let sound_1 = fmod::SoundBuilder::open(&media_path_for("drumloop.wav")).build(system)?;
    sound_1.set_mode(fmod::Mode::LOOP_OFF)?;

    let sound_2 = fmod::SoundBuilder::open(&media_path_for("jaguar.wav")).build(system)?;

    let sound_3 = fmod::SoundBuilder::open(&media_path_for("swish.wav")).build(system)?;

    let mut channel = None;

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

        let mut playing = false;
        let mut pos = 0;
        let mut length = 0;
        let mut paused = false;

        if let Some(channel) = channel {
            playing = channel.is_playing().unwrap_or_default();
            paused = channel.get_paused().unwrap_or_default();
            pos = channel.get_position(fmod::TimeUnit::MS).unwrap_or_default();

            if let Some(sound) = channel.get_current_sound().unwrap_or_default() {
                length = sound.get_length(fmod::TimeUnit::MS).unwrap_or_default();
            }
        }

        let (playing_count, _) = system.get_playing_channels()?;

        writeln!(stdout, "==================================================")?;
        writeln!(stdout, "Play Sound Example.")?;
        writeln!(stdout, "Adapted from the official FMOD example")?;
        writeln!(stdout, "==================================================")?;
        writeln!(stdout)?;
        writeln!(stdout, "Press 1 to play a mono sound (drumloop)")?;
        writeln!(stdout, "Press 2 to play a mono sound (jaguar)")?;
        writeln!(stdout, "Press 3 to play a stereo sound (swish)")?;
        writeln!(stdout, "Press Q to quit")?;
        writeln!(stdout)?;
        writeln!(
            stdout,
            "Time {:0>2}:{:0>2}:{:0>2}/{:0>2}:{:0>2}:{:0>2} : {}",
            pos / 1000 / 60,
            pos / 1000 % 60,
            pos / 10 % 100,
            length / 1000 / 60,
            length / 1000 % 60,
            length / 10 % 100,
            if paused {
                "Paused"
            } else if playing {
                "Playing"
            } else {
                "Stopped"
            }
        )?;
        writeln!(stdout, "Channels Playing {playing_count}")?;

        crossterm::terminal::enable_raw_mode()?;

        stdout.flush()?;

        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    crossterm::terminal::disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen, Show)?;

    sound_1.release()?;
    sound_2.release()?;
    sound_3.release()?;

    unsafe {
        system.close()?;
        system.release()?;
    }

    Ok(())
}
