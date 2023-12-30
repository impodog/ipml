use ipml::*;

#[cfg(test)]
mod tests {

    use super::*;

    fn parse(path: &'static str) -> Token {
        let str = std::fs::read_to_string(path).unwrap();
        let mut parser = Parser::new(str);
        match parser.parse() {
            Ok(tokens) => tokens,
            Err(e) => {
                println!("{}", e);
                panic!();
            }
        }
    }

    fn insert_functors(scope: &mut Scope) {
        scope.insert_functor(&string_list!("print"), |scope| {
            let value = scope.query_value(&string_list!("value"));
            println!("{}", value.borrow());
            Ok(rc_cell(Value::Null))
        });
    }

    fn run(token: Token) {
        let mut scope = Scope::new();
        insert_functors(&mut scope);
        let result = scope.feed(&token);
        match result {
            Ok(value) => println!("File exited: {}", value.borrow()),
            Err(e) => println!("{}", e),
        }
    }

    fn entry(path: &'static str) {
        let tokens = parse(path);
        //println!("{}", tokens);
        run(tokens);
    }

    #[test]
    fn test_structure() {
        entry("tests/structure.ipml");
    }
}
