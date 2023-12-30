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

    fn run(token: Token) {
        let mut scope = Scope::new();
        let result = scope.feed(&token);
        match result {
            Ok(value) => println!("{:#?}", value),
            Err(e) => println!("{}", e),
        }
        println!("{:#?}", scope);
    }

    fn entry(path: &'static str) {
        let tokens = parse(path);
        println!("{:#?}", tokens);
        run(tokens);
    }

    #[test]
    fn test_structure() {
        entry("tests/structure.ipml");
    }
}
