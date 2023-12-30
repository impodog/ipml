use crate::prelude::*;

pub type ScopeCell = RefCell<Scope>;
pub type ScopeRc = Rc<ScopeCell>;

#[derive(Debug, Clone)]
pub struct Scope {
    values: HashMap<String, ValueRc>,
    scopes: HashMap<String, ScopeRc>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            scopes: HashMap::new(),
        }
    }

    pub fn with_parent(name: String, parent: &mut Scope) -> Self {
        parent.scopes.insert(name, rc_cell(Self::new()));
        Self {
            values: HashMap::new(),
            scopes: HashMap::new(),
        }
    }

    fn touch_value(&mut self, name: &str) -> ValueRc {
        if let Some(value) = self.values.get(name) {
            value.clone()
        } else {
            let value = rc_cell(Value::Null);
            self.values.insert(name.to_string(), value.clone());
            value
        }
    }

    pub fn query_value(&mut self, name: &[String]) -> ValueRc {
        match name.len() {
            0 => rc_cell(Value::Null),
            1 => self.touch_value(&name[0]),
            _ => self
                .touch_scope(&name[0])
                .borrow_mut()
                .query_value(&name[1..]),
        }
    }

    pub fn set_value(&mut self, name: &[String], value: ValueRc) {
        match name.len() {
            0 => {}
            1 => {
                self.values.insert(name[0].clone(), value);
            }
            _ => {
                self.touch_scope(&name[0])
                    .borrow_mut()
                    .set_value(&name[1..], value);
            }
        }
    }

    fn touch_scope(&mut self, name: &str) -> ScopeRc {
        if let Some(scope) = self.scopes.get(name) {
            scope.clone()
        } else {
            let scope = rc_cell(Self::with_parent(name.to_string(), self));
            self.scopes.insert(name.to_string(), scope.clone());
            scope
        }
    }

    pub fn query_scope(&mut self, name: &[String]) -> ScopeRc {
        match name.len() {
            0 => rc_cell(self.clone()),
            1 => self.touch_scope(&name[0]),
            _ => self
                .touch_scope(&name[0])
                .borrow_mut()
                .query_scope(&name[1..]),
        }
    }

    pub fn set_scope(&mut self, name: &[String], scope: ScopeRc) {
        match name.len() {
            0 => {}
            1 => {
                self.scopes.insert(name[0].clone(), scope);
            }
            _ => {
                self.touch_scope(&name[0])
                    .borrow_mut()
                    .set_scope(&name[1..], scope);
            }
        }
    }
}
