use std::fs;
use std::path::Path;
use std::str::FromStr;
use std::fmt::Debug;

#[derive(Debug)]
pub struct FileReader {
    source: Vec<char>,
    source_pos: usize,
}
#[allow(dead_code)]
impl FileReader {
    pub fn from_file<P: AsRef<Path>>(filename: P) -> FileReader {
        let source = fs::read_to_string(filename).unwrap_or("".to_string());
        FileReader{ source: source.chars().collect(), source_pos: 0 }
    }
    fn skip_whitespace(&mut self) {
        loop {
            if self.source_pos >= self.source.len() { break; }
            if !self.source[self.source_pos].is_whitespace() { break; }
            self.source_pos += 1;
        }
    }
    pub fn next_word(&mut self) -> Option<String> {
        let mut result = String::new();
        self.skip_whitespace();
        loop {
            if self.source_pos >= self.source.len() { break; }
            if self.source[self.source_pos].is_whitespace() { break; }
            result.push(self.source[self.source_pos]);
            self.source_pos += 1;
        }
        if result == "" { None } else { Some(result) }
    }
    pub fn next<T: FromStr>(&mut self) -> Option<T> {
        let word = self.next_word();
        word.and_then(|t| t.parse().ok())
    }
    pub fn to_end_of_line(&mut self) -> Option<String> {
        let mut result = String::new();
        self.skip_whitespace();
        loop {
            if self.source_pos >= self.source.len() { break; }
            if self.source[self.source_pos] == '\n' || self.source[self.source_pos] == '\r' { break; }
            result.push(self.source[self.source_pos]);
            self.source_pos += 1;
        }
        if result == "" { None } else { Some(result) }
    }
}
