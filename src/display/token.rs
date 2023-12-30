use crate::prelude::*;

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "T(")?;
        match self {
            Token::Value(value) => write!(f, "{}", value)?,
            Token::Symbol(name) => write!(f, "{}", name.join("."))?,
            Token::Tag(name) => write!(f, "[{}]", name.join("."))?,
            Token::Block(_block) => {
                /*
                write!(f, "{{")?;
                let mut first = true;
                for token in block.data().iter() {
                    if first {
                        first = false;
                    } else {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", token)?;
                }
                write!(f, "}}")?;
                */
                write!(f, "{{Block}}")?;
            }
            Token::Operator(op) => write!(f, "{}", op)?,
            Token::Decorator(decor) => write!(f, "{:?}", decor)?,
        }
        write!(f, ")")?;
        Ok(())
    }
}
