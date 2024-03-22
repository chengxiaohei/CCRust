use crate::memory::memory3::Memory;
use crate::cache::cache1::Cache;
use std::vec::Vec;
use crate::common::Endian;

pub struct Core<T> {
    core_id: u8,
    bit_width: usize,
    reg_num: usize,
    pub regs: Vec<T>,
    icache: Option<Cache>,
    dcache: Option<Cache>,
    l2_cache: Option<Cache>,
    endian: Endian,
}

impl Core<u32> {
    #[allow(unused)]
    pub fn new(core_id: u8, bit_width: usize, reg_num: usize, icache: Option<Cache>, dcache: Option<Cache>, l2_cache: Option<Cache>, endian: Endian) -> Self {
        Self {
            core_id,
            bit_width: std::mem::size_of::<u32>() * std::mem::size_of::<u8>(),
            reg_num,
            regs: vec![0; reg_num],
            icache,
            dcache,
            l2_cache,
            endian
        }
    }

    #[allow(unused)]
    pub fn default() -> Self {
        Self {
            core_id: 0,
            bit_width: std::mem::size_of::<u32>() * std::mem::size_of::<u8>(),
            reg_num: 8,
            regs: vec![0; 8],
            icache: None,
            dcache: None,
            l2_cache: None,
            endian: Endian::SmallEndian,
        }
    }

    #[allow(unused)]
    pub fn load(&mut self, reg_index: usize, mem: &Memory, addr: usize) {
        if let Some(c) = &self.l2_cache {
            unimplemented!();
        } else {
            self.regs[reg_index] = mem.read_u32(addr).unwrap();
        }
    }

    #[allow(unused)]
    pub fn store(&self, reg_index: usize, mem: &mut Memory, addr: usize) {
        if let Some(c) = &self.l2_cache {
            unimplemented!();
        } else {
            mem.write_u32(addr, self.regs[reg_index]).unwrap();
        }
    }

    #[allow(unused)]
    pub fn add(&mut self, reg1_index: usize, reg2_index: usize, result_reg_index: usize) {
        if let Endian::SmallEndian = self.endian {
            self.regs[result_reg_index] = self.regs[reg1_index] + self.regs[reg2_index];
        } else {
            let mut reg1_data: u32 = 0;
            reg1_data += (self.regs[reg1_index] & 0x000000FF) << 24;
            reg1_data += (self.regs[reg1_index] & 0x0000FF00) << 8;
            reg1_data += (self.regs[reg1_index] & 0x00FF0000) >> 8;
            reg1_data += (self.regs[reg1_index] & 0xFF000000) >> 24;

            let mut reg2_data: u32 = 0;
            reg2_data += (self.regs[reg2_index] & 0x000000FF) << 24;
            reg2_data += (self.regs[reg2_index] & 0x0000FF00) << 8;
            reg2_data += (self.regs[reg2_index] & 0x00FF0000) >> 8;
            reg2_data += (self.regs[reg2_index] & 0xFF000000) >> 24;

            let mut result_reg_data: u32 = reg1_data + reg2_data;
            self.regs[result_reg_index] = 0;
            self.regs[result_reg_index] += (result_reg_data & 0x000000FF) << 24;
            self.regs[result_reg_index] += (result_reg_data & 0x0000FF00) << 8;
            self.regs[result_reg_index] += (result_reg_data & 0x00FF0000) >> 8;
            self.regs[result_reg_index] += (result_reg_data & 0xFF000000) >> 24;
        }
    }

    #[allow(unused)]
    pub fn sub(&mut self, reg1_index: usize, reg2_index: usize, result_reg_index: usize) {
        if let Endian::SmallEndian = self.endian {
            self.regs[result_reg_index] = self.regs[reg1_index] - self.regs[reg2_index];
        } else {
            let mut reg1_data: u32 = 0;
            reg1_data += (self.regs[reg1_index] & 0x000000FF) << 24;
            reg1_data += (self.regs[reg1_index] & 0x0000FF00) << 8;
            reg1_data += (self.regs[reg1_index] & 0x00FF0000) >> 8;
            reg1_data += (self.regs[reg1_index] & 0xFF000000) >> 24;

            let mut reg2_data: u32 = 0;
            reg2_data += (self.regs[reg2_index] & 0x000000FF) << 24;
            reg2_data += (self.regs[reg2_index] & 0x0000FF00) << 8;
            reg2_data += (self.regs[reg2_index] & 0x00FF0000) >> 8;
            reg2_data += (self.regs[reg2_index] & 0xFF000000) >> 24;

            let mut result_reg_data: u32 = reg1_data - reg2_data;
            self.regs[result_reg_index] = 0;
            self.regs[result_reg_index] += (result_reg_data & 0x000000FF) << 24;
            self.regs[result_reg_index] += (result_reg_data & 0x0000FF00) << 8;
            self.regs[result_reg_index] += (result_reg_data & 0x00FF0000) >> 8;
            self.regs[result_reg_index] += (result_reg_data & 0xFF000000) >> 24;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::memory::memory3::Memory;
    use crate::core::core1::Core;

    #[test]
    fn use_core() {
        const MEMORY_SIZE: usize = 1024;
        let mut mem: Memory = Memory::new(MEMORY_SIZE, 0x5A);
        let mut core: Core<u32> = Core::<u32>::default();
        core.load(0, &mem, 0);
        core.load(1, &mem, 4);
        core.add(0, 1, 2);
        core.store(2, &mut mem, 8);
        assert_eq!(core.regs[2], core.regs[0] + core.regs[1]);
    }
}