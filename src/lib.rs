mod expression;
mod tokenizer;

use crate::expression::infix_expression::InfixExpression;
use tokenizer::tokenizer::Tokenizer;

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
    let tokenizer = Tokenizer::from(&s);
    let infix = InfixExpression::from(tokenizer);
    let rpn: Vec<String> = vec![];

    println!("input: {}\ninfix: {:?}\nrpn: {:?}\n", s, infix, rpn);
}
