use crate::prelude::*;

pub struct SyntaxError {
    line: usize,
    cursor: usize,
    msg: String,
}

impl SyntaxError {
    pub fn new(line: usize, cursor: usize, msg: String) -> Self {
        Self { line, cursor, msg }
    }
}

impl Error for SyntaxError {}

impl Debug for SyntaxError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        Display::fmt(self, f)
    }
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "IPML Syntax Error occurred at line {}, cursor {}: {}",
            self.line, self.cursor, self.msg
        )
    }
}

pub struct RuntimeError {
    msg: String,
}

impl RuntimeError {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}

impl Error for RuntimeError {}

impl Debug for RuntimeError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        Display::fmt(self, f)
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.msg)
    }
}
