pub mod memory2;

use crate::memory2::{Endian, Memory};
const MEMORY_SIZE: usize = 1024;

fn main() {
    let mut mem: Memory = Memory::new(MEMORY_SIZE, 0x5A, Endian::BigEndian);
    println!("{:?}", &mem.read_u8(1).unwrap());
    let _ = mem.write_u8(1, 32);
    println!("{:?}", &mem.read_u8(1).unwrap());
    println!("Hello, world!");
}
