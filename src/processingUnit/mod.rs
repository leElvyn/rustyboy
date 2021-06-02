use crate::utils;
use crate::enums::enumsProcessingUnit::{ArgumentTypes};


mod memory;
mod registers;

struct Cpu {
    ram: memory::FullMemory,
    registers: registers::Registers
}

impl Cpu {

    // list asm instructions
    pub fn ld(mut self, end_location: &str, start_location: &str) {
        let start_arg_type:ArgumentTypes = utils::match_argument::match_arg(start_location);
        let end_arg_type:ArgumentTypes = utils::match_argument::match_arg(end_location);
        let start_value:u8 = match start_arg_type {
            ArgumentTypes::MemoryAddress => {self.ram.access_memory({ // retrieve the value of the memory address
                let mut chars = start_location.chars(); //convert (00) memory format to u8
                chars.next();
                chars.next_back();
                chars.as_str()}.parse::<u16>().unwrap())}, // finished converting
            ArgumentTypes::RegisterAddress => {self.registers.access(start_location)}, // access the register
            ArgumentTypes::NumericalValue => {start_location.parse::<u8>().unwrap()} // parse int
            _ => panic!()
        };
        match end_arg_type {
            ArgumentTypes::MemoryAddress => {
                self.ram.update_memory({{
                    let mut chars = start_location.chars();
                    chars.next();
                    chars.next_back();
                    chars.as_str()}.parse::<u16>().unwrap()}, start_value)
            }
            ArgumentTypes::RegisterAddress => self.registers.update(end_location, start_value),
            _ => panic!()
        }
    }
}