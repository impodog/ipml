use crate::prelude::*;

impl Scope {
    pub(crate) fn to_scope(&mut self, token: &Token) -> Result<ScopeRc, RuntimeError> {
        match token {
            Token::Tag(name) => Ok(self.query_scope(name)),
            _ => {
                if let Token::Block(block) = token {
                    match block.decor() {
                        BlockDecorator::SubScope => return self.to_subscope(token),
                        _ => {}
                    }
                }
                let mut scope = Scope::new();
                scope.feed(token)?;
                Ok(rc_cell(scope))
            }
        }
    }

    pub(crate) fn to_subscope(&mut self, token: &Token) -> Result<ScopeRc, RuntimeError> {
        match token {
            Token::Tag(name) => Ok(self.query_scope(name)),
            _ => {
                if let Token::Block(block) = token {
                    match block.decor() {
                        BlockDecorator::IndepScope => return self.to_scope(token),
                        _ => {}
                    }
                }
                let scope = Scope::with_parent(ANONYMOUS.to_string(), self);
                scope.borrow_mut().feed(token)?;
                Ok(scope)
            }
        }
    }

    pub(crate) fn to_list(&mut self, block: &BlockVec) -> Result<ValueRc, RuntimeError> {
        let mut list = VecDeque::new();
        for token in block.data() {
            list.push_back(self.as_true_value(token)?);
        }
        Ok(rc_cell(Value::List(list)))
    }

    pub(crate) fn to_functor(&mut self, block: &BlockVec) -> Result<ValueRc, RuntimeError> {
        let token = Token::Block(block.clone());
        Ok(rc_cell(Value::Functor(Functor::new(move |scope| {
            scope.feed(&token)?;
            Ok(scope.query_value(&[RETURN.to_string()]))
        }))))
    }

    pub(crate) fn as_feedable(
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

    pub(crate) fn as_true_value(&mut self, token: &Token) -> Result<ValueRc, RuntimeError> {
        match token {
            Token::Value(value) => Ok(rc_cell(value.clone())),
            Token::Symbol(name) => Ok(self.query_value(name)),
            Token::Block(block) => {
                match block.decor() {
                    BlockDecorator::Functor => self.to_functor(block),
                    _ => self.to_list(block),
                }
            },
            _ => Err(RuntimeError::new(format!(
                "[Eval] Expected value-convertible type (including literal values, symbols, and blocks(aka lists)), but got {}",
                token
            ))),
        }
    }
}
