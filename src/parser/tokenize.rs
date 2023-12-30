use super::*;
use crate::prelude::*;

fn parse_slice(result: &mut BlockVec, slice: &[Token]) -> usize {
    let mut i = 0;
    while i < slice.len() {
        match slice[i] {
            Token::Operator('(') => {
                let mut block = BlockVec::new();
                i += parse_slice(&mut block, &slice[i + 1..]);
                result.push(Token::Block(block));
            }
            Token::Operator(')') => {
                return i + 1;
            }
            _ => {
                result.push(slice[i].clone());
                i += 1;
            }
        }
    }
    slice.len()
}

pub trait Tokenize {
    fn tokenize(&mut self) -> Option<Result<Token, SyntaxError>>;

    fn parse(&mut self) -> Result<Token, SyntaxError> {
        let mut stack = BlockVec::new();
        while let Some(token) = self.tokenize() {
            stack.push(token?);
        }
        let mut block = BlockVec::new();
        parse_slice(&mut block, stack.data());
        Ok(Token::Block(block))
    }
}

impl Parser {
    fn escape_char(c: char) -> Result<char, SyntaxError> {
        match c {
            'n' => Ok('\n'),
            'r' => Ok('\r'),
            't' => Ok('\t'),
            '\\' => Ok('\\'),
            '"' => Ok('"'),
            _ => Err(SyntaxError::new(
                0,
                0,
                format!("Unknown escape character {}", c),
            )),
        }
    }
    fn next_number(&mut self) -> Result<Token, SyntaxError> {
        let mut num = String::new();
        let mut dot = false;
        num.push(self.next().unwrap());
        while let Some(c) = self.peek() {
            if c.is_digit(10) {
                num.push(c);
                self.next();
            } else if c == '.' {
                if dot {
                    return Err(self.error(format!("Unexpected float number {}{}", num, c)));
                } else {
                    dot = true;
                    num.push(c);
                    self.next();
                }
            } else {
                break;
            }
        }
        if dot {
            Ok(Token::Value(Value::Float(num.parse().unwrap())))
        } else {
            Ok(Token::Value(Value::Int(num.parse().unwrap())))
        }
    }

    fn next_string(&mut self) -> Result<Token, SyntaxError> {
        let mut string = String::new();
        let mut escape = false;
        self.next();
        while let Some(c) = self.peek() {
            if escape {
                string.push(Self::escape_char(c)?);
                escape = false;
            } else if c == '\\' {
                escape = true;
            } else if c == '"' {
                self.next();
                break;
            } else {
                string.push(c);
            }
            self.next();
        }
        Ok(Token::Value(Value::Str(string)))
    }

    fn next_tag(&mut self) -> Result<Token, SyntaxError> {
        let mut tag = vec![String::new()];
        self.next();
        while let Some(c) = self.peek() {
            if c == ']' {
                self.next();
                break;
            } else if c == '.' {
                if tag.last().unwrap().is_empty() {
                    return Err(self.error(format!("Empty tag section")));
                }
                tag.push(String::new());
            } else {
                tag.last_mut().unwrap().push(c);
            }
            self.next();
        }
        if tag.last().unwrap().is_empty() {
            return Err(self.error(format!("Empty tag section")));
        }
        Ok(Token::Tag(tag))
    }

    fn next_ident(&mut self) -> Result<Token, SyntaxError> {
        let mut symbol = vec![String::new()];
        while let Some(c) = self.peek() {
            match c {
                'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                    symbol.last_mut().unwrap().push(c);
                    self.next();
                }
                '.' => {
                    if symbol.last().unwrap().is_empty() {
                        return Err(self.error(format!("Empty symbol section")));
                    }
                    symbol.push(String::new());
                    self.next();
                }
                _ => break,
            }
        }
        if symbol.last().unwrap().is_empty() {
            return Err(self.error(format!("Empty symbol section")));
        }
        if symbol.len() == 1 {
            match symbol.last().unwrap().as_str() {
                "true" => Ok(Token::Value(Value::Bool(true))),
                "false" => Ok(Token::Value(Value::Bool(false))),
                "null" => Ok(Token::Value(Value::Null)),
                _ => Ok(Token::Symbol(symbol)),
            }
        } else {
            Ok(Token::Symbol(symbol))
        }
    }

    fn next_operator(&mut self) -> Result<Token, SyntaxError> {
        Ok(Token::Operator(self.next().unwrap()))
    }
}

impl Tokenize for Parser {
    fn tokenize(&mut self) -> Option<Result<Token, SyntaxError>> {
        if let Some(mut c) = self.peek() {
            while c.is_whitespace() {
                self.next();
                c = self.peek()?;
            }
            match c {
                '0'..='9' | '-' => Some(self.next_number()),
                '"' => Some(self.next_string()),
                '[' => Some(self.next_tag()),
                'a'..='z' | 'A'..='Z' | '_' => Some(self.next_ident()),
                _ => Some(self.next_operator()),
            }
        } else {
            None
        }
    }
}
