mod tokenizer;
mod tokens;

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
    let infix_tokens = Tokenizer::from(&s).parse();
    let rpn_tokens = infix_tokens.to_rpn_tokens();

    println!("input: {}\ninfix: {:?}\nrpn: {:?}\n", s, infix_tokens, rpn_tokens);
}
