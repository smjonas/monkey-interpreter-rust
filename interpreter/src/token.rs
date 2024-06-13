use phf::phf_map;

#[derive(Debug)]
pub enum Token {
    Illegal,
    EOF,
    Ident(String),
    Int(String),
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Eq,
    NotEq,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

// pub const ILLEGAL: &str = "ILLEGAL";
// pub const EOF: &str = "EOF";
//
// // Identifiers + literals;
// pub const IDENT: &str = "IDENT";
// pub const INT: &str = "INT";
//
// // Operators;
// pub const ASSIGN: &str = "=";
// pub const PLUS: &str = "+";
// pub const MINUS: &str = "-";
// pub const BANG: &str = "!";
// pub const ASTERISK: &str = "*";
// pub const SLASH: &str = "/";
//
// pub const LT: &str = "<";
// pub const GT: &str = ">";
//
// pub const EQ: &str = "==";
// pub const NOT_EQ: &str = "!=";
//
// // Delimiters;
// pub const COMMA: &str = ",";
// pub const SEMICOLON: &str = ";";
// pub const LPAREN: &str = "(";
// pub const RPAREN: &str = ")";
// pub const LBRACE: &str = "{";
// pub const RBRACE: &str = "}";
//
// // Keywords
// pub const FUNCTION: &str = "FUNCTION";
// pub const LET: &str = "LET";
// pub const TRUE: &str = "TRUE";
// pub const FALSE: &str = "FALSE";
// pub const IF: &str = "IF";
// pub const ELSE: &str = "ELSE";
// pub const RETURN: &str = "RETURN";
//

pub fn lookup_ident(ident: &str) -> Token {
    return match ident {
        "fn" => Token::Function,
        "let" => Token::Let,
        "true" => Token::True,
        "false" => Token::False,
        "if" => Token::If,
        "else" => Token::Else,
        "return" => Token::Return,
        _ => Token::Ident(ident.to_string()),
    };
}
