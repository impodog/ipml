use crate::prelude::*;

impl Scope {
    pub(super) fn to_scope(&mut self, block: &Token) -> Result<ScopeRc, RuntimeError> {
        match block {
            Token::Tag(name) => Ok(self.query_scope(name)),
            _ => {
                let mut scope = Scope::new();
                scope.feed(block)?;
                Ok(rc_cell(scope))
            }
        }
    }

    pub(super) fn to_subscope(&mut self, block: &Token) -> Result<ScopeRc, RuntimeError> {
        match block {
            Token::Tag(name) => Ok(self.query_scope(name)),
            _ => {
                let scope = Scope::with_parent("[anonymous]".to_string(), self);
                scope.borrow_mut().feed(block)?;
                Ok(scope)
            }
        }
    }

    pub(super) fn to_list(&mut self, block: &BlockVec) -> Result<ValueRc, RuntimeError> {
        let mut list = VecDeque::new();
        for token in block.data() {
            list.push_back(self.as_true_value(token)?);
        }
        Ok(rc_cell(Value::List(list)))
    }

    pub(super) fn as_feedable(
        &mut self,
        token: &Token,
    ) -> Result<Rc<RefCell<dyn Feed>>, RuntimeError> {
        match token {
            Token::Symbol(name) => Ok(self.query_value(name)),
            Token::Tag(name) => Ok(self.query_scope(name)),
            Token::Block(_block) => Ok(self.to_scope(token)?),
            _ => Err(RuntimeError::new(format!(
                "[Feed] Expected feedable type (including symbols, tags, and blocks(aka implicit scopes)), but got {}",
                token
            ))),
        }
    }

    pub(super) fn as_true_value(&mut self, token: &Token) -> Result<ValueRc, RuntimeError> {
        match token {
            Token::Value(value) => Ok(rc_cell(value.clone())),
            Token::Symbol(name) => Ok(self.query_value(name)),
            Token::Block(block) => Ok(self.to_list(block)?),
            _ => Err(RuntimeError::new(format!(
                "[RhsEval] Expected value-convertible type (including literal values, symbols, and blocks(aka lists)), but got {}",
                token
            ))),
        }
    }
}
