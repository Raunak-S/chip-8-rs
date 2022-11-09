mod display;
mod emulator;

use crate::display::Display;
use crate::emulator::Emulator;
use std::{io::Read, path::Path};

const PROGRAM_PATH: &str = "ibm-logo.ch8";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize chip-8 emulator

    let mut emulator = Emulator::new();

    // load Chip-8 program into memory

    emulator.load_rom(PROGRAM_PATH);

    // send Display into another thread and keep
    // an SDL EventSender to send commands to the
    // display across threads

    let mut display = Display::new();
    let event_subsystem = display.sdl_context.event()?;
    event_subsystem.register_custom_event::<crate::display::Event>();
    let event_sender = event_subsystem.event_sender();

    std::thread::spawn(move || {
        emulator.run(event_sender).unwrap();
    });

    display.run();

    Ok(())
}
