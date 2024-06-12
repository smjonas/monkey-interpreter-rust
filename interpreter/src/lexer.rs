use crate::token::*;

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0 as char,
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0 as char;
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        if self.ch == '\0' {
            let token = make_token_ch(EOF, self.ch);
            self.read_char();
            return token;
        }
        let token = match self.ch {
            '=' => make_token_ch(ASSIGN, self.ch),
            ';' => make_token_ch(SEMICOLON, self.ch),
            '(' => make_token_ch(LPAREN, self.ch),
            ')' => make_token_ch(RPAREN, self.ch),
            ',' => make_token_ch(COMMA, self.ch),
            '+' => make_token_ch(PLUS, self.ch),
            '{' => make_token_ch(LBRACE, self.ch),
            '}' => make_token_ch(RBRACE, self.ch),
            _ => {
                if is_letter(self.ch) {
                    let literal = self.read_identifier();
                    let type_ = lookup_ident(literal);
                    return make_token(type_, literal);
                } else if self.ch.is_digit(10) {
                    let literal = self.read_number();
                    return make_token(INT, literal);
                } else {
                    return make_token_ch(ILLEGAL, self.ch);
                }
            }
        };
        self.read_char();
        token
    }

    fn read_identifier(&mut self) -> &str {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        &self.input[position..self.position]
    }

    fn read_number(&mut self) -> &str {
        println!("pos: {}, char: {}", self.position, self.ch);
        let position = self.position;
        while self.ch.is_digit(10) {
            self.read_char();
        }
        let x = &self.input[position..self.position];
        println!("pos: {}, char: {}", self.position, self.ch);
        x
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_next_token() {
        let input = "let five = 5;
        let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
	return true;
} else {
	return false;
}

10 == 10;
10 != 9;"
            .to_string();

        let mut l = Lexer::new(input);
        let tests = vec![
            make_token(LET, "let"),
            make_token(IDENT, "five"),
            make_token(ASSIGN, "="),
            make_token(INT, "5"),
            make_token(SEMICOLON, ";"),
            make_token(LET, "let"),
            make_token(IDENT, "ten"),
            make_token(ASSIGN, "="),
            make_token(INT, "10"),
            make_token(SEMICOLON, ";"),
            make_token(LET, "let"),
            make_token(IDENT, "add"),
            make_token(ASSIGN, "="),
            make_token(FUNCTION, "fn"),
            make_token(LPAREN, "("),
            make_token(IDENT, "x"),
            make_token(COMMA, ","),
            make_token(IDENT, "y"),
            make_token(RPAREN, ")"),
            make_token(LBRACE, "{"),
            make_token(IDENT, "x"),
            make_token(PLUS, "+"),
            make_token(IDENT, "y"),
            make_token(SEMICOLON, ";"),
            make_token(RBRACE, "}"),
            make_token(SEMICOLON, ";"),
            make_token(LET, "let"),
            make_token(IDENT, "result"),
            make_token(ASSIGN, "="),
            make_token(IDENT, "add"),
            make_token(LPAREN, "("),
            make_token(IDENT, "five"),
            make_token(COMMA, ","),
            make_token(IDENT, "ten"),
            make_token(RPAREN, ")"),
            make_token(SEMICOLON, ";"),
            make_token(BANG, "!"),
            make_token(MINUS, "-"),
            make_token(SLASH, "/"),
            make_token(ASTERISK, "*"),
            make_token(INT, "5"),
            make_token(SEMICOLON, ";"),
            make_token(INT, "5"),
            make_token(LT, "<"),
            make_token(INT, "10"),
            make_token(GT, ">"),
            make_token(INT, "5"),
            make_token(SEMICOLON, ";"),
            make_token(IF, "if"),
            make_token(LPAREN, "("),
            make_token(INT, "5"),
            make_token(LT, "<"),
            make_token(INT, "10"),
            make_token(RPAREN, ")"),
            make_token(LBRACE, "{"),
            make_token(RETURN, "return"),
            make_token(TRUE, "true"),
            make_token(SEMICOLON, ";"),
            make_token(RBRACE, "}"),
            make_token(ELSE, "else"),
            make_token(LBRACE, "{"),
            make_token(RETURN, "return"),
            make_token(FALSE, "false"),
            make_token(SEMICOLON, ";"),
            make_token(RBRACE, "}"),
            make_token(INT, "10"),
            make_token(EQ, "=="),
            make_token(INT, "10"),
            make_token(SEMICOLON, ";"),
            make_token(INT, "10"),
            make_token(NOT_EQ, "!="),
            make_token(INT, "9"),
            make_token(SEMICOLON, ";"),
            make_token(EOF, ""),
        ];
        for test in tests {
            let token = l.next_token();
            assert_eq!(
                test.token_type, token.token_type,
                "Wrong token_type. Expected: {}, got: {}",
                test.token_type, token.token_type
            );
            assert_eq!(
                test.literal, token.literal,
                "Wrong literal. Expected: {}, got: {}",
                test.literal, token.literal
            );
        }
    }
}
