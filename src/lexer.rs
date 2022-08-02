use std::{
    str::Chars,
    collections::VecDeque,
};


#[derive(Debug)]
pub enum Token {
    Illegal,
    EOF,

    // 識別子, リテラル
    Ident(String),
    Int(usize),

    // 演算子
    Assign,
    Plus,
    Minus,
    Bang,
    Astarisk,
    Slash,
    Lt,
    Gt,
    Eq,
    NotEq,

    // デリミタ
    Comma,
    Semicolon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    // キーワード
    Functon,
    Let,
    True,
    False,
    Else,
    Return,
} impl Token {
    fn copy(&self) -> Self {
        match self {
            Token::Ident(string) => Token::Ident(string.clone()),
            Token::Int(n) => Token::Int(*n),

            Token::Illegal => Token::Illegal,
            Token::EOF => Token::EOF,
            Token::Assign => Token::Assign,
            Token::Plus => Token::Plus,
            Token::Minus => Token::Minus,
            Token::Bang => Token::Bang,
            Token::Astarisk => Token::Astarisk,
            Token::Slash => Token::Slash,
            Token::Lt => Token::Lt,
            Token::Gt => Token::Gt,
            Token::Eq => Token::Eq,
            Token::NotEq => Token::NotEq,
            Token::Comma => Token::Comma,
            Token::Semicolon => Token::Semicolon,
            Token::Lparen => Token::Lparen,
            Token::Rparen => Token::Rparen,
            Token::Lbrace => Token::Lbrace,
            Token::Rbrace => Token::Rbrace,
            Token::Functon => Token::Functon,
            Token::Let => Token::Let,
            Token::True => Token::True,
            Token::False => Token::False,
            Token::Else => Token::Else,
            Token::Return => Token::Return,
        }
    }
}


pub fn toknize(input: String) -> VecDeque<Token> {
    let mut input = input.chars();
    let mut tokens = VecDeque::new();
    let (mut token, mut next) = next_token(input.next(), &mut input);
    loop { match token {
        Token::EOF => break,
        ref other => {
            tokens.push_back(other.copy());
            (token, next) = next_token(next, &mut input);
        }
    } }
    tokens
}

fn next_token(next: Option<char>, input: &mut Chars) -> (Token, Option<char>) {
    match next {
        None => (Token::EOF, None),
        Some(char) => {
            match char {
                '+' => (Token::Plus, input.next()),
                '-' => (Token::Minus, input.next()),
                '*' => (Token::Astarisk, input.next()),
                '/' => (Token::Slash, input.next()),
                '<' => (Token::Lt, input.next()),
                '>' => (Token::Gt, input.next()),
                ',' => (Token::Comma, input.next()),
                ';' => (Token::Semicolon, input.next()),
                '(' => (Token::Lparen, input.next()),
                ')' => (Token::Rparen, input.next()),
                '{' => (Token::Lbrace, input.next()),
                '}' => (Token::Rbrace, input.next()),
                '=' => match input.next() {
                    Some('=') => (Token::Eq, input.next()),
                    other => (Token::Assign, other),
                },
                '!' => match input.next() {
                    Some('=') => (Token::NotEq, input.next()),
                    other => (Token::Bang, other),
                },
                'f' => match input.next() {
                    Some('a') => match input.next() {
                        Some('l') => match input.next() {
                            Some('s') => match input.next() {
                                Some('e') => match input.next() {
                                    Some(' ' | '\n') => (Token::False, input.next()),
                                    None => (Token::False, None),
                                    Some(ch) => {
                                        let (word, next) = read_a_word_with("false".to_owned(), Some(ch), input);
                                            if &word=="false" { (Token::False, next) }
                                            else              { (Token::Ident(word), next) }
                                    }
                                },
                                None => (Token::Ident("fals".to_owned()), input.next()),
                                some => read_a_token_from("fals".to_owned(), some, input)
                            },
                            None => (Token::Ident("fal".to_owned()), None),
                            some => read_a_token_from("fal".to_owned(), some, input)
                        },
                        None => (Token::Ident("fa".to_owned()), None),
                        some => (read_a_token_from("fa".to_owned(), some, input))
                    },
                    Some('n') => match input.next() {
                        Some(' ' | '\n') => (Token::Functon, input.next()),
                        None => (Token::Functon, None),
                        Some(ch) => {
                            let (word, next) = read_a_word_with("fn".to_owned(), Some(ch), input);
                            if &word == "fn" { (Token::Functon, next) }
                            else             { (Token::Ident(word), next) }
                        }
                    },
                    None => (Token::Ident('f'.to_string()), None),
                    some => read_a_token_from("f".to_owned(), some, input),
                },
                'l' => match input.next() {
                    Some('e') => match input.next() {
                        Some('t') => match input.next() {
                            Some(' ' | '\n') => (Token::Let, input.next()),
                            None => (Token::Let, None),
                            Some(ch) => {
                                let (word, next) = read_a_word_with("let".to_string(), Some(ch), input);
                                if &word == "let" { (Token::Let, next) }
                                else              { (Token::Ident(word), next) }
                            },
                        },
                        None => (Token::Ident("le".to_owned()), None),
                        some => read_a_token_from("le".to_owned(), some, input),
                    },
                    None => (Token::Ident("le".to_owned()), None),
                    some => read_a_token_from("l".to_owned(), some, input)
                },
                't' => match input.next() {
                    Some('r') => match input.next() {
                        Some('u') => match input.next() {
                            Some('e') => match input.next() {
                                Some(' ' | '\n') => (Token::True, input.next()),
                                None => (Token::True, None),
                                some => {
                                    let (word, next) = read_a_word_with("true".to_owned(), some, input);
                                    if &word == "true" { (Token::True, next) }
                                    else               { (Token::Ident(word), next) }
                                },
                            },
                            None => (Token::True, None),
                            some => read_a_token_from("tre".to_owned(), some, input),
                        },
                        None => (Token::Ident("tr".to_owned()), None),
                        some => read_a_token_from("tr".to_owned(), some, input),
                    },
                    None => (Token::Ident("t".to_owned()), None),
                    some => read_a_token_from("t".to_owned(), some, input),
                },
                'e' => match input.next() {
                    Some('l') => match input.next() {
                        Some('s') => match input.next() {
                            Some('e') => (Token::Else, input.next()),
                            None      => (Token::Else, None),
                            some => {
                                let (word, next) = read_a_word_with("else".to_owned(), some, input);
                                if &word == "els" { (Token::Else, next) }
                                else               { (Token::Ident(word), next) }
                            }
                        },
                        None => (Token::Ident("el".to_owned()), None),
                        some => read_a_token_from("el".to_owned(), some, input),
                    },
                    None => (Token::Ident("e".to_owned()), None),
                    some => read_a_token_from("e".to_owned(), some, input)
                },
                'r' => match input.next() {
                    Some('e') => match input.next() {
                        Some('t') => match input.next() {
                            Some('u') => match input.next() {
                                Some('r') => match input.next() {
                                    Some('n') => match input.next() {
                                        Some(' ' | '\n') => (Token::Return, input.next()),
                                        None => (Token::Return, None),
                                        some => {
                                            let (word, next) = read_a_word_with("return".to_owned(), some, input);
                                            if &word == "return" { (Token::Return, next) }
                                            else                 { (Token::Ident(word), next) }
                                        }
                                    },
                                    None => (Token::Ident("retur".to_owned()), None),
                                    some => read_a_token_from("retur".to_owned(), some, input),
                                }
                                None => (Token::Ident("retu".to_owned()), None),
                                some => read_a_token_from("retu".to_owned(), some, input),
                            },
                            None => (Token::Ident("ret".to_owned()), None),
                            some => read_a_token_from("ret".to_owned(), some, input),
                        },
                        None => (Token::Ident("re".to_owned()), None),
                        some => read_a_token_from("re".to_owned(), some, input),
                    },
                    None => (Token::Ident("r".to_owned()), None),
                    some => read_a_token_from("r".to_owned(), some, input),
                }
                digit @ '0'..='9' => {
                    let mut digits = vec![digit.to_digit(10).unwrap() as usize];
                    let mut next = input.next();
                    while let Some(d) = next {
                        if d.is_ascii_digit() {
                            digits.push(d.to_digit(10).unwrap() as usize);
                            next = input.next();
                        } else {
                            break;
                        }
                    }
                    (Token::Int(digits.iter().fold(0, |a,b| 10 * a + b)), next)
                },
                ch @ ('a'..='z'|'A'..='Z'|'_') => {
                    let (word, next) = read_a_word_with(ch.to_string(), input.next(), input);
                    (Token::Ident(word), next)
                },
                ' ' | '\n' => next_token(input.next(), input),
                _ => (Token::Illegal, input.next())
            }
        },
    }
}

fn read_a_word_with(mut init: String, mut next: Option<char>, input: &mut Chars) -> (String, Option<char>) {
    while let Some(n) = next {
        match n {
            c @ ('a'..='z'|'A'..='Z'|'0'..='9'|'_') => {
                init.push(c);
                next = input.next();
            },
            _ => break,
        }
    }
    (init, next)
}
fn read_a_token_from(mut init: String, mut next: Option<char>, input: &mut Chars) -> (Token, Option<char>) {
    while let Some(n) = next {
        match n {
            c @ ('a'..='z'|'A'..='Z'|'0'..='9'|'_') => {
                init.push(c);
                next = input.next();
            },
            _ => break,
        }
    }
    (Token::Ident(init), next)
}