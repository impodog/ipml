use crate::prelude::*;

pub type ValueRc = RcCell<Value>;

pub trait FunctorInner: Fn(&mut Scope) -> Result<ValueRc, RuntimeError> {}

impl<F: 'static + Fn(&mut Scope) -> Result<ValueRc, RuntimeError>> FunctorInner for F {}

#[derive(Clone)]
pub struct Functor(RcCell<dyn FunctorInner>);

impl Functor {
    pub fn new<F: 'static + FunctorInner>(f: F) -> Self {
        Self(rc_cell(f))
    }

    pub fn call(&self, scope: &mut Scope) -> Result<ValueRc, RuntimeError> {
        self.0.borrow()(scope)
    }
}

impl PartialEq for Functor {
    fn eq(&self, other: &Self) -> bool {
        return self as *const Self == other as *const Self;
    }
}

impl PartialOrd for Functor {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return (self as *const Self).partial_cmp(&(other as *const Self));
    }
}

impl std::fmt::Debug for Functor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Functor({:x})", self as *const Functor as usize)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Int(isize),
    Float(f64),
    Bool(bool),
    Str(String),
    List(VecDeque<ValueRc>),
    Functor(Functor),
    Null,
}

impl Value {
    pub fn to_bool(&self) -> bool {
        match self {
            Value::Int(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::Bool(b) => *b,
            Value::Str(s) => !s.is_empty(),
            Value::List(l) => !l.is_empty(),
            Value::Functor(_) => true,
            Value::Null => false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Token {
    Value(Value),
    Symbol(Vec<String>),
    Tag(Vec<String>),
    Block(BlockVec),
    Operator(char),
    Decorator(BlockDecorator),
}

#[derive(Debug, Clone)]
pub struct BlockVec(Vec<Token>, BlockDecorator);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockDecorator {
    None,
    Functor,
    SubScope,
    IndepScope,
}

impl BlockVec {
    pub fn new() -> Self {
        Self(Vec::new(), BlockDecorator::None)
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

    pub fn set_decor(&mut self, decor: BlockDecorator) {
        self.1 = decor;
    }

    #[inline(always)]
    pub fn data(&self) -> &Vec<Token> {
        &self.0
    }

    #[inline(always)]
    pub fn decor(&self) -> &BlockDecorator {
        &self.1
    }
}
