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
} impl Token {
    fn copy(&self) -> Self {
        match self {
            Token::Ident(string) => Token::Ident(string.clone()),
            Token::Int(n) => Token::Int(*n),

            Token::Illegal => Token::Illegal,
            Token::EOF => Token::EOF,
            Token::Assign => Token::Assign,
            Token::Plus => Token::Plus,
            Token::Comma => Token::Comma,
            Token::Semicolon => Token::Semicolon,
            Token::Lparen => Token::Lparen,
            Token::Rparen => Token::Rparen,
            Token::Lbrace => Token::Lbrace,
            Token::Rbrace => Token::Rbrace,
            Token::Functon => Token::Functon,
            Token::Let => Token::Let,
        }
    }
}

#[allow(unused)]
pub struct Lexer<'l> {
    input:     Chars<'l>,
    current:   Token,
} impl<'l> Lexer<'l> {
    pub fn toknize(input: String) -> VecDeque<Token> {
        let mut input = input.chars();
        let mut tokens = VecDeque::new();
        let (mut token, mut next) = Self::next_token(input.next(), &mut input);
        loop { match token {
            Token::EOF => break,
            ref other => {
                tokens.push_back(other.copy());
                (token, next) = Self::next_token(next, &mut input);
            }
        } }
        tokens
    }

    fn next_token(next: Option<char>, input: &mut Chars) -> (Token, Option<char>) {
        match next {
            None => (Token::EOF, None),
            Some(char) => {
                match char {
                    '=' => (Token::Assign, input.next()),
                    '+' => (Token::Plus, input.next()),
                    ',' => (Token::Comma, input.next()),
                    ';' => (Token::Semicolon, input.next()),
                    '(' => (Token::Lparen, input.next()),
                    ')' => (Token::Rparen, input.next()),
                    '{' => (Token::Lbrace, input.next()),
                    '}' => (Token::Rbrace, input.next()),
                    'f' => {
                        match input.next() {
                            Some('n') => {
                                match input.next() {
                                    Some(' ' | '\n') => (Token::Functon, input.next()),
                                    None => (Token::Functon, None),
                                    Some(ch) => {
                                        let mut ident = "fn".to_string();
                                        let mut next = Some(ch);
                                        while let Some(n) = next {
                                            match n {
                                                c @ ('a'..='z'|'A'..='Z'|'0'..='9'|'_') => {
                                                    ident.push(c);
                                                    next = input.next();
                                                },
                                                _ => break,
                                            }
                                        }
                                        if &ident == "fn" {
                                            (Token::Functon, next)
                                        } else {
                                            (Token::Ident(ident), next)
                                        }
                                    }
                                }
                            },
                            Some(ch) => {
                                let mut ident = 'f'.to_string();
                                let mut next = Some(ch);
                                while let Some(n) = next {
                                    match n {
                                        c @ ('a'..='z'|'A'..='Z'|'0'..='9'|'_') => {
                                            ident.push(c);
                                            next = input.next();
                                        },
                                        _ => break,
                                    }
                                }
                                (Token::Ident(ident), next)
                            },
                            None => (Token::Ident('f'.to_string()), None)
                        }
                    },
                    'l' => {
                        match input.next() {
                            Some('e') => {
                                match input.next() {
                                    Some('t') => {
                                        match input.next() {
                                            Some(' ' | '\n') => (Token::Let, input.next()),
                                            None => (Token::Let, None),
                                            Some(ch) => {
                                                let mut ident = "let".to_string();
                                                let mut next = Some(ch);
                                                while let Some(n) = next {
                                                    match n {
                                                        c @ ('a'..='z'|'A'..='Z'|'0'..='9'|'_') => {
                                                            ident.push(c);
                                                            next = input.next();
                                                        },
                                                        _ => break,
                                                    }
                                                }
                                                if &ident == "let" {
                                                    (Token::Let, next)
                                                } else {
                                                    (Token::Ident(ident), next)
                                                }
                                            },
                                        }
                                        
                                    },
                                    Some(ch) => {
                                        let mut ident = "le".to_string();
                                        let mut next = Some(ch);
                                        while let Some(n) = next {
                                            match n {
                                                c @ ('a'..='z'|'A'..='Z'|'0'..='9'|'_') => {
                                                    ident.push(c);
                                                    next = input.next();
                                                },
                                                _ => break,
                                            }
                                        }
                                        (Token::Ident(ident), next)
                                    },
                                    None => (Token::Ident("le".to_owned()), None)
                                }
                            },
                            Some(ch) => {
                                let mut ident = 'l'.to_string();
                                let mut next = Some(ch);
                                while let Some(n) = next {
                                    match n {
                                        c @ ('a'..='z'|'A'..='Z'|'0'..='9'|'_') => {
                                            ident.push(c);
                                            next = input.next();
                                        },
                                        _ => break,
                                    }
                                }
                                (Token::Ident(ident), next)
                            },
                            None => (Token::Ident('l'.to_string()), None)
                        }
                    },
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
                        let mut ident = String::from(ch);
                        let mut next = input.next();
                        while let Some(n) = next {
                            match n {
                                c @ ('a'..='z'|'A'..='Z'|'0'..='9'|'_') => {
                                    ident.push(c);
                                    next = input.next();
                                },
                                _ => break,
                            }
                        }
                        (Token::Ident(ident), next)
                    },
                    ' ' | '\n' => Self::next_token(input.next(), input),
                    _ => (Token::Illegal, input.next())
                }
            },
        }
    }
}

