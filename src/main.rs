fn main() {
    use ipml::*;

    let str = std::fs::read_to_string("examples/example.ipml").unwrap();

    // Parse the file, returning one token
    let mut parser = Parser::new(str);
    let token;
    match parser.parse() {
        Ok(token_) => {
            token = token_;
        }
        Err(e) => {
            println!("ERROR! {}", e);
            panic!();
        }
    }

    // Initialize the scope with default functor
    let mut scope = Scope::new();
    init_functor(&mut scope).unwrap();

    // Feed the parsed token to the scope
    let result = scope.feed(&token);
    // Do cleanup(if necessary)
    scope.cleanup();

    match result {
        Ok(_) => {}
        Err(e) => {
            println!("ERROR! {}", e);
            panic!();
        }
    }
    // Output the result
    println!("Resulting Scope: {}", scope);

    // Query one single value
    let foo = scope.query_value(&string_list!("foo")).unwrap();
    // foo: Rc<RefCell<Value>>
    println!("foo: {}", foo.borrow());

    // Query one single sub-scope
    let my_scope = scope.query_scope(&string_list!("my_scope")).unwrap();
    // my_scope: Rc<RefCell<Scope>>
    println!("my_scope: {}", my_scope.borrow());

    // You can also simplify deeply nested queries using string_list!
    // This is the same as you would write "my_scope.bar"
    let bar = scope.query_value(&string_list!("my_scope", "bar")).unwrap();
    // bar: Rc<RefCell<Value>>
    println!("bar: {}", bar.borrow());

    // You can also set values using simple functions
    scope
        .set_value(
            &string_list!("my_scope", "bar"),
            // rc_cell is a shortcut function that creates a Rc<RefCell<T>>
            rc_cell(Value::Str("Hello, world!".to_string())),
        )
        // Don't actually write unwrap() in your code, please add proper error handling
        .unwrap();

    // You can create sub-scopes using Scope::with_parent, of course, that is empty
    let my_scope2 = Scope::with_parent("my_scope2".to_string(), &mut scope).unwrap();
    // my_scope2: Rc<RefCell<Scope>>
    my_scope2
        .borrow_mut()
        .set_value(
            &string_list!("bar"),
            rc_cell(Value::Str("This is another message!".to_string())),
        )
        .unwrap();
}
