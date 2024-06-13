use crate::token;
use std::any::Any;

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> &str {
        if !self.statements.is_empty() {
            self.statements[0].token_literal()
        } else {
            ""
        }
    }
}

pub trait Node {
    fn token_literal(&self) -> &str;
}

pub trait Statement: Node {
    fn statement_node(&self);
    fn as_any(&self) -> &dyn Any;
}

pub trait Expression: Node {
    fn expression_node(&self);
}

pub struct LetStatement {
    pub token: token::Token,
    pub name: Identifier,
    pub value: Option<Box<dyn Expression>>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> &str {
        self.token.literal
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Identifier {
    pub token: token::Token,
    pub value: &'static str,
}

impl Node for Identifier {
    fn token_literal(&self) -> &str {
        self.token.literal
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}
