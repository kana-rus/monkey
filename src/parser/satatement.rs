use super::{
    Parser,
    expression::{Expression, ParseExpression}
};

#[derive(Debug)]
pub enum Statemant {
    Let(Identifier),
}

#[derive(Debug)]
pub struct Identifier {
    name: String,
    value: Expression,
}


pub trait ParseStatemant {
    fn let_statement(&mut self) -> Statemant;
} impl ParseStatemant for Parser {
    fn let_statement(&mut self) -> Statemant {
        self.consume_let();
        let name = self.expect_ident();
        self.consume_assign();
        let value = self.expression();
        self.consume_semicolon();
        Statemant::Let(Identifier{name, value})
    }
}
