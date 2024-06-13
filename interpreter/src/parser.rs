use crate::ast::*;
use crate::lexer::*;
use crate::token::*;
use std::any::Any;

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            cur_token: Token::EOF,
            peek_token: Token::EOF,
        };
        // Read two tokens, so cur_token and peek_token are both set
        parser.next_token();
        parser.next_token();
        parser
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token;
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: vec![] };
        while let self.cur_token != EOF {
            let stmt = self.parse_statement();
            if stmt.is_some() {
                program.statements.push(stmt.unwrap());
            }
            self.next_token();
        }
        program
    }

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        return match self.cur_token {
            LET => self.parse_let_statement(),
            _ => None,
        };
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        if !self.expect_peek(Token::Ident(_))) {
            return None;
        }
        let name = Identifier {
            token: self.cur_token,
            value: self.cur_token.literal,
        };
        if !self.expect_peek(Token::Assign) {
            return None;
        }
        while !self.cur_token_is(Token::Semicolon) {
            self.next_token();
        }
        Some(Box::new(LetStatement {
            token: self.cur_token,
            name,
            value: None,
        }))
    }

    fn cur_token_is(&self, token: &Token) -> bool {
        self.cur_token == token
    }

    fn peek_token_is(&self, token: &Token) -> bool {
        self.peek_token == token
    }

    fn expect_peek(&mut self, token: &Token) -> bool {
        if self.peek_token_is(token) {
            self.next_token();
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_string() {
        let input = "let x = 5;
let y = 10;
let foobar = 838383;"
            .to_string();
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        assert_eq!(
            program.statements.len(),
            3,
            "program.statements does not contain 3 statements, got={}",
            program.statements.len()
        );
        let tests = vec!["x", "y", "foobar"];
        for (i, test) in tests.iter().enumerate() {
            let statement = program.statements[i];
        }
    }

    fn test_let_statement(s: Box<dyn Statement>, name: &str) {
        assert_eq!(
            s.token_literal(),
            "let",
            "s.token_literal not 'let', got={}",
            s.token_literal()
        );
        let let_stmt = s.as_any().downcast_ref::<LetStatement>().unwrap();
        assert_eq!(
            let_stmt.name.value, name,
            "let_stmt.name.value not '{}', got={}",
            name, let_stmt.name.value
        );
        assert_eq!(
            let_stmt.name.token_literal(),
            name,
            "let_stmt.name.token_literal not '{}', got={}",
            name,
            let_stmt.name.token_literal()
        );
    }
}
