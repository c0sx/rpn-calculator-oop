use std::io::{self, Write};

use crate::calculator::calculation;

pub struct Cli {
    stdin: io::Stdin,
    stdout: io::Stdout,
}

impl Cli {
    pub fn new() -> Cli {
        Cli {
            stdin: io::stdin(),
            stdout: io::stdout(),
        }
    }

    pub fn welcome(&mut self, message: &str) -> String {
        self.output_string(message);

        self.read_to_string()
    }

    pub fn output_results(&mut self, result: calculation::Result) {
        let expression = result
            .expression
            .iter()
            .map(|t| t.value.as_str())
            .collect::<Vec<&str>>()
            .join(" ");

        self.output_string(expression.as_str());
        self.output_string(result.value.to_string().as_str());
    }

    fn read_to_string(&self) -> String {
        let mut input = String::new();

        self.stdin
            .read_line(&mut input)
            .expect("При обработки ввода возникла ошибка");

        String::from(input.trim())
    }

    fn output_string(&mut self, s: &str) -> io::Result<()> {
        let mut line = String::from(s);
        line.push('\n');

        self.stdout.write_all(line.as_bytes())?;
        self.stdout.flush()
    }
}
