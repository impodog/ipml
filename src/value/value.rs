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

impl std::fmt::Debug for Functor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Functor({:x})", self as *const Functor as usize)
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Int(isize),
    Float(f64),
    Bool(bool),
    Str(String),
    List(VecDeque<ValueRc>),
    Functor(Functor),
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
