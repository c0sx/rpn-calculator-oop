mod expression;
mod tokenizer;
mod calculator;

use crate::calculator::calculator::Calculator;
use crate::tokenizer::tokenizer::Tokenizer;

pub fn run() {
    // parse(String::from("(1 + 2) * 4 + 3 + (-1)"));
    // parse(String::from("3 + (-1)"));
    // parse(String::from("-1"));
    // parse(String::from("- ( -3 )"));
    // parse(String::from("1 + 2"));
    // parse(String::from("1 - 2"));
    parse(String::from("-1 - 2"));
    // parse(String::from("11 - (-11)"));
    // parse(String::from("(-11)"));

    // input
    // parse
    // translate
    // calculate
    // output
}

fn parse(s: String) {
    print!("{} = ", &s);
    let tokenizer = Tokenizer::from(&s);
    let calculator = Calculator::new();

    let result = calculator.calculate(tokenizer.parse());
    print!("{}\n", result);
}
