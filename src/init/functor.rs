use crate::prelude::*;

#[macro_export]
macro_rules! string_list {
    ($($x:expr),*) => {
        [$(String::from($x)),*]
    };
}

pub fn init_functor(scope: &mut Scope) {
    scope.insert_functor(&string_list!("print"), |scope| {
        let v = scope.query_value(&string_list!("V"));
        println!("{}", v.borrow());
        Ok(rc_cell(Value::Null))
    });
    scope.insert_functor(&string_list!("if"), |scope| {
        let c = scope.query_value(&string_list!("C"));
        let t = scope.query_value(&string_list!("T"));
        let f = scope.query_value(&string_list!("F"));
        if c.borrow().to_bool() {
            Ok(t)
        } else {
            Ok(f)
        }
    });
    scope.insert_functor(&string_list!("while"), |scope| {
        let mut c = scope.query_value(&string_list!("C")).borrow().to_bool();
        let b = scope.query_value(&string_list!("B"));
        while c {
            c = b.borrow_mut().call(scope)?.borrow().to_bool();
        }
        Ok(rc_cell(Value::Null))
    });
    scope.insert_functor(&string_list!("return"), |scope| {
        Ok(scope.query_value(&string_list!("V")))
    });

    scope.insert_functor(&string_list!("add"), |scope| {
        match (
            &*scope.query_value(&string_list!("A")).borrow(),
            &*scope.query_value(&string_list!("B")).borrow(),
        ) {
            (Value::Int(a), Value::Int(b)) => Ok(rc_cell(Value::Int(a + b))),
            (Value::Float(a), Value::Float(b)) => Ok(rc_cell(Value::Float(a + b))),
            (a, b) => Err(RuntimeError::new(format!(
                "[Sub] Expected two numbers, but got {} and {}",
                a, b
            ))),
        }
    });
    scope.insert_functor(&string_list!("sub"), |scope| {
        match (
            &*scope.query_value(&string_list!("A")).borrow(),
            &*scope.query_value(&string_list!("B")).borrow(),
        ) {
            (Value::Int(a), Value::Int(b)) => Ok(rc_cell(Value::Int(a - b))),
            (Value::Float(a), Value::Float(b)) => Ok(rc_cell(Value::Float(a - b))),
            (a, b) => Err(RuntimeError::new(format!(
                "[Sub] Expected two numbers, but got {} and {}",
                a, b
            ))),
        }
    });
    scope.insert_functor(&string_list!("mul"), |scope| {
        match (
            &*scope.query_value(&string_list!("A")).borrow(),
            &*scope.query_value(&string_list!("B")).borrow(),
        ) {
            (Value::Int(a), Value::Int(b)) => Ok(rc_cell(Value::Int(a * b))),
            (Value::Float(a), Value::Float(b)) => Ok(rc_cell(Value::Float(a * b))),
            (a, b) => Err(RuntimeError::new(format!(
                "[Mul] Expected two numbers, but got {} and {}",
                a, b
            ))),
        }
    });
    scope.insert_functor(&string_list!("div"), |scope| {
        match (
            &*scope.query_value(&string_list!("A")).borrow(),
            &*scope.query_value(&string_list!("B")).borrow(),
        ) {
            (Value::Int(a), Value::Int(b)) => Ok(rc_cell(Value::Int(a / b))),
            (Value::Float(a), Value::Float(b)) => Ok(rc_cell(Value::Float(a / b))),
            (a, b) => Err(RuntimeError::new(format!(
                "[Div] Expected two numbers, but got {} and {}",
                a, b
            ))),
        }
    });
    scope.insert_functor(&string_list!("eq"), |scope| {
        match (
            &*scope.query_value(&string_list!("A")).borrow(),
            &*scope.query_value(&string_list!("B")).borrow(),
        ) {
            (a, b) => Ok(rc_cell(Value::Bool(a == b))),
        }
    });
    scope.insert_functor(&string_list!("ne"), |scope| {
        match (
            &*scope.query_value(&string_list!("A")).borrow(),
            &*scope.query_value(&string_list!("B")).borrow(),
        ) {
            (a, b) => Ok(rc_cell(Value::Bool(a != b))),
        }
    });
    scope.insert_functor(&string_list!("lt"), |scope| {
        match (
            &*scope.query_value(&string_list!("A")).borrow(),
            &*scope.query_value(&string_list!("B")).borrow(),
        ) {
            (a, b) => Ok(rc_cell(Value::Bool(a < b))),
        }
    });
    scope.insert_functor(&string_list!("le"), |scope| {
        match (
            &*scope.query_value(&string_list!("A")).borrow(),
            &*scope.query_value(&string_list!("B")).borrow(),
        ) {
            (a, b) => Ok(rc_cell(Value::Bool(a <= b))),
        }
    });
    scope.insert_functor(&string_list!("gt"), |scope| {
        match (
            &*scope.query_value(&string_list!("A")).borrow(),
            &*scope.query_value(&string_list!("B")).borrow(),
        ) {
            (a, b) => Ok(rc_cell(Value::Bool(a > b))),
        }
    });
    scope.insert_functor(&string_list!("ge"), |scope| {
        match (
            &*scope.query_value(&string_list!("A")).borrow(),
            &*scope.query_value(&string_list!("B")).borrow(),
        ) {
            (a, b) => Ok(rc_cell(Value::Bool(a >= b))),
        }
    });
}
