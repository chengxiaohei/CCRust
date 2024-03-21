mod memory;
mod common;
mod cpu;
mod cache;
mod tcm;

use common::Endian;
use memory::memory3::Memory;
use cpu::Cpu;
const MEMORY_SIZE: usize = 1024;

fn main() {
    println!("Hello, world!");
    let mut mem: Memory = Memory::new(MEMORY_SIZE, 0x5A);
    let mut cpu: Cpu = Cpu::default();
    cpu.read_mem(0, &mem, 0);
    cpu.read_mem(1, &mem, 4);
    cpu.add(0, 1, 2);
    cpu.write_mem(3, &mut mem, 8);
}
