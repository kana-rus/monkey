#![allow(unused)]

mod lexer; use lexer::toknize;
mod parser; use parser::Parser;


fn main() {
    let tokens = toknize(String::from(/*"
let five = 5;
let ten = 10;
let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
!-/5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
    "*/"
let five = 1 + 2* 2;
    "));

    let parser = Parser::new(tokens);
    println!("{:?}", parser.parse());
}
