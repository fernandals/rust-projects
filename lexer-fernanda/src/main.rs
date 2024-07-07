use std::io;
use std::vec::Vec;

fn is_operator(c: char) -> bool {
    c == '+' || c == '-' || c == '*' || c == '/'
}

struct Lexer {
    input: String,
    tokens: Vec< (String, usize) >,
}

impl Lexer {
    fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input,
            tokens: Vec::new(),
        };
        
        lexer.process_input();
        lexer
    }

    fn process_input(&mut self) {
        let mut pos = 0;

        while pos < self.input.len() {
            let curr_char = self.input.chars().nth(pos).unwrap();

            if curr_char.is_whitespace() {
                pos += 1;
            } else if curr_char.is_digit(10) {
                let init_pos = pos;
                let mut number = String::new();

                while (pos < self.input.len()) &&
                    (self.input.chars().nth(pos).unwrap().is_digit(10)) {
                    number.push(self.input.chars().nth(pos).unwrap());
                    pos += 1;
                }
               
                self.tokens.push( (number, init_pos) );
            } else if is_operator(curr_char) {
                self.tokens.push( (curr_char.to_string(), pos) );
                pos += 1;
            } else {
                self.tokens.push( ("ERROR".to_string(), pos) );
                pos += 1;
            }
        }
    }

    fn print_tokens(&self) {
        for token in &self.tokens {
            if token.0 == "ERROR" {
                print!("Error at {} ", token.1);
            } else {
                print!("{token:?} ");
            }   
        }
    }
}

fn main() {
    loop {
        let mut input = String::new();

        println!("Please enter the expression:");
    
        io::stdin().read_line(&mut input)
            .expect("Failed to read line.");

        let lexer = Lexer::new(input);
        lexer.print_tokens();
        println!();
    }
    
}
