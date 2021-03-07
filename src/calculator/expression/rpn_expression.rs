use crate::calculator::{self, token::TokenType};

pub struct RpnExpression {
    pub tokens: Vec<TokenType>,
}

impl RpnExpression {
    pub fn new(tokens: Vec<TokenType>) -> RpnExpression {
        RpnExpression { tokens }
    }

    pub fn execute(&self) -> calculator::Result {
        let mut stack: Vec<f64> = Vec::new();

        for token in &self.tokens {
            if token.is_numeric() {
                stack.push(
                    token
                        .value()
                        .parse::<f64>()
                        .expect("Ошибка при парсинге аргумента"),
                );

                continue;
            }

            let result = self.evaluate(&token, &mut stack);
            stack.push(result);
        }

        let result = stack.pop();
        let result = match result {
            Some(result) => result,
            None => panic!("Возникла ошибка при вычислении"),
        };

        calculator::Result::new(self.tokens.clone(), result)
    }

    fn evaluate(&self, token: &TokenType, arguments: &mut Vec<f64>) -> f64 {
        if token.is_operator() == false {
            panic!("токен должен быть операцией")
        }

        match token.value().as_str() {
            "+" => self.add(arguments),
            "*" => self.multiply(arguments),
            "/" => self.divide(arguments),
            "-" => self.subtract(arguments),
            _ => panic!("Недопустимая операция"),
        }
    }

    fn add(&self, arguments: &mut Vec<f64>) -> f64 {
        let mut args = self.get_arguments(arguments);
        if args.len() == 1 {
            return self.positive(&mut args);
        }

        let (a, b) = (args[0], args[1]);
        a + b
    }

    fn subtract(&self, arguments: &mut Vec<f64>) -> f64 {
        let mut args = self.get_arguments(arguments);
        if args.len() == 1 {
            return self.negative(&mut args);
        }

        let (a, b) = (args[0], args[1]);

        b - a
    }

    fn multiply(&self, arguments: &mut Vec<f64>) -> f64 {
        let args = self.get_arguments(arguments);
        let (a, b) = (args[0], args[1]);
        a * b
    }

    fn divide(&self, arguments: &mut Vec<f64>) -> f64 {
        let args = self.get_arguments(arguments);
        let (a, b) = (args[0], args[1]);

        if a == 0.0 {
            panic!("divizion by zero")
        }

        b / a
    }

    fn negative(&self, arguments: &mut Vec<f64>) -> f64 {
        let args = self.get_arguments(arguments);
        let a = args[0];

        return a * -1.0;
    }

    fn positive(&self, arguments: &mut Vec<f64>) -> f64 {
        let args = self.get_arguments(arguments);
        let a = args[0];

        a
    }

    fn get_arguments(&self, arguments: &mut Vec<f64>) -> Vec<f64> {
        let mut args: Vec<f64> = Vec::new();

        let limit = 2;
        let mut iter = 0;

        while iter < limit {
            iter += 1;

            if let Some(arg) = arguments.pop() {
                args.push(arg)
            } else {
                break;
            }
        }

        args
    }
}
