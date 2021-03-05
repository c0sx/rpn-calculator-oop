mod calculator;
mod cli;
mod expression;
mod tokenizer;

use crate::calculator::calculation;
use crate::calculator::calculator::Calculator;
use crate::cli::cli::Cli;
use crate::tokenizer::tokenizer::Tokenizer;

pub fn run() {
    let mut cli = Cli::new();

    let input = cli.welcome("Введите выражение:");
    let result = calculate(input);

    cli.output_results(result);
}

fn calculate(s: String) -> calculation::Result {
    let tokenizer = Tokenizer::from(&s);
    let calculator = Calculator::new();

    calculator.calculate(tokenizer.parse())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculator() {
        let input = vec![
            ("(-1)", -1),
            ("(1 + 2) * 4 + 3 + (-1)", 14),
            ("3 + (-1)", 2),
            ("-1", -1),
            ("- ( -3 )", 3),
            ("1 + 2", 3),
            ("1 - 2", -1),
            ("-1 - 2", -3),
            ("11 - (-11)", 22),
            ("(-11)", -11),
        ];

        for (left, right) in input {
            let result = calculate(String::from(left));
            assert_eq!(result, right as f64);
        }
    }
}
