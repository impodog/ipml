use crate::prelude::*;

impl Feed for Value {
    fn call(&mut self, _scope: &mut Scope) -> Result<ValueRc, RuntimeError> {
        match self {
            Value::Functor(functor) => functor.call(_scope),
            _ => Err(RuntimeError::new(format!(
                "[Feed] Expected functor, but got {}",
                self
            ))),
        }
    }
}
