pub struct Numeric {
    numbers: Vec<char>,
}

impl Numeric {
    pub fn new() -> Numeric {
        Numeric {
            numbers: vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'],
        }
    }

    pub fn is_valid(&self, c: &char) -> bool {
        self.numbers.contains(c)
    }
}
