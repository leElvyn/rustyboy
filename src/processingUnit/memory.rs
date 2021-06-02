#[derive(Copy, Clone)]
pub struct FullMemory {
    work_ram: [u8; 8192]
}

impl FullMemory {

    pub fn update_memory(&mut self, address: u16, value: u8) {
        match address {
            0..=8192 => {self.work_ram[address as usize] = value}
            _ => {panic!()}
        }
    }
    pub fn access_memory(&self, address: u16) -> u8 {
        match address {
            0..=8192 => {return self.work_ram[address as usize]}
            _ => {return 0}
        }
    }
}

