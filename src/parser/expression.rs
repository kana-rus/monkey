use super::Tokens;


pub struct Expression {

}

pub trait ParseExpression {
    fn expression(tokens: Tokens) -> Expression;
}