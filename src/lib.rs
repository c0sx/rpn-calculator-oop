use tokenizer::tokenizer::Tokenizer;

use crate::expression::expression::Expression;
use crate::calculator::calculator::Calculator;

mod expression;
mod tokenizer;
mod calculator;

pub fn run() {
    parse(String::from("(1 + 2) * 4 + 3 + (-1)"));
    parse(String::from("-1"));
    parse(String::from("- ( -3 )"));
    parse(String::from("1 + 2"));
    parse(String::from("1 - 2"));
    parse(String::from("(-1) - 2"));

    // input
    // parse
    // translate
    // calculate
    // output
}

fn parse(s: String) {
    print!("input: {}\n", &s);
    let tokenizer = Tokenizer::from(&s);

    let expression = Expression::from(tokenizer);
    print!("expression: {:?}\n", expression);

    let mut calculator = Calculator::new();
    let result = calculator.calculate(expression);
    println!("result: {}\n",result);
}
