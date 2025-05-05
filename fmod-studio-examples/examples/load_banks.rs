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

use fmod_studio_examples::media_path_for;

use std::ffi::{c_uint, c_void};
use std::io::prelude::*;

#[derive(Clone, Copy)]
enum LoadBankMethod {
    File,
    Memory,
    MemoryPoint,
    Custom,
}

impl std::fmt::Display for LoadBankMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            LoadBankMethod::File => "File",
            LoadBankMethod::Memory => "Memory",
            LoadBankMethod::MemoryPoint => "Memory-Point",
            LoadBankMethod::Custom => "Custom",
        };
        f.write_str(str)
    }
}

struct CustomFilesystem;

impl fmod::FileSystem for CustomFilesystem {
    fn open(name: &fmod::Utf8CStr, userdata: *mut c_void) -> fmod::Result<fmod::FileInfo> {
        eprintln!("{name:?}");
        // loadBankCustom doesn't pass a name parameter by default so we have to use userdata instead
        // we made sure to include the nul in loadBankCustom so this should be ok?
        let filename = unsafe { fmod::Utf8CStr::from_ptr_unchecked(userdata.cast()) };
        let Ok(file) = std::fs::File::open(filename.as_str()) else {
            return Err(fmod::FMOD_RESULT::FMOD_ERR_FILE_NOTFOUND.into());
        };

        let file_size = file.metadata().unwrap().len();
        let Ok(file_size) = u32::try_from(file_size) else {
            return Err(fmod::FMOD_RESULT::FMOD_ERR_FILE_BAD.into());
        };

        let handle = Box::into_raw(Box::new(file)).cast();

        Ok(fmod::FileInfo { handle, file_size })
    }

    fn close(handle: *mut c_void, _: *mut c_void) -> fmod::Result<()> {
        let boxed_file: Box<std::fs::File> = unsafe { Box::from_raw(handle.cast()) };
        drop(boxed_file);
        Ok(())
    }
}

impl fmod::FileSystemSync for CustomFilesystem {
    fn read(
        handle: *mut c_void,
        _: *mut c_void,
        mut buffer: fmod::FileBuffer<'_>,
    ) -> fmod::Result<()> {
        let file: &mut std::fs::File = unsafe { &mut *handle.cast() };
        let mut reader = file.take(buffer.capacity() as _);
        match std::io::copy(&mut reader, &mut buffer) {
            Ok(_) => Ok(()),
            Err(_) => Err(fmod::FMOD_RESULT::FMOD_ERR_FILE_BAD.into()),
        }
    }

    fn seek(handle: *mut c_void, _: *mut c_void, position: c_uint) -> fmod::Result<()> {
        let file: &mut std::fs::File = unsafe { &mut *handle.cast() };
        match file.seek(std::io::SeekFrom::Start(position as _)) {
            Ok(_) => Ok(()),
            Err(_) => Err(fmod::FMOD_RESULT::FMOD_ERR_FILE_BAD.into()),
        }
    }
}

type MemoryPointData =
    aligned_vec::AVec<u8, aligned_vec::ConstAlign<{ fmod::studio::LOAD_POINT_ALIGNMENT }>>;

fn load_bank(
    system: &fmod::studio::System,
    method: LoadBankMethod,
    filename: &fmod::Utf8CStr,
) -> fmod::Result<fmod::studio::Bank> {
    match method {
        LoadBankMethod::File => {
            system.load_bank_file(filename, fmod::studio::LoadBankFlags::NONBLOCKING)
        }
        LoadBankMethod::Memory => {
            let Ok(data) = std::fs::read(filename.as_str()) else {
                return Err(fmod::FMOD_RESULT::FMOD_ERR_FILE_NOTFOUND.into());
            };
            system.load_bank_memory(&data, fmod::studio::LoadBankFlags::NONBLOCKING)
        }
        LoadBankMethod::MemoryPoint => {
            let Ok(data) = std::fs::read(filename.as_str()) else {
                return Err(fmod::FMOD_RESULT::FMOD_ERR_FILE_NOTFOUND.into());
            };
            // We relly should read directly into this but this is easier :P
            let mut aligned_data = MemoryPointData::with_capacity(0, data.len());
            aligned_data.extend_from_slice(&data);

            let bank = unsafe {
                system.load_bank_pointer(&aligned_data, fmod::studio::LoadBankFlags::NONBLOCKING)
            }?;

            let boxed = Box::into_raw(Box::new(aligned_data)).cast();
            bank.set_userdata(boxed)?;

            Ok(bank)
        }
        LoadBankMethod::Custom => {
            // we must include the nul!
            let userdata = fmod::studio::LoadBankUserdata::from_slice(filename.as_bytes_with_nul());
            system.load_bank_custom::<CustomFilesystem>(
                userdata,
                fmod::studio::LoadBankFlags::NONBLOCKING,
            )
        }
    }
}

fn loading_state_as_str(state: &fmod::studio::LoadingState) -> &'static str {
    match state {
        fmod::studio::LoadingState::Unloading => "unloading  ",
        fmod::studio::LoadingState::Unloaded => "unloaded   ",
        fmod::studio::LoadingState::Loading => "loading    ",
        fmod::studio::LoadingState::Loaded => "loaded     ",
        fmod::studio::LoadingState::Error(error) => match error {
            fmod::Error::Fmod(fmod::FMOD_RESULT::FMOD_ERR_NOTREADY) => "error (rdy)",
            fmod::Error::Fmod(fmod::FMOD_RESULT::FMOD_ERR_FILE_BAD) => "error (bad)",
            fmod::Error::Fmod(fmod::FMOD_RESULT::FMOD_ERR_FILE_NOTFOUND) => "error (mis)",
            _ => "error      ",
        },
    }
}

fn get_handle_string(bank: Option<fmod::studio::Bank>) -> &'static str {
    match bank {
        None => "null   ",
        Some(bank) if bank.is_valid() => "valid  ",
        Some(_) => "invalid",
    }
}

struct SystemCallbacks;

impl fmod::studio::SystemCallback for SystemCallbacks {
    fn bank_unload(
        _: fmod::studio::System,
        bank: fmod::studio::Bank,
        _: *mut c_void,
    ) -> fmod::Result<()> {
        let userdata = bank.get_userdata()?;
        if !userdata.is_null() {
            let boxed: Box<MemoryPointData> = unsafe { Box::from_raw(userdata.cast()) };
            drop(boxed);
        };
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let builder = unsafe {
        // Safety: we call this before calling any other functions and only in main, so this is safe
        fmod::studio::SystemBuilder::new()?
    };

    let system = builder.build(
        1024,
        fmod::studio::InitFlags::NORMAL,
        fmod::InitFlags::NORMAL,
    )?;
    system.set_callback::<SystemCallbacks>(fmod::studio::SystemCallbackMask::BANK_UNLOAD)?;

    const BANKS: [&str; 4] = ["SFX.bank", "Music.bank", "Vehicles.bank", "VO.bank"];

    let mut banks = [None; BANKS.len()];
    let mut want_bank_loaded = [false; BANKS.len()];
    let mut want_sample_load = true;

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

            if let Some(index) = ['1', '2', '3', '4']
                .into_iter()
                .enumerate()
                .find_map(|(i, c)| (character == c).then_some(i))
            {
                if !want_bank_loaded[index] {
                    let method = match index {
                        0 => LoadBankMethod::File,
                        1 => LoadBankMethod::Memory,
                        2 => LoadBankMethod::MemoryPoint,
                        3 => LoadBankMethod::Custom,
                        _ => unreachable!(),
                    };
                    let filename = media_path_for(BANKS[index]);
                    let bank = load_bank(&system, method, &filename)?;
                    banks[index] = Some(bank);
                } else {
                    banks[index].unwrap().unload()?;
                }
                want_bank_loaded[index] = !want_bank_loaded[index];
            }

            match character {
                ' ' => want_sample_load = !want_sample_load,
                'q' => break 'main_loop,
                _ => {}
            }
        }

        let load_state: [_; BANKS.len()] = std::array::from_fn(|i| match banks[i] {
            Some(b) if b.is_valid() => b.get_loading_state().unwrap(),
            Some(_) | None => fmod::studio::LoadingState::Unloaded,
        });
        let sample_load_state: [_; BANKS.len()] = std::array::from_fn(|i| match banks[i] {
            Some(b) if b.is_valid() && load_state[i] == fmod::studio::LoadingState::Loaded => {
                let state = b.get_sample_loading_state().unwrap();
                let is_load = matches!(
                    state,
                    fmod::studio::LoadingState::Loading | fmod::studio::LoadingState::Loaded
                );
                if want_sample_load && state == fmod::studio::LoadingState::Unloaded {
                    b.load_sample_data().unwrap()
                } else if !want_sample_load && is_load {
                    b.unload_sample_data().unwrap()
                }
                state
            }
            Some(_) | None => fmod::studio::LoadingState::Unloaded,
        });

        system.update()?;

        execute!(stdout, Clear(ClearType::All))?;

        execute!(stdout, MoveTo(0, 0))?;
        crossterm::terminal::disable_raw_mode()?;

        stdout.write_all(b"==================================================\n")?;
        stdout.write_all(b"Bank Load Example.\n")?;
        stdout.write_all(b"Adapted from the official FMOD example\n")?;
        stdout.write_all(b"==================================================")?;
        stdout.write_all(b"Name            *mut c_void  Bank-State  Sample-State\n")?;

        for i in 0..BANKS.len() {
            writeln!(
                stdout,
                "{:>15} {} {} {}",
                BANKS[i],
                get_handle_string(banks[i]),
                loading_state_as_str(&load_state[i]),
                loading_state_as_str(&sample_load_state[i])
            )?;
        }

        stdout.write_all(b"\n")?;
        writeln!(
            stdout,
            "Press 1 to load bank 1 via {}",
            LoadBankMethod::File
        )?;
        writeln!(
            stdout,
            "Press 2 to load bank 2 via {}",
            LoadBankMethod::Memory
        )?;
        writeln!(
            stdout,
            "Press 3 to load bank 3 via {}",
            LoadBankMethod::MemoryPoint
        )?;
        writeln!(
            stdout,
            "Press 4 to load bank 4 via {}",
            LoadBankMethod::Custom
        )?;
        stdout.write_all(b"Press SPACE to toggle sample data\n")?;
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
