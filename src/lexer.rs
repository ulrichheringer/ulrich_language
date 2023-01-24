#![allow(dead_code)]
use core::panic;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    Number,
    Identifier,
    Let,
    //Const,
    Equals,
    BinaryOperator,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Semicolon,
    Comma,
    Dot,
    EOF,
}

pub fn keywords() -> HashMap<&'static str, TokenType> {
    return HashMap::from([("let", TokenType::Let)]);
}

#[derive(Debug, Clone)]
pub struct Token {
    pub value: String,
    pub ttype: TokenType,
}

impl Token {
    pub fn new(value: String, ttype: TokenType) -> Self {
        Self { value, ttype }
    }
}

fn split(src: String) -> VecDeque<char> {
    let mut array = VecDeque::<char>::new();
    for i in src.chars() {
        array.push_back(i);
    }
    array
}

fn push_token(t: &mut VecDeque<Token>, s: &mut VecDeque<char>, ttype: TokenType) {
    if let Some(r) = s.pop_front() {
        t.push_back(Token::new(r.to_string(), ttype));
    }
}

pub fn tokenize(source_code: String) -> VecDeque<Token> {
    // Token array
    let mut t: VecDeque<Token> = VecDeque::new();
    let mut s = split(source_code);
    let mut line: u8 = 1;
    while s.len() > 0 {
        if s[0] == '(' {
            push_token(&mut t, &mut s, TokenType::OpenParen);
        } else if s[0] == ')' {
            push_token(&mut t, &mut s, TokenType::CloseParen);
        } else if s[0] == '{' {
            push_token(&mut t, &mut s, TokenType::OpenBrace);
        } else if s[0] == '}' {
            push_token(&mut t, &mut s, TokenType::CloseBrace);
        } else if s[0] == '[' {
            push_token(&mut t, &mut s, TokenType::OpenBracket);
        } else if s[0] == ']' {
            push_token(&mut t, &mut s, TokenType::CloseBracket);
        } else if s[0] == '+' || s[0] == '-' || s[0] == '*' || s[0] == '/' || s[0] == '%' {
            push_token(&mut t, &mut s, TokenType::BinaryOperator);
        } else if s[0] == '=' {
            push_token(&mut t, &mut s, TokenType::Equals);
        } else if s[0] == ';' {
            push_token(&mut t, &mut s, TokenType::Semicolon);
        } else if s[0] == ',' {
            push_token(&mut t, &mut s, TokenType::Comma);
        } else if s[0] == '.' {
            push_token(&mut t, &mut s, TokenType::Dot);
        } else {
            if s[0].is_numeric() {
                let mut num: String = String::new();
                while s.len() > 0 && s[0].is_numeric() {
                    if let Some(r) = s.pop_front() {
                        num.push(r);
                    }
                }
                t.push_back(Token::new(num, TokenType::Number));
            } else if s[0].is_alphabetic() {
                let mut ident: String = String::new();
                while s.len() > 0 && s[0].is_alphabetic() {
                    if let Some(r) = s.pop_front() {
                        ident.push(r);
                    }
                }
                let keywords_hash = keywords();
                if let Some(&r) = keywords_hash.get(ident.as_str()) {
                    t.push_back(Token::new(ident, r));
                } else {
                    t.push_back(Token::new(ident, TokenType::Identifier));
                }
            } else if s[0] == '\t' || s[0] == ' ' {
                s.pop_front().unwrap();
            } else if s[0] == '\n' {
                line += 1;
            } else {
                panic!("Unrecognised character {} at line {line}", s[0]);
            }
        }
    }
    t.push_back(Token::new("EndOfFile".to_string(), TokenType::EOF));
    return t;
}
