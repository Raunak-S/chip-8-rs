use std::io::Read;

use sdl2::event::EventSender;

use crate::display::Event;

pub(crate) struct Emulator {
    pub mem: [u8;4096],
    pub pc: u16,
    pub i: u16,
    pub stack: Vec<u16>,
}



impl Emulator {
    pub fn new() -> Self {
        let mem = [0;4096];
        let pc = 0x200;
        let i = 0;
        let stack = vec![];
        Self {
            mem,
            pc,
            i,
            stack,
        }
    }
    // Load .ch8 file into emulator
    pub fn load(&mut self, path_to_rom: &str) {
        let mut reader = std::io::BufReader::new(std::fs::File::open(path_to_rom).unwrap());
        reader.read(&mut self.mem[0x200..]).unwrap();
    }
    pub fn run(&mut self, event_sender: EventSender) -> Result<(), Box<dyn std::error::Error>> {
        loop {

            // fetch
    
            let instr_one = self.mem[usize::try_from(self.pc)?];
            let instr_two = self.mem[usize::try_from(self.pc)?+1];
    
            self.pc += 2;    // increment PC by 2 to point to next instruction
    
            // decode
    
            match instr_one>>4 {
                0x0 => {
                    // check for 00E0 command specifically? currently ignoring the last 3 bytes
                    
                    event_sender.push_custom_event(Event::Clear)?;
                    println!("Screen cleared");
                },
                0x1 => {
                    
                    // fill pc with jump address
                    let j_addr = u16::from_be_bytes([0x0F & instr_one, instr_two]);
                    self.pc = j_addr;

                    println!("Jumped to address {:#04x}", j_addr);
                },
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

        Ok(())
    }
}