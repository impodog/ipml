use crate::prelude::*;

macro_rules! not_implemented {
    ($self: ident, $name: ident) => {
        Err(RuntimeError::new(format!(
            "{} is not implemented for {}::{}",
            stringify!($name),
            std::any::type_name::<Self>(),
            $self
        )))
    };
}

pub trait Feed
where
    Self: Debug + Display,
{
    fn feed(&mut self, _token: &Token) -> Result<(), RuntimeError> {
        not_implemented!(self, feed)
    }

    fn call(&mut self, _scope: &mut Scope) -> Result<ValueRc, RuntimeError> {
        not_implemented!(self, call)
    }
}

impl Feed for Scope {
    fn feed(&mut self, token: &Token) -> Result<(), RuntimeError> {
        match token {
            Token::Block(block) => {
                if block.len() % 3 != 0 {
                    return Err(RuntimeError::new(format!(
                        "[Execute] Executed block length must be a multiple of 3, but got {}",
                        block.len()
                    )));
                }
                let mut i = 0;
                while i < block.len() {
                    let lhs = &block.data()[i];
                    let op = match &block.data()[i + 1] {
                        Token::Operator(op) => op,
                        _ => {
                            return Err(RuntimeError::new(format!(
                                "[Execute] Expected operator at block position {}, but got {}",
                                i + 1,
                                &block.data()[i + 1]
                            )))
                        }
                    };
                    let rhs = &block.data()[i + 2];
                    let result = self.execute(lhs, *op, rhs)?;
                    if match &*result.borrow() {
                        Value::Null => false,
                        _ => true,
                    } {
                        self.set_value(&[RETURN.to_string()], result)?;
                    }
                    i += 3;
                }
                self.cleanup();
                Ok(())
            }
            _ => Err(RuntimeError::new(format!(
                "[Scope] Scope can only be fed with blocks instead of {}",
                token
            ))),
        }
    }
}
