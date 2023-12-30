use crate::prelude::*;

pub type ValueCell = RefCell<Value>;
pub type ValueRc = Rc<ValueCell>;

#[derive(Debug, Clone)]
pub enum Value {
    Int(isize),
    Float(f64),
    Bool(bool),
    Str(String),
    List(VecDeque<Value>),
    Null,
}

#[derive(Debug, Clone)]
pub enum Token {
    Value(Value),
    Symbol(Vec<String>),
    Tag(Vec<String>),
    Block(BlockVec),
    Operator(char),
}

#[derive(Debug, Clone)]
pub struct BlockVec(Vec<Token>);

impl BlockVec {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, token: Token) {
        self.0.push(token);
    }

    pub fn pop(&mut self) -> Option<Token> {
        self.0.pop()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline(always)]
    pub fn data(&self) -> &Vec<Token> {
        &self.0
    }
}
