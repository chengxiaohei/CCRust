
#[derive(Debug, Clone, Copy)]
struct Word([u8; 4]);

impl Word {
    fn new() -> Word { Word([0;4]) }
    
}

#[derive(Debug)]
pub struct Line([Word; 4]);

impl Line {
    fn new() -> Line { Line([Word::new(); 4]) }
}

#[derive(Debug)]
pub struct Memory {
    byte_num: usize,
    word_num: usize,
    line_num: usize,
    pub lines: Vec<Line>,
}

impl Memory {
    pub fn new(bytes: usize) -> Memory {
        Memory {
            byte_num: bytes, 
            word_num: bytes/4, 
            line_num: bytes/(4*4),
            lines: Vec::with_capacity(bytes/(4*4)),
        }
    }

    pub fn init(&mut self) -> () {
        for _ in 0..self.line_num {
            self.lines.push(Line::new())
        }
    }

    pub fn size(&self) -> usize { self.byte_num }
    pub fn read_line(&self, index: usize) -> &Line { &self.lines[index] }
    pub fn write_line(&mut self, index: usize, line: Line) -> Result<(), &str> {
        if index >= self.line_num {
            return Err("overflow");
        }
        self.lines[index] = line;
        Ok(())
    }       
    pub fn read_word(&self, index: usize) {
        
    }

}