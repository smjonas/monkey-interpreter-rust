use std::io::{self, Write};

use crate::lexer::Lexer;
use crate::token;

const PROMPT: &str = ">> ";

pub fn start() {
    loop {
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let mut lexer = Lexer::new(input);
        loop {
            let token = lexer.next_token();
            println!("{:?}", token);
            if tokenF {
                break;
            }
        }
    }
}
