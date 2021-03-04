pub struct Separator {
    separators: Vec<char>,
}

impl Separator {
    pub fn new() -> Separator {
        Separator {
            separators: vec!['+', '-', '*', '/', '(', ')'],
        }
    }

    pub fn is_valid(&self, c: &char) -> bool {
        self.separators.contains(c)
    }

    pub fn is_maybe_numeric(&self, c: &char) -> bool {
        ['-'].contains(c)
    }
}
