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

    emulator.run(None).unwrap();

    Ok(())
}
