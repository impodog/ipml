use crate::prelude::*;

impl Scope {
    fn do_feed(&mut self, lhs: &Token, rhs: &Token) -> Result<ValueRc, RuntimeError> {
        self.as_feedable(lhs)?.borrow_mut().feed(rhs)?;
        Ok(rc_cell(Value::Null))
    }

    fn do_assign(&mut self, lhs: &Token, rhs: &Token) -> Result<(), RuntimeError> {
        match lhs {
            Token::Symbol(name) => {
                let value = self.as_true_value(rhs)?;
                self.set_value(name, value)?;
                Ok(())
            }
            Token::Tag(name) => {
                let scope = match rhs {
                    Token::Tag(tag) => rc_clone(self.query_scope(tag)),
                    Token::Block(_) => self.to_scope(rhs)?,
                    _ => {
                        return Err(RuntimeError::new(format!(
                            "[Assign] Expected tag or block at right hand side, but got {}",
                            rhs
                        )))
                    }
                };
                self.set_scope(name, scope)?;
                Ok(())
            }
            _ => Err(RuntimeError::new(format!(
                "[Assign] Expected symbol or tag at left hand side, but got {}",
                lhs
            ))),
        }
    }

    fn do_call(&mut self, lhs: &Token, rhs: &Token) -> Result<ValueRc, RuntimeError> {
        self.as_feedable(lhs)?
            .borrow_mut()
            .call(&mut *self.to_subscope(rhs)?.borrow_mut())
            .map_err(|e| RuntimeError::new(format!("[Call] When calling {},\n{}", lhs, e)))
    }

    pub(super) fn execute(
        &mut self,
        lhs: &Token,
        op: char,
        rhs: &Token,
    ) -> Result<ValueRc, RuntimeError> {
        match op {
            ':' => self.do_feed(lhs, rhs),
            '=' => {
                self.do_assign(lhs, rhs)?;
                Ok(rc_cell(Value::Null))
            }
            '!' => self.do_call(lhs, rhs),
            _ => Err(RuntimeError::new(format!(
                "[Execute] Operator \'{}\' is not supported",
                op
            ))),
        }
    }
}
