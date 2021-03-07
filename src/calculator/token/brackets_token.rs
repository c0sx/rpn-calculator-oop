use crate::calculator::token::{Token, TokenType};

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

    fn move_on_sort(&self, output_queue: &mut Vec<TokenType>, stack: &mut Vec<TokenType>) {
        if self.value == "(" {
            stack.push(TokenType::Brackets(BracketsToken::new(self.value.clone())));
        } else {
            let mut open_brackets_counter = 0;

            while let Some(token) = stack.pop() {
                if token.is_open_bracket() == false {
                    output_queue.push(token);
                } else {
                    open_brackets_counter += 1;
                    break;
                }
            }

            if open_brackets_counter == 0 {
                panic!("Пропущена открывающая скобка")
            }
        }
    }
}

impl BracketsToken {
    pub fn new(value: String) -> BracketsToken {
        BracketsToken {
            value
        }
    }
}
