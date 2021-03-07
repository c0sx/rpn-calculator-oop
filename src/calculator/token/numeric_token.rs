use crate::calculator::token::{Token};

#[derive(Debug)]
pub struct NumericToken {
    value: String
}

impl Clone for NumericToken {
    fn clone(&self) -> Self {
        NumericToken::new(self.value.clone())
    }
}

impl Token for NumericToken {
    fn value(&self) -> String {
        return self.value.clone()
    }

    fn len(&self) -> usize {
        self.value.chars().count()
    }

    fn priority(&self) -> Result<u8, &'static str> {
        Err("Токен должен быть оператором")
    }
}

impl NumericToken {
    pub fn new(value: String) -> NumericToken {
        NumericToken {
            value
        }
    }
}
