use std::vec::Vec;
use crate::common::Endian;

pub struct Memory {
    data: Vec<u8>,
    size: usize,
}

impl Memory {
    pub fn new(size: usize, init_val: u8) -> Memory {
        Memory {
            data: vec![init_val; size], 
            size,
        }
    }

    #[allow(unused)]
    pub fn default(size: usize) -> Memory {
        Memory {
            data: vec![0u8; size],
            size,
        }
    }

    #[allow(unused)]
    pub fn fill(&mut self, val: u8) {
        self.data.fill(val);
    }

    #[allow(unused)]
    pub fn get_size(&self) -> usize {
        self.size
    }

    #[allow(unused)]
    pub fn read_u8(&self, addr: usize) -> Result<u8, ()> {
        if addr >= self.size {
            return Err(())
        }
        Ok(self.data[addr])
    }

    #[allow(unused)]
    pub fn read_u16(&self, addr: usize) -> Result<u16, ()> {
        if addr >= self.size - std::mem::size_of::<u16>() + std::mem::size_of::<u8>() {
            return Err(())
        }
        let mut result: u16 = 0;
        result += (self.data[addr + 1] as u16) << 8;
        result += self.data[addr] as u16;
        Ok(result)
    }

    #[allow(unused)]
    pub fn read_u32(&self, addr: usize) -> Result<u32, ()> {
        if addr >= self.size - std::mem::size_of::<u32>() + std::mem::size_of::<u8>() {
            return Err(())
        }
        let mut result: u32 = 0;
        result += (self.data[addr + 3] as u32) << 24;
        result += (self.data[addr + 2] as u32) << 16;
        result += (self.data[addr + 1] as u32) << 8;
        result += self.data[addr] as u32;
        Ok(result)
    }

    #[allow(unused)]
    pub fn write_u8(&mut self, addr: usize, data: u8) -> Result<(), ()> {
        if addr >= self.size {
            return Err(())
        }
        self.data[addr] = data;
        Ok(())
    }

    #[allow(unused)]
    pub fn write_u16(&mut self, addr: usize, data: u16) -> Result<(), ()> {
        if addr >= self.size - std::mem::size_of::<u32>() + std::mem::size_of::<u8>() {
            return Err(())
        }
        self.data[addr] = data as u8;
        self.data[addr + 1] = (data >> 8) as u8;
        Ok(())
    }

    #[allow(unused)]
    pub fn write_u32(&mut self, addr: usize, data:u32) -> Result<(), ()> {
        if addr >= self.size - std::mem::size_of::<u32>() + std::mem::size_of::<u8>() {
            return Err(())
        }
        self.data[addr] = data as u8;
        self.data[addr + 1] = (data >> 8) as u8;
        self.data[addr + 2] = (data >> 16) as u8;
        self.data[addr + 3] = (data >> 24) as u8;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::Memory;
    use crate::common::Endian;

    const TEST_CYCLE: usize = 1000;
    const TEST_MEMORY_SIZE: usize = 1024;

    #[test]
    fn read_default_memory() {
        let m = Memory::default(TEST_MEMORY_SIZE);
        for index in 0..m.get_size() {
            assert_eq!(m.read_u8(index), Ok(0));
        }
    }

    #[test]
    fn read_fill_memory() {
        let mut mem = Memory::new(TEST_MEMORY_SIZE, 0x86);
        mem.fill(0x86);
        for index in 0..mem.get_size() {
            assert_eq!(mem.read_u8(index), Ok(0x86));
        }
    }

    #[test]
    fn write_and_read_memory_u8() {
        let mut mem = Memory::default(TEST_MEMORY_SIZE);
        for _ in 0..TEST_CYCLE {
            for addr in 0..mem.get_size() {
                mem.write_u8(addr, 0x5A).unwrap();
                assert_eq!(mem.read_u8(addr), Ok(0x5A));
            }
        }
    }
}