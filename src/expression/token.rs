#[derive(Debug)]
pub struct Token {
    pub value: String,
}

impl Clone for Token {
    fn clone(&self) -> Self {
        Token::new(self.value.clone())
    }
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

    pub fn is_brackets(&self) -> bool {
        return self.is_open_bracket() || self.is_close_bracket()
    }

    pub fn is_close_bracket(&self) -> bool {
        self.value == ")"
    }

    pub fn priority(&self) -> Result<u8, &'static str> {
        if self.is_operator() == false {
            return Err("Токен должен быть оператором")
        }

        let value = match self.value.as_str() {
            "*" => 1,
            "/" => 1,
            "+" => 0,
            "-" => 0,
            _ => panic!("Токен должен быть оператором")
        };

        return Ok(value)
    }

    pub fn clone(&self) -> Token {
        Token::new(self.value.clone())
    }
}
