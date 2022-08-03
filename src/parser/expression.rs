use super::{
    Token,
    Tokens,
    Parser,
};

type Int = usize;
#[derive(Debug)]
enum Ope {
    Add,
    Sub,
    Mul,
    Div,
} use Ope::*;
type Link<Node> = Option<Box<Node>>;
fn link(node: Node) -> Link<Node> {
    Some(Box::new(node))
}


/*
expr    ::= mul ('+' mul | '-' mul)*
mul     ::= primary ('*' praimry | '/' primary)*
primary ::= num | '(' expr ')'
*/
#[derive(Debug)]
pub struct Expression {
    root: Node,
}
#[derive(Debug)]
enum Elem {
    Int(Int),
    Ope(Ope),
    Init,
}
#[derive(Debug)]
struct Node {
    elem: Elem,
    left: Link<Node>,
    right: Link<Node>,
} impl Node {
    fn new() -> Node {
        Node {
            elem: Elem::Init,
            left: None,
            right: None
        }
    }
}


pub trait ParseExpression {
    fn expression(&mut self) -> Expression;
} impl ParseExpression for Parser {
    fn expression(&mut self) -> Expression {
        let root = expr(self);
        Expression { root }
    }
}

fn expr(parser: &mut Parser) -> Node {
    let mut node = mul(parser);
    match parser.current {
        // None => node,
        Token::Plus => {
            if parser.peek.is_none() { panic!("Add-ope doesn't have righthand"); }
            parser.read_1();
            node = Node {
                elem: Elem::Ope(Add),
                left: link(node),
                right: link(mul(parser)),
            };
            node
        },
        Token::Minus => {
            if parser.peek.is_none() { panic!("Sub-ope doesn't have righthand"); }
            parser.read_1();
            Node {
                elem: Elem::Ope(Sub),
                left: link(node),
                right: link(mul(parser)),
            }
        },
        _ => node,
    }
}
fn mul(parser: &mut Parser) -> Node {
    let node = primary(parser);
    match parser.current {
        // None => node,
        Token::Astarisk => {
            if parser.peek.is_none() { panic!("Mul-ope doesn't have righthand"); }
            parser.read_1();
            Node {
                elem: Elem::Ope(Mul),
                left: link(node),
                right: link(primary(parser))
            }
        },
        Token::Slash => {
            if parser.peek.is_none() { panic!("Div-ope doesn't have righthand"); }
            parser.read_1();
            Node {
                elem: Elem::Ope(Sub),
                left: link(node),
                right: link(primary(parser))
            }
        },
        _ => node,
    }
}
fn primary(parser: &mut Parser) -> Node {
    match parser.current {
        Token::Int(n) => {
            parser.read_1();
            Node {
                elem: Elem::Int(n),
                left: None,
                right: None,
            }
        },
        Token::Lparen => {
            if parser.peek.is_none() { panic!("Lparen doesn't close"); }
            parser.read_1();

            let expr = expr(parser);

            if parser.peek.is_none() { panic!("Lparen doesn't close"); }
            parser.read_1();
            match parser.current {
                Token::Rparen => {
                    parser.read_1();
                    expr
                },
                ref other => panic!("unexpected token: {:?}", other),
            }
        },
        ref other => panic!("unexpected token: {:?}", other),
    }
}