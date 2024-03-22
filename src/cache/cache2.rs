
pub enum CacheLevel {
    Level1,
    Level2,
    Level3
}

impl Default for CacheLevel {
    fn default() -> Self {
        Self::Level1
    }
}

pub enum CacheType {
    ICache,
    DCache,
}

impl Default for CacheType {
    fn default() -> Self {
        Self::DCache
    }
}

#[derive(Default)]
pub struct Cache {
    level: CacheLevel,
    type_: CacheType,
    size: usize,  /* byte */
    line_count: usize,
    set_count: usize,
}

impl Cache {
    pub fn new(level: CacheLevel, type_: CacheType, size: usize, line_count: usize, set_count: usize) -> Self {
        Self {
            level,
            type_,
            size,
            line_count,
            set_count,
        }
    }
}