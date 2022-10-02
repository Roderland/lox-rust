use crate::LoxError;
use crate::token::{Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(
            Token::new(TokenType::Eof, "".to_string(), None, self.line)
        );
        Ok(&self.tokens)
    }

    fn scan_token(&self) {}

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

