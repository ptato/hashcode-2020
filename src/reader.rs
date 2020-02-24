use std::fs;
use std::path::Path;
use std::str::FromStr;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Reader {
    source: Vec<char>,
    source_pos: usize,
}
#[allow(dead_code)]
impl Reader {
    pub fn from_file<P: AsRef<Path>>(filename: P) -> Self {
        let source = fs::read_to_string(filename).unwrap_or("".to_string());
        Self{ source: source.chars().collect(), source_pos: 0 }
    }
    fn skip_whitespace(&mut self) {
        loop {
            if self.source_pos >= self.source.len() { break; }
            if !self.source[self.source_pos].is_whitespace() { break; }
            self.source_pos += 1;
        }
    }
    pub fn next_word(&mut self) -> String {
        let mut result = String::new();
        self.skip_whitespace();
        loop {
            if self.source_pos >= self.source.len() { break; }
            if self.source[self.source_pos].is_whitespace() { break; }
            result.push(self.source[self.source_pos]);
            self.source_pos += 1;
        }
        result
    }
    pub fn next<T: FromStr>(&mut self) -> T {
        self.next_word().parse().ok().unwrap()
    }
    pub fn to_end_of_line(&mut self) -> String {
        let mut result = String::new();
        self.skip_whitespace();
        loop {
            if self.source_pos >= self.source.len() { break; }
            if self.source[self.source_pos] == '\n' || self.source[self.source_pos] == '\r' { break; }
            result.push(self.source[self.source_pos]);
            self.source_pos += 1;
        }
        result
    }
}
