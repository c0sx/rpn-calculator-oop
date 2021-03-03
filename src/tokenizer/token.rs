use super::numeric::Numeric;
use super::separator::Separator;

pub struct Token {
    value: String,
    is_filled: bool,

    numeric: Numeric,
    separators: Separator
}

impl Token {
    pub fn new() -> Token {
        Token {
            value: String::new(),
            is_filled: false,

            numeric: Numeric::new(),
            separators: Separator::new()
        }
    }

    pub fn push(&mut self, c: char) {
        if self.numeric.is_valid(&c) {
            self.value.push(c);
            return;
        }

        if self.separators.is_valid(&c) {
            if self.value.len() == 0 {
                self.value.push(c);
            }

            self.is_filled = true;
            return;
        }

        panic!("Неправильный токен {}", c)
    }

    pub fn is_filled(&self) -> bool {
        self.is_filled
    }

    // todo
    pub fn to_string(&self) -> String {
        self.value.clone()
    }
}
