use crate::calculator::token::{Token};

#[derive(Debug)]
pub struct OperationToken {
    value: String
}

impl Clone for OperationToken {
    fn clone(&self) -> Self {
        OperationToken::new(self.value.clone())
    }
}

impl Token for OperationToken {
    fn value(&self) -> String {
        return self.value.clone()
    }

    fn len(&self) -> usize {
        self.value.chars().count()
    }

    fn priority(&self) -> Result<u8, &'static str> {
        let value = match self.value.as_str() {
            "*" => 1,
            "/" => 1,
            "+" => 0,
            "-" => 0,
            _ => return Err("Токен должен быть оператором")
        };

        return Ok(value);
    }
}

impl OperationToken {
    pub fn new(value: String) -> OperationToken {
        OperationToken {
            value
        }
    }
}
