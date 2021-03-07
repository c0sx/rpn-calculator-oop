use crate::calculator::token::{Token};

#[derive(Debug)]
pub struct BracketsToken {
    value: String
}

impl Clone for BracketsToken {
    fn clone(&self) -> Self {
        BracketsToken::new(self.value.clone())
    }
}

impl Token for BracketsToken {
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

impl BracketsToken {
    pub fn new(value: String) -> BracketsToken {
        BracketsToken {
            value
        }
    }
}
