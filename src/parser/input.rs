pub struct Input {
    input: String,

    separators: Vec<char>,
    numeric: Vec<char>,
    cursor: usize,
    prev_token: String,
}

// имплементация интерфейса итератора
impl Iterator for Input {
    type Item = String; // String to Token?

    // на каждой итерации берем срез строки от текущего положения курсора и пытаемся найти в нем следующий токен
    fn next(&mut self) -> Option<Self::Item> {
        self.get_next_token()
    }
}

impl Input {
    pub fn new(
        input: &String,
        separators: &Vec<char>,
        numeric: &Vec<char>,
    ) -> Input {
        let without_whitespaces = input.split_whitespace().collect::<String>();

        Input {
            input: without_whitespaces,
            separators: separators.clone(),
            numeric: numeric.clone(),
            cursor: 0,
            prev_token: String::new(),
        }
    }

    // получить следующий токен
    // трансформировать унарные токены
    fn get_next_token(&mut self) -> Option<String> {
        if self.cursor >= self.input.len() {
            return None;
        }

        let substring = &self.input[self.cursor..];
        let token = self.parse_next_token(substring);

        let token = self.transform_token_if_needed(&token);
        self.prev_token = token.clone();
        self.cursor += token.len();

        return if token.len() == 0 {
            None
        } else {
            Some(token)
        };
    }

    // получение следующего токена со среза строки (для итератора)
    fn parse_next_token(&self, s: &str) -> String {
        let mut token = String::new();

        for c in s.chars().into_iter() {
            if self.numeric.contains(&c) {
                token.push(c);
                continue;
            }

            if self.separators.contains(&c) {
                if token.len() == 0 {
                    token.push(c);
                }

                break;
            }
        }

        token
    }

    // отделяем унарные токены от бинарных для логики в калькуляторе
    fn transform_token_if_needed(&self, token: &String) -> String {
        if token.len() == 0 || token != "-" {
            return token.clone();
        }

        let operators = self
            .separators
            .iter()
            .map(|op| op.to_string())
            .collect::<Vec<String>>();

        let is_unary_token = if self.prev_token.len() > 0 {
            operators.contains(&self.prev_token)
        } else {
            false
        };

        return if is_unary_token {
            String::from("~")
        } else {
            token.clone()
        };
    }
}
