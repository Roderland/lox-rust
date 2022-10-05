
use crate::token::{Token, TokenType};
use crate::expr::*;
use crate::LoxError;
use crate::token::Object;
use crate::token::TokenType::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Option<Box<dyn Expr>> {
        return self.expression().ok();
    }

    fn expression(&mut self) -> Result<Box<dyn Expr>, LoxError> {
        return self.equality();
    }

    fn equality(&mut self) -> Result<Box<dyn Expr>, LoxError> {
        let left = self.comparison()?;

        while self.try_match(&[BangEqual, EqualEqual]) {
            let operator = self.previous().unwrap().clone();
            let right= self.comparison()?;
            return Ok(BinaryExpr::new(left, operator, right));
        }

        Ok(left)
    }

    fn comparison(&mut self) -> Result<Box<dyn Expr>, LoxError> {
        let left = self.term()?;

        while self.try_match(&[Greater, GreaterEqual, Less, LessEqual]) {
            let operator = self.previous().unwrap().clone();
            let right = self.term()?;
            return Ok(BinaryExpr::new(left, operator, right));
        }

        Ok(left)
    }

    fn term(&mut self) -> Result<Box<dyn Expr>, LoxError> {
        let left = self.factor()?;

        while self.try_match(&[Minus, Plus]) {
            let operator = self.previous().unwrap().clone();
            let right = self.factor()?;
            return Ok(BinaryExpr::new(left, operator, right));
        }

        Ok(left)
    }

    fn factor(&mut self) -> Result<Box<dyn Expr>, LoxError> {
        let left = self.unary()?;

        while self.try_match(&[Slash, Star]) {
            let operator = self.previous().unwrap().clone();
            let right = self.unary()?;
            return Ok(BinaryExpr::new(left, operator, right));
        }

        Ok(left)
    }

    fn unary(&mut self) -> Result<Box<dyn Expr>, LoxError> {
        if self.try_match(&[Bang, Minus]) {
            let operator = self.previous().unwrap().clone();
            let right = self.unary()?;
            return Ok(UnaryExpr::new(operator, right));
        }

        return self.primary();
    }

    fn primary(&mut self) -> Result<Box<dyn Expr>, LoxError> {
        if self.try_match(&[False]) {
            return Ok(LiteralExpr::new(Object::False));
        }
        if self.try_match(&[True]) {
            return Ok(LiteralExpr::new(Object::True));
        }
        if self.try_match(&[Nil]) {
            return Ok(LiteralExpr::new(Object::Nil));
        }
        if self.try_match(&[Number, String]) {
            let value = self.previous().unwrap().clone().literal.unwrap();
            return Ok(LiteralExpr::new(value));
        }
        if self.try_match(&[LeftParen]) {
            let expr = self.expression()?;
            self.consume(&RightParen, "Expect ')'  after expression.")?;
            return Ok(GroupingExpr::new(expr));
        }
        panic!("TODO");
    }

    fn try_match(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn advance(&mut self) -> Option<&Token> {
        self.current += 1;
        self.previous()
    }

    fn check(&self, typ: &TokenType) -> bool {
        match self.peek() {
            Some(token) if token.typ == *typ => true,
            _ => false
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn previous(&self) -> Option<&Token> {
        self.tokens.get(self.current - 1)
    }

    fn consume(&mut self, typ: &TokenType, message: &str) -> Result<Option<&Token>, LoxError> {
        if self.check(typ) {
            return Ok(self.advance());
        } else {
            let p = self.peek();
            Err(LoxError::error(p.unwrap().line, message.to_string()))
        }
    }

    fn synchronize(&mut self) {
        self.advance();

        while let Some(t) = self.peek() {
            if self.previous().unwrap().typ == SemiColon {
                return;
            }

            if matches!(t.typ, Class | Fun | Var | For | If | While | Print | Return) {
                return;
            }

            self.advance();
        }
    }
}