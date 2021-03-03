#[derive(Debug)]
pub struct Token {
    pub value: String,
}

impl Token {
    pub fn new(value: String) -> Token {
        Token { value }
    }

    pub fn len(&self) -> usize {
        self.value.len()
    }

    pub fn is_numeric(&self) -> bool {
        self.value.parse::<f64>().is_ok()
    }

    pub fn is_operator(&self) -> bool {
        ["+", "-", "*", "/"].contains(&self.value.as_str())
    }

    pub fn is_open_bracket(&self) -> bool {
        self.value == "("
    }

    pub fn is_close_bracket(&self) -> bool {
        self.value == ")"
    }
}
