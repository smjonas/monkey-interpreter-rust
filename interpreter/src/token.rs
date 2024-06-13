use phf::phf_map;

pub const ILLEGAL: &str = "ILLEGAL";
pub const EOF: &str = "EOF";

// Identifiers + literals;
pub const IDENT: &str = "IDENT";
pub const INT: &str = "INT";

// Operators;
pub const ASSIGN: &str = "=";
pub const PLUS: &str = "+";
pub const MINUS: &str = "-";
pub const BANG: &str = "!";
pub const ASTERISK: &str = "*";
pub const SLASH: &str = "/";

pub const LT: &str = "<";
pub const GT: &str = ">";

pub const EQ: &str = "==";
pub const NOT_EQ: &str = "!=";

// Delimiters;
pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";
pub const LPAREN: &str = "(";
pub const RPAREN: &str = ")";
pub const LBRACE: &str = "{";
pub const RBRACE: &str = "}";

// Keywords
pub const FUNCTION: &str = "FUNCTION";
pub const LET: &str = "LET";
pub const TRUE: &str = "TRUE";
pub const FALSE: &str = "FALSE";
pub const IF: &str = "IF";
pub const ELSE: &str = "ELSE";
pub const RETURN: &str = "RETURN";

pub static KEYWORDS: phf::Map<&'static str, &'static str> = phf_map! {
    "fn" => FUNCTION,
    "let" => LET,
    "true" => TRUE,
    "false" => FALSE,
    "if" => IF,
    "else" => ELSE,
    "return" => RETURN,
};

type TokenType = String;

#[derive(Debug)]
pub struct Token {
    pub token_type: String,
    pub literal: TokenType,
}

pub fn lookup_ident(ident: &str) -> &str {
    match KEYWORDS.get(ident) {
        Some(token_type) => token_type,
        None => IDENT,
    }
}

pub fn make_token(token_type: &str, literal: &str) -> Token {
    Token {
        token_type: token_type.to_string(),
        literal: literal.to_string(),
    }
}

pub fn make_token_ch(token_type: &str, literal: char) -> Token {
    make_token(token_type, &literal.to_string())
}
