use std::io::Read;

use sdl2::event::EventSender;

use crate::display::{Event, Display};

const FONT: [u8; 80] = 
[0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
0x20, 0x60, 0x20, 0x20, 0x70, // 1
0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
0x90, 0x90, 0xF0, 0x10, 0x10, // 4
0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
0xF0, 0x10, 0x20, 0x40, 0x40, // 7
0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
0xF0, 0x90, 0xF0, 0x90, 0x90, // A
0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
0xF0, 0x80, 0x80, 0x80, 0xF0, // C
0xE0, 0x90, 0x90, 0x90, 0xE0, // D
0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
0xF0, 0x80, 0xF0, 0x80, 0x80];  // F

pub(crate) struct Emulator {
    pub mem: [u8; 4096],
    pub pc: u16,
    pub i: u16,
    pub stack: Vec<u16>,
    registers: [u8; 16],
    // Display BUFfer internally holds on/off
    // pixels of the display using 64x32 array
    // of booleans
    dbuf: [[u8; 64]; 32],
}

impl Emulator {
    pub fn new() -> Self {
        let mut mem = [0; 4096];
        let pc = 0x200;
        let i = 0;
        let stack = vec![];
        let registers = [0u8; 16];
        let dbuf = [[0; 64]; 32];
        // load font into memory
        // TODO: decide if this is the best way to load
        // font into memory
        let mut reader = std::io::BufReader::new(&FONT[..]);
        reader.read(&mut mem[0x050..]).unwrap();
        Self {
            mem,
            pc,
            i,
            stack,
            registers,
            dbuf,
        }
    }
    // Load .ch8 file into emulator
    pub fn load_rom(&mut self, path_to_rom: &str) {
        let mut reader = std::io::BufReader::new(std::fs::File::open(path_to_rom).unwrap());
        reader.read(&mut self.mem[0x200..]).unwrap();
    }
    pub fn load_font(&mut self, path_to_font: &str) {
        todo!()
    }
    fn set_register(&mut self, register: u8, value: u8) {
        self.registers[usize::try_from(register).unwrap()] = value;
    }
    pub fn run(&mut self, event_sender: Option<EventSender>) -> Result<(), Box<dyn std::error::Error>> {
    
        let mut display = Display::new();

        loop {
            // fetch

            let instr_one = self.mem[usize::try_from(self.pc)?];
            let instr_two = self.mem[usize::try_from(self.pc)? + 1];
            
            // print current memory address being decoded
            print!("{:#04x}: ", self.pc);

            self.pc += 2; // increment PC by 2 to point to next instruction


            // decode

            match instr_one >> 4 {
                0x0 => {
                    // check for 00E0 command specifically? currently ignoring the last 3 bytes

                    display.clear();
                    println!("Screen cleared");
                }
                0x1 => {
                    // fill pc with jump address
                    self.pc = u16::from_be_bytes([0x0F & instr_one, instr_two]);

                    println!("Jumped to address {:#04x}", self.pc);
                }
                0x6 => {

                    let register = 0x0F & instr_one;
                    self.set_register(register, instr_two);

                    println!("Set register V{} to {:#04X}", register.to_string(), instr_two);
                }
                0x7 => {

                    let r_index = 0x0F & instr_one;
                    self.registers[usize::try_from(r_index).unwrap()] += instr_two;

                    println!("Add {:#04X} to register V{}", instr_two, r_index.to_string());
                }
                0xA => {

                    self.i = u16::from_be_bytes([0x0F & instr_one, instr_two]);

                    println!("Set index register to {:#04X}", self.i);
                }
                0xD => {

                    let x_reg_index = 0x0F & instr_one;
                    let y_reg_index = instr_two >> 4;
                    let height = 0xF & instr_two;

                    // load coordinates from registers
                    let mut x_coord = (self.registers[usize::try_from(x_reg_index).unwrap()] % 64) as usize;
                    let mut y_coord = (self.registers[usize::try_from(y_reg_index).unwrap()] % 32) as usize;

                    self.registers[0xF] = 0;
                    
                    for n in 0..height {
                        let sprite_addr = self.i + u16::try_from(n).unwrap();
                        let sprite_byte = self.mem[usize::try_from(sprite_addr).unwrap()];
                        for p in 0..8 {
                            // if the sprite bit is on
                            if sprite_byte & 2u8.pow(7-p) != 0 {
                                // if the display buffer point is 1
                                if self.dbuf[y_coord][x_coord] == 1 {
                                    self.dbuf[y_coord][x_coord] = 0;
                                    self.registers[0xF] = 1;
                                } else {
                                    self.dbuf[y_coord][x_coord] = 1;
                                }
                            }
                            if x_coord == 63 {
                                break
                            }
                            x_coord += 1;
                        }
                        x_coord -= 8;
                        if y_coord == 31 {
                            break;
                        }
                        y_coord += 1;
                    }
                    display.draw(self.dbuf);
                    println!("display/draw");
                }
                _ => {
                    dbg!(instr_one >> 4);
                    todo!()
                }
            }
            ::std::thread::sleep(std::time::Duration::from_millis(500));
        }

        Ok(())
    }
}
