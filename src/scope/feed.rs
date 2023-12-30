use crate::prelude::*;

macro_rules! not_implemented {
    ($self: ident, $name: ident) => {
        Err(RuntimeError::new(format!(
            "{} is not implemented for {:?}",
            stringify!($name),
            $self
        )))
    };
}

pub trait Feed
where
    Self: std::fmt::Debug,
{
    fn feed(&mut self, _token: &Token) -> Result<ValueRc, RuntimeError> {
        not_implemented!(self, feed)
    }
}

impl Scope {
    fn assign(&mut self, lhs: &Token, rhs: &Token) -> Result<(), RuntimeError> {
        match lhs {
            Token::Symbol(name) => {
                let value = match rhs {
                    Token::Value(value) => value.clone(),
                    _ => {
                        return Err(RuntimeError::new(format!(
                            "[Assign] Expected value at right hand side, but got {:?}",
                            rhs
                        )))
                    }
                };
                self.set_value(name, rc_cell(value));
                Ok(())
            }
            Token::Tag(name) => {
                let scope = match rhs {
                    Token::Tag(tag) => {
                        let mut scope = Scope::new();
                        scope.set_value(tag, rc_cell(Value::Null));
                        rc_cell(scope)
                    }
                    Token::Block(_) => {
                        let mut scope = Scope::new();
                        scope.feed(rhs)?;
                        rc_cell(scope)
                    }
                    _ => {
                        return Err(RuntimeError::new(format!(
                            "[Assign] Expected tag or block at right hand side, but got {:?}",
                            rhs
                        )))
                    }
                };
                self.set_scope(name, scope);
                Ok(())
            }
            _ => Err(RuntimeError::new(format!(
                "[Assign] Expected symbol or tag at left hand side, but got {:?}",
                lhs
            ))),
        }
    }

    fn execute(&mut self, lhs: &Token, op: char, rhs: &Token) -> Result<ValueRc, RuntimeError> {
        match op {
            '=' => {
                self.assign(lhs, rhs)?;
                Ok(rc_cell(Value::Null))
            }
            _ => Err(RuntimeError::new(format!(
                "[Scope] Operator \'{}\' is not supported",
                op
            ))),
        }
    }
}

impl Feed for Scope {
    fn feed(&mut self, token: &Token) -> Result<ValueRc, RuntimeError> {
        match token {
            Token::Block(block) => {
                if block.len() % 3 != 0 {
                    return Err(RuntimeError::new(format!(
                        "[Scope] Executed block length must be a multiple of 3, but got {}",
                        block.len()
                    )));
                }
                let mut i = 0;
                let mut result = rc_cell(Value::Null);
                while i < block.len() {
                    let lhs = &block.data()[i];
                    let op = match &block.data()[i + 1] {
                        Token::Operator(op) => op,
                        _ => {
                            return Err(RuntimeError::new(format!(
                                "[Scope] Expected operator at block position {}, but got {:?}",
                                i + 1,
                                &block.data()[i + 1]
                            )))
                        }
                    };
                    let rhs = &block.data()[i + 2];
                    result = self.execute(lhs, *op, rhs)?;
                    i += 3;
                }
                Ok(result)
            }
            _ => Err(RuntimeError::new(format!(
                "[Scope] Scope can only be fed with blocks instead of {:?}",
                token
            ))),
        }
    }
}
