use crate::common::Endian;
use crate::memory::memory3::Memory;
use crate::cache::cache1::Cache;
use std::ops::Add;

enum BitWidth {
    Bits8(u8),
    Bits16(u16),
    Bits32(u32),
    Bits64(u64),
}

pub struct Cpu<T> {
    bit_width: BitWidth,
    reg_num: usize,
    regs: Vec<T>,
    cache: Option<Cache>,
    endian: Endian,
}

impl<T> Cpu<T> {
    pub fn new(bit_width: BitWidth, reg_num: usize, cache: Option<Cache>, endian: Endian) -> Cpu<T> {

    }

    pub fn default() -> Cpu<T> {
        Cpu<T> {
            registers: vec![0u32; 8],
            cache: None,
            endian: Endian::SmallEndian,
        }
    }

    pub fn change_cache(&mut self, cache: Option<Cache>) {
        self.cache = cache;
    }

    pub fn read_mem(&mut self, reg_index: usize, mem: &Memory, addr: usize) {
        let result: u32 = 0;

        if let Endian::SmallEndian = self.endian {
            self.registers[reg_index] = mem.read_u32(addr).unwrap();
        } else {
            result = mem.read_u32()
        }
    }
    
    pub fn write_mem(&self, reg_index: usize, mem: &mut Memory, addr: usize) {
        mem.write_u32(addr, self.registers[reg_index]).unwrap();
    }

    pub fn add(&mut self, reg1_index: usize, reg2_index: usize, result_reg_index: usize) {
        self.registers[result_reg_index] = self.registers[reg1_index] + self.registers[reg2_index];
    }

    pub fn sub(&mut self, reg1_index: usize, reg2_index: usize, result_reg_index: usize) {
        self.registers[result_reg_index] = self.registers[reg1_index] - self.registers[reg2_index];
    }
}

#[cfg(test)]
mod tests {

}