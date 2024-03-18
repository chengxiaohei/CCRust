#[derive(Debug)]
struct Cache {
    level: u8,
    size: u32,
}

impl Cache {
    pub fn new(level: u8, size: u32) -> Cache {
        Cache{level, size}
    }
}

#[derive(Debug)]
pub struct Cache1 {
    cache: Cache,
    
}

impl Cache1 {
    pub fn new(level: u8, size: u32) -> Cache1 {
        Cache1{cache: Cache::new(level, size)}
    }
}

pub struct Cache2 {

}