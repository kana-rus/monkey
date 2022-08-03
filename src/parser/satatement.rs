use super::{
    Tokens,
    expression::{Expression}
};


pub enum Statemant {
    Let(LetStatement),
}


type LetStatement = Identifier;
struct Identifier {
    name: String,
    value: Expression,
}


pub trait ParseStatemant {
    fn statement(&mut self) -> Statemant;
}
