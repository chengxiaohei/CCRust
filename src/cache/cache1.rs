struct CacheState {
    modified: bool,
    exclusived: bool,
    shared: bool,
    invalid: bool,
}

impl Default for CacheState {
    fn default() -> Self {
        Self {
            modified: false,
            exclusived: false,
            shared: false,
            invalid: true,
        }
    }
}
struct CacheLine {
    index: usize,
    state: CacheState,
    byte_num: usize,
    bytes: Vec<u8>,
}

impl CacheLine {
    fn new(index: usize, state: CacheState, word_num: usize) -> Self {
        Self {
            index,
            state,
            byte_num: word_num * 4,
            bytes: vec![0; word_num * 4],
        }
    }
}

pub struct Cache {
    line_num: usize,
    lines: Vec<CacheLine>,
    total_size: usize
}

impl Cache {
    pub fn new(line_num: usize, words_per_line: usize) -> Self {
        let mut cache: Self = Self {
            line_num,
            lines: Vec::with_capacity(line_num),
            total_size: line_num * words_per_line * 4,
        };
        for index in 0..line_num {
            cache.lines.push(CacheLine::new(index, CacheState::default(), words_per_line));
        }
        cache
    }
}


#[cfg(test)]
mod tests {

}