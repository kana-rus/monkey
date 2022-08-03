mod satatement;
mod expression;


use super::lexer::{Token, Tokens};

use satatement::{
    ParseStatemant, Statemant,
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
            statements.push(self.statement());
        }
        statements
    }
}

impl ParseStatemant for Parser {
    fn statement(&mut self) -> Statemant {
        
    }
}


    fn read_1(parser: &mut Parser) {
        parser.current = parser.peek.take().expect("No more token!");
        parser.peek = parser.tokens.pop();
    }
    fn consume_let(parser: &mut Parser) {
        match parser.current {
            Token::Let => read_1(parser),
            _ => panic!("Token::Let expected"),
        }
    }
    fn expect_ident(parser: &mut Parser) -> String {
        match parser.current.copy() {
            Token::Ident(ident) => { read_1(parser); ident },
            _ => panic!("Token::Ident expected"),
        }
    }
    fn consume_assign(parser: &mut Parser) {
        match parser.current {
            Token::Assign => read_1(parser),
            _ => panic!("Token::Assign expected"),
        }
    }
    fn consume_semicolon(parser: &mut Parser) {
        match parser.current {
            Token::Semicolon => read_1(parser),
            _ => panic!("Token::Semicolon expected"),
        }
    }