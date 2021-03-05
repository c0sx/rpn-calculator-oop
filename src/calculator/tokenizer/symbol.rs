#[derive(PartialEq)]
#[derive(Debug)]
pub enum Symbol {
    Numeric,
    BinaryOperator,
    MaybeUnaryOperator,
    Brackets,
}

impl Symbol {
    pub fn from(c: &char) -> Symbol {
        return if vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'].contains(c) {
            Symbol::Numeric
        } else if vec!['*', '/'].contains(c) {
            Symbol::BinaryOperator
        } else if vec!['+', '-'].contains(c) {
            Symbol::MaybeUnaryOperator
        } else if vec!['(', ')'].contains(c) {
            Symbol::Brackets
        } else {
            panic!("Недопустимый символ")
        };
    }
}
