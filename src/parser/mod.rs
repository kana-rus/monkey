mod satatement;
mod expression;


use super::lexer::{Token, Tokens};

use satatement::{
    ParseStatemant, Statemant, Identifier,
};
use expression::{
    ParseExpression, Expression,
};




type Program = Vec<Statemant>;






pub struct Parser {
    tokens: Tokens,
    current: Token,
    peek: Option<Token>,
} impl Parser {

    pub fn new(mut tokens: Tokens) -> Parser {
        let current = tokens.pop().unwrap();
        let peek = tokens.pop();
        Parser {
            tokens, current, peek
        }
    }

    pub fn parse(mut self) -> Program {
        let mut statements = Vec::<Statemant>::new();
        while !self.current.is_eof() {
            statements.push(self.let_statement());
        }
        statements
    }

    fn read_1(&mut self) {
        self.current = self.peek.take().expect("No more token!");
        self.peek = self.tokens.pop();
    }
    fn consume_let(&mut self) {
        match self.current {
            Token::Let => self.read_1(),
            _ => panic!("Token::Let expected"),
        }
    }
    fn expect_ident(&mut self) -> String {
        match self.current.copy() {
            Token::Ident(ident) => { self.read_1(); ident },
            _ => panic!("Token::Ident expected"),
        }
    }
    fn consume_assign(&mut self) {
        match self.current {
            Token::Assign => self.read_1(),
            _ => panic!("Token::Assign expected"),
        }
    }
    fn consume_semicolon(&mut self) {
        match self.current {
            Token::Semicolon => self.read_1(),
            _ => panic!("Token::Semicolon expected"),
        }
    }
}
    