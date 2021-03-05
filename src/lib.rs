mod calculator;
mod cli;

pub fn run() {
    let mut cli = cli::Cli::new();

    let input = cli.welcome("Введите выражение:");
    let calc = calculator::Calculator::new();
    let result = calc.calculate_from_string(&input);

    cli.output_results(result);
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
            let calc = calculator::Calculator::new();
            let result = calc.calculate_from_string(&String::from(left));
            assert_eq!(result.value, right as f64);
        }
    }
}
