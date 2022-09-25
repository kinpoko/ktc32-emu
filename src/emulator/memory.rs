#[derive(Debug)]
pub struct Memory {
    pub data: Vec<u8>,
}

pub const MEMORY_SIZE: u32 = 1024; // 1KiB

impl Memory {
    pub fn new(data: &Vec<u8>) -> Memory {
        let mut memory = Memory { data: Vec::new() };

        memory.data = data.clone();
        memory
    }

    pub fn init(&mut self) {
        for _i in 0..((MEMORY_SIZE as usize) - self.data.len()) {
            self.data.push(0);
        }
    }

    pub fn read_data(&self, address: u32) -> u32 {
        let data = (self.data[(address + 3) as usize] as u32) << 24
            | (self.data[(address + 2) as usize] as u32) << 16
            | (self.data[(address + 1) as usize] as u32) << 8
            | self.data[address as usize] as u32;
        return data;
    }

    pub fn write_data(&mut self, address: u32, data: u32) {
        self.data[(address) as usize] = (data & 0x000000FF) as u8;
        self.data[(address + 1) as usize] = (data & 0x0000FF00) as u8;
        self.data[(address + 2) as usize] = (data & 0x00FF0000) as u8;
        self.data[(address + 3) as usize] = (data & 0xFF000000) as u8;
    }
}