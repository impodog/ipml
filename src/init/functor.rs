use crate::prelude::*;

#[macro_export]
macro_rules! string_list {
    ($($x:expr),*) => {
        [$(String::from($x)),*]
    };
}

pub fn init_functor(scope: &mut Scope) -> Result<(), RuntimeError> {
    scope.insert_functor(&string_list!("print"), |scope| {
        let v = scope.query_value(&string_list!("V"))?;
        println!("{}", v.borrow());
        Ok(rc_cell(Value::Null))
    })?;
    scope.insert_functor(&string_list!("if"), |scope| {
        let c = scope.query_value(&string_list!("C"))?;
        let t = scope.query_value(&string_list!("T"))?;
        let f = scope.query_value(&string_list!("F"))?;
        if c.borrow().to_bool() {
            Ok(t)
        } else {
            Ok(f)
        }
    })?;
    scope.insert_functor(&string_list!("while"), |scope| {
        let mut c = scope.query_value(&string_list!("C"))?.borrow().to_bool();
        let b = scope.query_value(&string_list!("B"))?;
        while c {
            c = b.borrow_mut().call(scope)?.borrow().to_bool();
        }
        Ok(rc_cell(Value::Null))
    })?;
    scope.insert_functor(&string_list!("return"), |scope| {
        Ok(scope.query_value(&string_list!("V"))?)
    })?;

    scope.insert_functor(&string_list!("add"), |scope| {
        match (
            &*scope.query_value(&string_list!("A"))?.borrow(),
            &*scope.query_value(&string_list!("B"))?.borrow(),
        ) {
            (Value::Int(a), Value::Int(b)) => Ok(rc_cell(Value::Int(a + b))),
            (Value::Float(a), Value::Float(b)) => Ok(rc_cell(Value::Float(a + b))),
            (a, b) => Err(RuntimeError::new(format!(
                "[add] Expected two numbers, but got {} and {}",
                a, b
            ))),
        }
    })?;
    scope.insert_functor(&string_list!("sub"), |scope| {
        match (
            &*scope.query_value(&string_list!("A"))?.borrow(),
            &*scope.query_value(&string_list!("B"))?.borrow(),
        ) {
            (Value::Int(a), Value::Int(b)) => Ok(rc_cell(Value::Int(a - b))),
            (Value::Float(a), Value::Float(b)) => Ok(rc_cell(Value::Float(a - b))),
            (a, b) => Err(RuntimeError::new(format!(
                "[sub] Expected two numbers, but got {} and {}",
                a, b
            ))),
        }
    })?;
    scope.insert_functor(&string_list!("mul"), |scope| {
        match (
            &*scope.query_value(&string_list!("A"))?.borrow(),
            &*scope.query_value(&string_list!("B"))?.borrow(),
        ) {
            (Value::Int(a), Value::Int(b)) => Ok(rc_cell(Value::Int(a * b))),
            (Value::Float(a), Value::Float(b)) => Ok(rc_cell(Value::Float(a * b))),
            (a, b) => Err(RuntimeError::new(format!(
                "[mul] Expected two numbers, but got {} and {}",
                a, b
            ))),
        }
    })?;
    scope.insert_functor(&string_list!("div"), |scope| {
        match (
            &*scope.query_value(&string_list!("A"))?.borrow(),
            &*scope.query_value(&string_list!("B"))?.borrow(),
        ) {
            (Value::Int(a), Value::Int(b)) => Ok(rc_cell(Value::Int(a / b))),
            (Value::Float(a), Value::Float(b)) => Ok(rc_cell(Value::Float(a / b))),
            (a, b) => Err(RuntimeError::new(format!(
                "[div] Expected two numbers, but got {} and {}",
                a, b
            ))),
        }
    })?;
    scope.insert_functor(&string_list!("eq"), |scope| {
        match (
            &*scope.query_value(&string_list!("A"))?.borrow(),
            &*scope.query_value(&string_list!("B"))?.borrow(),
        ) {
            (a, b) => Ok(rc_cell(Value::Bool(a == b))),
        }
    })?;
    scope.insert_functor(&string_list!("ne"), |scope| {
        match (
            &*scope.query_value(&string_list!("A"))?.borrow(),
            &*scope.query_value(&string_list!("B"))?.borrow(),
        ) {
            (a, b) => Ok(rc_cell(Value::Bool(a != b))),
        }
    })?;
    scope.insert_functor(&string_list!("lt"), |scope| {
        match (
            &*scope.query_value(&string_list!("A"))?.borrow(),
            &*scope.query_value(&string_list!("B"))?.borrow(),
        ) {
            (a, b) => Ok(rc_cell(Value::Bool(a < b))),
        }
    })?;
    scope.insert_functor(&string_list!("le"), |scope| {
        match (
            &*scope.query_value(&string_list!("A"))?.borrow(),
            &*scope.query_value(&string_list!("B"))?.borrow(),
        ) {
            (a, b) => Ok(rc_cell(Value::Bool(a <= b))),
        }
    })?;
    scope.insert_functor(&string_list!("gt"), |scope| {
        match (
            &*scope.query_value(&string_list!("A"))?.borrow(),
            &*scope.query_value(&string_list!("B"))?.borrow(),
        ) {
            (a, b) => Ok(rc_cell(Value::Bool(a > b))),
        }
    })?;
    scope.insert_functor(&string_list!("ge"), |scope| {
        match (
            &*scope.query_value(&string_list!("A"))?.borrow(),
            &*scope.query_value(&string_list!("B"))?.borrow(),
        ) {
            (a, b) => Ok(rc_cell(Value::Bool(a >= b))),
        }
    })?;
    scope.insert_functor(&string_list!("not"), |scope| {
        Ok(rc_cell(Value::Bool(
            !scope.query_value(&string_list!("V"))?.borrow().to_bool(),
        )))
    })?;

    scope.insert_functor(&string_list!("push_back"), |scope| {
        match (
            &mut *scope.query_value(&string_list!("L"))?.borrow_mut(),
            scope.query_value(&string_list!("V"))?,
        ) {
            (Value::List(l), v) => {
                l.push_back(v);
                Ok(rc_cell(Value::Null))
            }
            (a, b) => Err(RuntimeError::new(format!(
                "[push_back] Expected a list and a value, but got {} and {}",
                a,
                b.borrow(),
            ))),
        }
    })?;
    scope.insert_functor(&string_list!("push_front"), |scope| {
        match (
            &mut *scope.query_value(&string_list!("L"))?.borrow_mut(),
            scope.query_value(&string_list!("V"))?,
        ) {
            (Value::List(l), v) => {
                l.push_front(v);
                Ok(rc_cell(Value::Null))
            }
            (a, b) => Err(RuntimeError::new(format!(
                "[push_front] Expected a list and a value, but got {} and {}",
                a,
                b.borrow(),
            ))),
        }
    })?;
    scope.insert_functor(&string_list!("pop_back"), |scope| {
        match &mut *scope.query_value(&string_list!("L"))?.borrow_mut() {
            Value::List(l) => match l.pop_back() {
                Some(v) => Ok(v),
                None => Ok(rc_cell(Value::Null)),
            },
            a => Err(RuntimeError::new(format!(
                "[pop_back] Expected a list, but got {}",
                a,
            ))),
        }
    })?;
    scope.insert_functor(&string_list!("pop_front"), |scope| {
        match &mut *scope.query_value(&string_list!("L"))?.borrow_mut() {
            Value::List(l) => match l.pop_front() {
                Some(v) => Ok(v),
                None => Ok(rc_cell(Value::Null)),
            },
            a => Err(RuntimeError::new(format!(
                "[pop_front] Expected a list, but got {}",
                a,
            ))),
        }
    })?;
    scope.insert_functor(&string_list!("index"), |scope| {
        match (
            &*scope.query_value(&string_list!("L"))?.borrow(),
            &*scope.query_value(&string_list!("I"))?.borrow(),
        ) {
            (Value::List(l), Value::Int(i)) => match l.get(*i as usize) {
                Some(v) => Ok(v.clone()),
                None => Ok(rc_cell(Value::Null)),
            },
            (a, b) => Err(RuntimeError::new(format!(
                "[index] Expected a list and an integer, but got {} and {}",
                a, b,
            ))),
        }
    })?;
    scope.insert_functor(&string_list!("size"), |scope| {
        match &*scope.query_value(&string_list!("L"))?.borrow() {
            Value::List(l) => Ok(rc_cell(Value::Int(l.len() as isize))),
            a => Err(RuntimeError::new(format!(
                "[size] Expected a list, but got {}",
                a,
            ))),
        }
    })?;
    scope.insert_functor(&string_list!("empty"), |scope| {
        match &*scope.query_value(&string_list!("L"))?.borrow() {
            Value::List(l) => Ok(rc_cell(Value::Bool(l.is_empty()))),
            a => Err(RuntimeError::new(format!(
                "[empty] Expected a list, but got {}",
                a,
            ))),
        }
    })?;
    scope.insert_functor(&string_list!("clear"), |scope| {
        match &mut *scope.query_value(&string_list!("L"))?.borrow_mut() {
            Value::List(l) => {
                l.clear();
                Ok(rc_cell(Value::Null))
            }
            a => Err(RuntimeError::new(format!(
                "[clear] Expected a list, but got {}",
                a,
            ))),
        }
    })?;
    scope.insert_functor(&string_list!("replace"), |scope| {
        match (
            &mut *scope.query_value(&string_list!("L"))?.borrow_mut(),
            &*scope.query_value(&string_list!("I"))?.borrow(),
            scope.query_value(&string_list!("V"))?,
        ) {
            (Value::List(l), Value::Int(i), v) => match l.get_mut(*i as usize) {
                Some(e) => {
                    *e = v;
                    Ok(rc_cell(Value::Null))
                }
                None => Err(RuntimeError::new(format!(
                    "[replace] Index out of range: {}",
                    i
                ))),
            },
            (a, b, c) => Err(RuntimeError::new(format!(
                "[replace] Expected a list, an integer and a value, but got {}, {} and {}",
                a,
                b,
                c.borrow(),
            ))),
        }
    })?;

    scope.insert_functor(&string_list!("mode"), |scope| {
        let mut mode: ScopeMode = Default::default();
        for (k, v) in scope.get_values().iter() {
            match k.as_str() {
                "filter" => {
                    mode.filter = v.borrow().to_bool();
                }
                _ => {}
            }
        }
        if let Some(parent) = scope.get_parent() {
            parent.mode = mode;
        }
        Ok(rc_cell(Value::Null))
    })?;

    Ok(())
}
