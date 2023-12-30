use crate::prelude::*;

impl Display for Scope {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Scope {{")?;
        let mut first = true;
        for (name, value) in self.get_values().iter() {
            if first {
                first = false;
            } else {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", name, value.borrow())?;
        }
        for (name, scope) in self.get_scopes().iter() {
            if first {
                first = false;
            } else {
                write!(f, ", ")?;
            }
            write!(f, "[{}]: {}", name, scope.borrow())?;
        }
        write!(f, "}}")
    }
}
