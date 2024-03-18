mod cpu;
mod cache;
mod memory;

use crate::cache::Cache1;
use crate::memory::Memory;

fn main() {
    let cache: Cache1 = Cache1::new(1,1);
    let mut mem: Memory = Memory::new(1028);
    mem.init();

    println!("{:?}", cache);
    println!("{:?}", &mem.lines[1]);
    println!("Hello, world!");
}
