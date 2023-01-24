use std::collections::VecDeque;

use crate::ast::Expr;
use crate::ast::Program;
use crate::lexer::tokenize;
use crate::lexer::Token;
use crate::lexer::TokenType;

pub struct Parser {
    tokens: VecDeque<Token>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            tokens: VecDeque::<Token>::new(),
        }
    }
    fn at(&self) -> Token {
        self.tokens[0].clone()
    }
    fn eat(&mut self) -> Token {
        if let Some(r) = self.tokens.pop_front() {
            return r;
        } else {
            panic!("error while eating");
        }
    }
    fn expect(&mut self, expectedtype: TokenType) -> Token {
        if let Some(prev) = self.tokens.pop_front() {
            if prev.ttype != expectedtype {
                panic!("Expected {:#?} found {:#?}", expectedtype, prev);
            }
            return prev;
        } else {
            panic!("Error expecting");
        }
    }
    fn is_eof(&mut self) -> bool {
        return self.tokens[0].ttype == TokenType::EOF;
    }
    fn parse_additive_expr(&mut self) -> Expr {
        let mut left = self.parse_multiplicative_expr();
        while self.at().value == "+" || self.at().value == "-" {
            let operator = self.eat().value;
            let right = self.parse_multiplicative_expr();
            left = Expr::BinaryExpr {
                kind: "BinaryExpr".to_string(),
                left: Box::new(left),
                right: Box::new(right),
                operator,
            }
        }
        return left;
    }
    fn parse_multiplicative_expr(&mut self) -> Expr {
        let mut left = self.parse_primary();
        while self.at().value == "/" || self.at().value == "*" || self.at().value == "%" {
            let operator = self.eat().value;
            let right = self.parse_primary();
            left = Expr::BinaryExpr {
                kind: "BinarExpr".to_string(),
                left: Box::new(left),
                right: Box::new(right),
                operator,
            }
        }
        return left;
    }
    fn parse_primary(&mut self) -> Expr {
        match self.tokens[0].ttype {
            TokenType::Number => Expr::NumberLiteral {
                kind: "NumberLiteral".to_string(),
                value: self
                    .tokens
                    .pop_front()
                    .unwrap()
                    .value
                    .parse::<i64>()
                    .unwrap(),
            },
            TokenType::Identifier => Expr::Identifier {
                kind: "Identifier".to_string(),
                value: self.tokens.pop_front().unwrap().value.to_string(),
            },
            TokenType::OpenParen => {
                self.eat();
                let value = self.parse_additive_expr();
                self.expect(TokenType::CloseParen);
                return value;
            }
            _ => panic!("Não entendi"),
        }
    }
    pub fn produce_ast(&mut self, source_code: String) -> Program {
        self.tokens = tokenize(source_code);
        let mut program: Program = Program {
            kind: "Program".to_string(),
            body: VecDeque::<Expr>::new(),
        };
        while !self.is_eof() {
            program.body.push_back(self.parse_additive_expr());
        }
        return program;
    }
}
