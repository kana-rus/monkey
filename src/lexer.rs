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
    fn from_keyword(keyword: &str) -> Token {
        match keyword {
            "fn" => Token::Functon,
            "let" => Token::Let,
            "true" => Token::True,
            "false" => Token::False,
            "else" => Token::Else,
            "return" => Token::Return,
            _ => unreachable!(),
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
                    Some('a') => expect("false", "fa".to_owned(), input.next(), input),
                    Some('n') => expect("fn", "fn".to_owned(), input.next(), input),
                    None => (Token::Ident('f'.to_string()), None),
                    some => read_a_keyword_from("f".to_owned(), some, input),
                },
                'l' => expect("let", 'l'.to_string(), input.next(), input),
                't' => expect("true", 't'.to_string(), input.next(), input),
                'e' => expect("else", 'e'.to_string(), input.next(), input),
                'r' => expect("return", 'r'.to_string(), input.next(), input),
                ch @ ('a'..='z'|'A'..='Z'|'_') => read_a_keyword_from(ch.to_string(), input.next(), input),
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
                ' ' | '\n' => next_token(input.next(), input),
                _ => (Token::Illegal, input.next())
            }
        },
    }
}




fn read_a_keyword_from(init: String, mut next: Option<char>, input: &mut Chars) -> (Token, Option<char>) {
    let mut word = init;
    while let Some(n) = next {
        match n {
            c @ ('a'..='z'|'A'..='Z'|'0'..='9'|'_') => {
                word.push(c);
                next = input.next();
            },
            _ => break,
        }
    }
    match word.as_str() {
        keyword @ ("fn"|"let"|"true"|"false"|"else"|"return") => (Token::from_keyword(keyword), next),
        _ => (Token::Ident(word), next)
    }
}

fn expect(
    keyword: &str, mut current: String, next: Option<char>, input: &mut Chars
) -> (Token, Option<char>) {
    let (current_len, keyword_len) = (current.len(), keyword.len());

    if current_len > keyword_len { panic!("current is longer than goal!"); }

    if current_len == keyword_len {
        read_a_keyword_from(keyword.to_owned(), next, input)
    } else {
        let expected_char = keyword.chars().nth(current_len + 1).unwrap();
        match next {
            None => (Token::Ident(current.clone()), None),
            Some(ch) if ch == expected_char => {
                current.push(expected_char);
                expect(keyword, current, input.next(), input)
            },
            some => {
                read_a_keyword_from(current.clone(), some, input)
            },
        }
    }
}