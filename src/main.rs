mod display;

use std::{path::Path, io::Read};
use crate::display::Display;

const PROGRAM_PATH: &str = "ibm-logo.ch8";

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut mem: [u8;4096] = [0;4096];
    let mut pc: u16 = 0;
    let mut I: u16 = 0;
    let stack: Vec<u16> = vec![];
    
    // load Chip-8 program into memory

    let mut reader = std::io::BufReader::new(std::fs::File::open(PROGRAM_PATH)?);

    reader.read(mem.as_mut_slice())?;

    let mut display = Display::new();
    display.demo();

    loop {

        // fetch

        let instr_one = mem[usize::try_from(pc)?];
        let instr_two = mem[usize::try_from(pc)?+1].to_be_bytes();

        pc += 2;    // increment PC by 2 to point to next instruction

        

        // decode

        match instr_one>>4 {
            0x0 => {println!("clear screen")},
            0x1 => {println!("jump")},
            0x6 => {println!("set register")},
            0x7 => {println!("add value")},
            0xA => {println!("set index")},
            0xD => {println!("display/draw")},
            _ => {
                dbg!(instr_one>>4);
                todo!()
            },
        }

        // execute

    }

    println!("Hello, world!");
}
