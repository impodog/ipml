use crate::prelude::*;

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Value::Null => write!(f, "null"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Str(s) => write!(f, "{}", s),
            Value::List(list) => {
                write!(f, "[")?;
                let mut first = true;
                for item in list.iter() {
                    if first {
                        first = false;
                    } else {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item.borrow())?;
                }
                write!(f, "]")
            }
            Value::Functor(functor) => write!(f, "{:?}", functor),
        }
    }
}
