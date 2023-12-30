use crate::prelude::*;

pub type ScopeRc = RcCell<Scope>;

#[derive(Debug)]
pub struct Scope {
    values: HashMap<String, ValueRc>,
    scopes: HashMap<String, ScopeRc>,

    // Don't do anything with this pointer, it's just a reference to the parent scope
    // If everything works correctly, this should never be dangling
    parent: *mut Scope,
}

impl Clone for Scope {
    fn clone(&self) -> Self {
        Self {
            values: self.values.clone(),
            scopes: self.scopes.clone(),
            parent: std::ptr::null_mut(),
        }
    }
}

impl Scope {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            scopes: HashMap::new(),
            parent: std::ptr::null_mut(),
        }
    }

    pub fn with_parent(name: String, parent: &mut Scope) -> ScopeRc {
        let child = rc_cell(Self {
            values: HashMap::new(),
            scopes: HashMap::new(),
            parent,
        });
        parent.scopes.insert(name, child.clone());
        child
    }

    fn get_value(&self, name: &str) -> Option<ValueRc> {
        if let Some(value) = self.values.get(name) {
            Some(value.clone())
        } else if let Some(parent) = self.get_parent() {
            parent.get_value(name)
        } else {
            None
        }
    }

    fn touch_value(&mut self, name: &str) -> ValueRc {
        if let Some(value) = self.values.get(name) {
            value.clone()
        } else {
            if let Some(value) = self.get_value(name) {
                value
            } else {
                let value = rc_cell(Value::Null);
                self.values.insert(name.to_string(), value.clone());
                value
            }
        }
    }

    pub fn query_value(&mut self, name: &[String]) -> ValueRc {
        match name.len() {
            0 => rc_cell(Value::Null),
            1 => self.touch_value(&name[0]),
            _ => self
                .query_scope(&name[0..name.len() - 1])
                .borrow_mut()
                .touch_value(&name[name.len() - 1]),
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

    fn get_scope(&self, name: &str) -> Option<ScopeRc> {
        if let Some(scope) = self.scopes.get(name) {
            Some(scope.clone())
        } else if let Some(parent) = self.get_parent() {
            parent.get_scope(name)
        } else {
            None
        }
    }

    fn touch_scope(&mut self, name: &str) -> ScopeRc {
        if let Some(scope) = self.scopes.get(name) {
            scope.clone()
        } else {
            let scope = Self::with_parent(name.to_string(), self);
            scope
        }
    }

    fn query_scope_help(&mut self, name: &[String], is_begin: bool) -> ScopeRc {
        match name.len() {
            0 => rc_cell(self.clone()),
            1 => {
                if let Some(scope) = self.get_scope(&name[0]) {
                    scope
                } else {
                    self.touch_scope(&name[0])
                }
            }
            _ => {
                if is_begin {
                    if let Some(scope) = self.get_scope(&name[0]) {
                        scope.borrow_mut().query_scope_help(&name[1..], false)
                    } else {
                        self.touch_scope(&name[0])
                            .borrow_mut()
                            .query_scope_help(&name[1..], false)
                    }
                } else {
                    self.touch_scope(&name[0])
                        .borrow_mut()
                        .query_scope_help(&name[1..], false)
                }
            }
        }
    }

    pub fn query_scope(&mut self, name: &[String]) -> ScopeRc {
        let scope = self.query_scope_help(name, true);
        scope
    }

    pub fn set_scope(&mut self, name: &[String], scope: ScopeRc) -> Result<(), RuntimeError> {
        match name.len() {
            0 => Ok(()),
            1 => self.link_child(&name[0], scope),
            _ => self
                .touch_scope(&name[0])
                .borrow_mut()
                .set_scope(&name[1..], scope),
        }
    }

    pub fn insert_functor(&mut self, name: &[String], functor: impl FunctorInner + 'static) {
        match name.len() {
            0 => {}
            1 => {
                self.values.insert(
                    name[0].clone(),
                    rc_cell(Value::Functor(Functor::new(functor))),
                );
            }
            _ => {
                self.touch_scope(&name[0])
                    .borrow_mut()
                    .insert_functor(&name[1..], functor);
            }
        }
    }

    pub fn get_parent(&self) -> Option<&mut Scope> {
        if self.parent.is_null() {
            None
        } else {
            Some(unsafe { &mut *self.parent })
        }
    }

    pub fn link_child(&mut self, name: &str, child: ScopeRc) -> Result<(), RuntimeError> {
        if child.borrow().parent.is_null() {
            child.borrow_mut().parent = self;
            self.scopes.insert(name.to_string(), child);
            Ok(())
        } else {
            Err(RuntimeError::new(format!(
                "[Scope] Scope {} already has a parent and cannot be linked",
                child.borrow()
            )))
        }
    }

    pub(crate) fn get_values(&self) -> &HashMap<String, ValueRc> {
        &self.values
    }

    pub(crate) fn get_scopes(&self) -> &HashMap<String, ScopeRc> {
        &self.scopes
    }
}
