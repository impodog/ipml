use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Parser {
    str: String,
    pos: usize,
    line: usize,
    prev_cursor: usize,
    cursor: usize,
}

impl Parser {
    pub fn new(str: String) -> Self {
        Self {
            str,
            pos: 0,
            line: 1,
            prev_cursor: 1,
            cursor: 1,
        }
    }

    pub fn next(&mut self) -> Option<char> {
        if self.pos < self.str.len() {
            let c = self.str.chars().nth(self.pos);
            self.pos += 1;

            self.prev_cursor = self.cursor;
            if c == Some('\n') {
                self.line += 1;
                self.cursor = 1;
            } else {
                self.cursor += 1;
            }

            c
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.str.chars().nth(self.pos)
    }

    pub fn back(&mut self) {
        self.pos -= 1;
        self.cursor = self.prev_cursor;
        if self.str.chars().nth(self.pos) == Some('\n') {
            self.line -= 1;
        }
    }

    pub fn error(&self, msg: String) -> SyntaxError {
        SyntaxError::new(self.line, self.cursor, msg)
    }
}
