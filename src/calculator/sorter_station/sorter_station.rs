use crate::calculator::calculus::RpnExpression;
use crate::calculator::token::{TokenType};

pub struct SorterStation {}

impl SorterStation {
    pub fn new() -> SorterStation {
        SorterStation {}
    }

    pub fn sort(&self, expression: &Vec<TokenType>) -> RpnExpression {
        let mut output_queue: Vec<TokenType> = vec![];
        let mut stack: Vec<TokenType> = vec![];

        self.process_tokens(expression, &mut output_queue, &mut stack);
        self.remains_in_stack(&mut output_queue, &mut stack);

        RpnExpression::new(output_queue)
    }

    fn process_tokens(
        &self,
        expression: &Vec<TokenType>,
        output_queue: &mut Vec<TokenType>,
        stack: &mut Vec<TokenType>,
    ) {
        for token in expression {
            token.move_on_sort(output_queue, stack);
        }
    }

    fn remains_in_stack(&self, output_queue: &mut Vec<TokenType>, stack: &mut Vec<TokenType>) {
        println!("queue {:?}", output_queue);
        while let Some(token) = stack.pop() {
            if token.is_brackets() {
                panic!("Присутствует незакрытая скобка")
            }

            output_queue.push(token)
        }
    }
}
