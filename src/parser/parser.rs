use super::input::Input;

pub struct Parser {
    separators: Vec<char>,
    numeric: Vec<char>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            separators: vec!['+', '-', '*', '/', '(', ')', '~', '"'],
            numeric: vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'],
        }
    }

    pub fn parse(&self, input: &String) -> Vec<String> {
        let iterator = Input::new(input, &self.separators, &self.numeric);
        iterator.into_iter().collect::<Vec<String>>()
    }
}
