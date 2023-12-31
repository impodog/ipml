use crate::prelude::*;

pub type ScopeRc = RcCell<Scope>;

#[derive(Debug, Clone, Copy)]
pub struct ScopeMode {
    pub filter: bool,
}

impl Default for ScopeMode {
    fn default() -> Self {
        Self { filter: false }
    }
}

#[derive(Debug)]
pub struct Scope {
    values: HashMap<String, ValueRc>,
    scopes: HashMap<String, ScopeRc>,

    // Don't do anything with this pointer, it's just a reference to the parent scope
    // If everything works correctly, this should never be dangling
    parent: *mut Scope,
    depth: usize,

    pub mode: ScopeMode,
}

impl Clone for Scope {
    fn clone(&self) -> Self {
        Self {
            values: self.values.clone(),
            scopes: self.scopes.clone(),
            parent: std::ptr::null_mut(),
            depth: self.depth,
            mode: self.mode,
        }
    }
}

impl Scope {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            scopes: HashMap::new(),
            parent: std::ptr::null_mut(),
            depth: 0,
            mode: Default::default(),
        }
    }

    pub fn with_parent(name: String, parent: &mut Scope) -> Result<ScopeRc, RuntimeError> {
        let child = rc_cell(Self::new());
        parent.link_child(&name, child.clone())?;
        Ok(child)
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

    pub fn query_value(&mut self, name: &[String]) -> Result<ValueRc, RuntimeError> {
        match name.len() {
            0 => Err(RuntimeError::new(format!(
                "[Scope] Expected at least one name, but got {}",
                name.len()
            ))),
            1 => Ok(self.touch_value(&name[0])),
            _ => Ok(self
                .query_scope(&name[0..name.len() - 1])?
                .borrow_mut()
                .touch_value(&name[name.len() - 1])),
        }
    }

    pub fn set_value(&mut self, name: &[String], value: ValueRc) -> Result<(), RuntimeError> {
        match name.len() {
            0 => Ok(()),
            1 => {
                self.values.insert(name[0].clone(), value);
                Ok(())
            }
            _ => {
                self.query_scope(&name[0..name.len() - 1])?
                    .try_borrow_mut()
                    .map_err(|_| {
                        RuntimeError::new(format!(
                            "[Scope] Setting the value of {} and so making it mutable, is illegal",
                            name.join(".")
                        ))
                    })?
                    .values
                    .insert(name[name.len() - 1].clone(), value);
                Ok(())
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

    fn touch_scope(&mut self, name: &str) -> Result<ScopeRc, RuntimeError> {
        if let Some(scope) = self.scopes.get(name) {
            Ok(scope.clone())
        } else {
            let scope = Self::with_parent(name.to_string(), self)?;
            Ok(scope)
        }
    }

    fn query_scope_help(
        &mut self,
        name: &[String],
        is_begin: bool,
    ) -> Result<ScopeRc, RuntimeError> {
        match name.len() {
            0 => Err(RuntimeError::new(format!(
                "[Scope] Expected at least one name, but got {}",
                name.len()
            ))),
            1 => {
                if let Some(scope) = self.get_scope(&name[0]) {
                    Ok(scope)
                } else {
                    self.touch_scope(&name[0])
                }
            }
            _ => {
                if is_begin {
                    if let Some(scope) = self.get_scope(&name[0]) {
                        scope.borrow_mut().query_scope_help(&name[1..], false)
                    } else {
                        self.touch_scope(&name[0])?
                            .borrow_mut()
                            .query_scope_help(&name[1..], false)
                    }
                } else {
                    self.touch_scope(&name[0])?
                        .borrow_mut()
                        .query_scope_help(&name[1..], false)
                }
            }
        }
    }

    pub fn query_scope(&mut self, name: &[String]) -> Result<ScopeRc, RuntimeError> {
        self.query_scope_help(name, true)
    }

    pub fn set_scope(&mut self, name: &[String], scope: ScopeRc) -> Result<(), RuntimeError> {
        match name.len() {
            0 => Ok(()),
            1 => self.link_child(&name[0], scope),
            _ => self
                .touch_scope(&name[0])?
                .borrow_mut()
                .set_scope(&name[1..], scope),
        }
    }

    pub fn link_scope(&mut self, name: &[String], scope: ScopeRc) -> Result<(), RuntimeError> {
        match name.len() {
            0 => Ok(()),
            1 => {
                if scope.borrow().is_parent_of(self) {
                    return Err(RuntimeError::new(format!(
                        "[Scope] {} and {} are related and cannot be linked, otherwise it will cause memory leaks",
                        self,
                        scope.borrow()
                    )));
                }
                self.scopes.insert(name[0].clone(), scope);
                Ok(())
            }
            _ => self
                .touch_scope(&name[0])?
                .borrow_mut()
                .link_scope(&name[1..], scope),
        }
    }

    pub fn insert_functor(
        &mut self,
        name: &[String],
        functor: impl FunctorInner + 'static,
    ) -> Result<(), RuntimeError> {
        match name.len() {
            0 => {}
            1 => {
                self.values.insert(
                    name[0].clone(),
                    rc_cell(Value::Functor(Functor::new(functor))),
                );
            }
            _ => {
                self.touch_scope(&name[0])?
                    .borrow_mut()
                    .insert_functor(&name[1..], functor)?;
            }
        }
        Ok(())
    }

    pub fn get_parent(&self) -> Option<&mut Scope> {
        if self.parent.is_null() {
            None
        } else {
            Some(unsafe { &mut *self.parent })
        }
    }

    // This function checks whether the two scopes are related, so as to prevent memory leaks
    pub fn is_parent_of(&self, other: &Scope) -> bool {
        if self.depth >= other.depth {
            return false;
        }
        let mut parent = other.get_parent();
        while let Some(p) = parent {
            if p.depth == self.depth {
                return p as *const Scope == self as *const Scope;
            }
            parent = p.get_parent();
        }
        false
    }

    pub fn link_child(&mut self, name: &str, child: ScopeRc) -> Result<(), RuntimeError> {
        if child.borrow().is_parent_of(self) {
            return Err(RuntimeError::new(format!(
                "[Scope] {} and {} are related and cannot be linked, otherwise it will cause memory leaks",
                self,
                child.borrow()
            )));
        } else if child.borrow().parent.is_null() {
            child.borrow_mut().parent = self;
            child.borrow_mut().depth = self.depth + 1;
            child.borrow_mut().mode = self.mode;
            self.scopes.insert(name.to_string(), child);
            Ok(())
        } else {
            Err(RuntimeError::new(format!(
                "[Scope] {} already has a parent and cannot be linked",
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

    pub(crate) fn cleanup_temp(&mut self) {
        if self.mode.filter {
            let _ = self.scopes.remove(ANONYMOUS);
            let _ = self.values.remove(RETURN);
        }
    }

    pub fn cleanup(&mut self) {
        if self.mode.filter {
            for (_, v) in self.scopes.iter() {
                v.borrow_mut().cleanup();
            }

            let mut to_remove = Vec::new();
            for (k, v) in self.values.iter() {
                let mut flag = false;
                match &*v.borrow() {
                    Value::Functor(_) => flag = true,
                    _ => {}
                }
                if let Some('_') = k.chars().nth(0) {
                    flag = true;
                }
                if flag {
                    to_remove.push(k.clone());
                }
            }
            for k in to_remove {
                self.values.remove(&k);
            }
        }
    }
}
