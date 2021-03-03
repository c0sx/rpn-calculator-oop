mod tokenizer;
mod expression;

use tokenizer::tokenizer::Tokenizer;
use crate::expression::infix_expression::InfixExpression;

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
    let infix = InfixExpression::from(Tokenizer::from(&s));
    let rpn: Vec<String> = vec![];

    println!("input: {}\ninfix: {:?}\nrpn: {:?}\n", s, infix, rpn);
}
