use std::io;

struct Lexer {
    expr: String,
    tokens: Vec< (bool, String, usize) >,
    position: usize,
}

impl Lexer {
    fn new(expr: String) -> Lexer {
        let mut lexer = Lexer {
            expr,
            tokens: Vec::new(),
            position: 0,
        };
        
        lexer.process();
        lexer
    }

    fn process(&mut self) {
        loop {
            match self.next_token() {
                Ok(token) => self.add_token(token),
                Err(_) => break,
            }
        }
    }

    fn next_token(&mut self) -> Result<(bool, String, usize), ()> {
        while self.position < self.expr.len() &&
            self.expr.chars().nth(self.position).unwrap().is_whitespace() {
            self.position += 1;
        }

        if self.position >= self.expr.len() {
            return Err(());
        }

        let current_char = self.expr.chars().nth(self.position).unwrap();
        if current_char.is_digit(10) {
            let mut num = String::new();
            let begin = self.position;

            while self.position < self.expr.len() &&
                self.expr.chars().nth(self.position).unwrap().is_digit(10) {
                num.push(self.expr.chars().nth(self.position).unwrap());
                self.position += 1;
            }
            
            return Ok((true, num, begin)); 
        }

        if current_char == '+' || current_char == '-' || current_char == '*' || current_char == '/' {
            self.position += 1;
            return Ok((true, current_char.to_string(), self.position-1));
        }

        self.position += 1;
        Ok((false, "".to_string(), self.position-1))
    }

    fn add_token(&mut self, token: (bool, String, usize)) {
        self.tokens.push(token);
    }

    fn print_tokens(&self) {
        for token in &self.tokens {
            if token.0 {
                let t = (&token.1, token.2);
                print!("{t:?} ");
            } else {
                print!("Error at {} position ", token.2);
            }
        }
        println!();
    }
}

fn main() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("error reading line");

        let lexer = Lexer::new(input);
        lexer.print_tokens();
    }
}
