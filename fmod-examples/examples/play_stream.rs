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

    let path = media_path_for("wave_vorbis.fsb");
    let sound_builder =
        fmod::SoundBuilder::open(&path).with_mode(fmod::Mode::LOOP_NORMAL | fmod::Mode::D2);
    let sound = system.create_stream(&sound_builder)?;

    let sub_sounds = sound.get_sub_sound_count()?;
    let sound_to_play = if sub_sounds > 0 {
        sound.get_sub_sound(0)?
    } else {
        sound
    };

    let channel = system.play_sound(sound_to_play, None, false)?;

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
                    let paused = channel.get_paused()?;
                    channel.set_paused(!paused)?;
                }
                'q' => break 'main_loop,
                _ => {}
            }
        }

        system.update()?;

        execute!(stdout, Clear(ClearType::All))?;
        execute!(stdout, MoveTo(0, 0))?;
        crossterm::terminal::disable_raw_mode()?;

        let playing = channel.is_playing().unwrap_or_default();
        let paused = channel.get_paused().unwrap_or_default();
        let pos = channel.get_position(fmod::TimeUnit::MS).unwrap_or_default();
        let length = sound.get_length(fmod::TimeUnit::MS).unwrap_or_default();

        writeln!(stdout, "==================================================")?;
        writeln!(stdout, "Play Stream Example.")?;
        writeln!(stdout, "Adapted from the official FMOD example")?;
        writeln!(stdout, "==================================================")?;
        writeln!(stdout)?;
        writeln!(stdout, "Press 1 to toggle pause")?;
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

        crossterm::terminal::enable_raw_mode()?;

        stdout.flush()?;

        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    crossterm::terminal::disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen, Show)?;

    sound.release()?;
    unsafe {
        system.close()?;
        system.release()?;
    }

    Ok(())
}
