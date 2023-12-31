# IPML - Interactive Primitive Markup Language

## Quick Start

To parse a example file, try the following code:

```rust
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
}
```

You can further read, write, expand, or continue to feed more tokens to the scope.

```rust
fn main() {
    // ...

    / Query one single value
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
```

## Scripting

### Basic Syntax

A line of comment starts with `#`,  and all text, after `#`, before the next line, will be ignored.

A valid block always have token number that is a multiple of 3. For each 3 tokens, the middle one must be operator, directing what to do. Then, the left one and the right one is operated using the operator.

### Token

A token is a single unit of IPML. It can be a literal value, a name, a tag, a block, etc.

|   Type    |                            Usage                             |                Examples                 |
| :-------: | :----------------------------------------------------------: | :-------------------------------------: |
|   Value   |               A literal value, e.g. int, float               |        1, 2.5, "Hello!", (1 2 3)        |
|  Symbol   |   A name that can be referred to a value in current scope    |  foo, my_scope.value, my_sym123, $test  |
|    Tag    | A name wrapped in \[ ], that can be referred to a sub-scope in current scope |    [my_scope], [structure], [config]    |
|   Block   |   Code wrapped in \( ) and used by scopes to execute stuff   | (hello = "Hello!"), (print! (V = 3.14)) |
| Operator  |            Any stray char except all other types             |                 : ! = ~                 |
| Decorator |       A keyword put before Block, indicating its type        |            fn, indep, subsc             |

### Value

A value holds a dynamic part of memory. It can be initialized using different kinds of commands.

|  Type  |                            Usage                             |               Literal Form Examples                |
| :----: | :----------------------------------------------------------: | :------------------------------------------------: |
|  Int   |                       A system integer                       |                   1, 5, -3, -123                   |
| Float  |                A system floating point number                |                 3.14159, -4.2, 8.0                 |
|  Bool  |             A value holding either true or false             |                    true, false                     |
| String |                      A string of chars                       |    "Hello, world!", "Greetings!\nHow are you?"     |
|  List  |               A list(actually deque) of values               | (1  2  3), ("a"  "b"  1.2  true), ((1  2)  (3  4)) |
|  Null  | A value that indicates nothing, or acts as a placeholder for unknown values |                        null                        |

### Scope

The scope holds all the data that belong to it.

There are two kinds of data - Value and Scope. Value can be accessed using a Symbol Token, while Scope being Tag Token. Please note that when implying a scope in a value reference(for example `my_scope.value`), \[ ] are not allowed. The interpreter will make inferences.

#### Auto Filling

Whenever a required sub-scope or value is missing, the position is automatically set to default(for Value it's null, and for Scope it's an empty Scope).

#### Search Method

For any nested name given, the name, either value or scope, is searched in both current and ancestor scopes. If any match, use that value scope, otherwise, create a default value or scope in current scope. Then the rest is directly searched, without searching in ancestor scopes.

### External Functor

An external functor is implemented elsewhere, but looks and feels the same as any other functors. In Rust, you can simply insert one of those using `Scope::insert_functor` and use it in your scope.

## Examples

To see more about the syntax and usage, please refer to the [examples](examples) folder.
