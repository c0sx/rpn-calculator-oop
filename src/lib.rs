mod parser;

use parser::parser::Parser;

pub fn run() {
    let p = Parser::new();

    let value = String::from("(1 + 2) * 4 + 3 + (-1) kek");
    println!("{}", value);

    let tokens = p.parse(&value);

    println!("{:?}", tokens);

    // input
    // parse
    // translate
    // calculate
    // output
}
