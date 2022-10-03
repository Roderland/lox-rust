use crate::token::{Object, Token, TokenType};
use crate::LoxError;

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: Vec<char>) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxError> {
        let mut had_error: Option<LoxError> = None;
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => {}
                Err(e) => {
                    e.report("".to_string());
                    had_error = Some(e);
                }
            }
        }

        self.add_token_eof();
        if let Some(e) = had_error {
            Err(e)
        } else {
            Ok(&self.tokens)
        }
    }

    fn scan_token(&mut self) -> Result<(), LoxError> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::SemiColon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let token = if self.try_match('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token);
            }
            '=' => {
                let token = if self.try_match('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token);
            }
            '>' => {
                let token = if self.try_match('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token);
            }
            '<' => {
                let token = if self.try_match('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token);
            }
            '/' => {
                if self.try_match('/') {
                    loop {
                        match self.peek() {
                            Some(ch) if ch != '\n' => {
                                self.advance();
                            }
                            _ => break,
                        }
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string()?,
            _ => {
                if Scanner::is_digit(Some(c)) {
                    self.number();
                } else if Scanner::is_alpha(Some(c)) {
                    self.identifier();
                } else {
                    return Err(LoxError::error(
                        self.line,
                        "Unexpected character".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }

    fn string(&mut self) -> Result<(), LoxError> {
        loop {
            match self.peek() {
                Some('"') => break,
                Some(ch) => {
                    if ch == '\n' {
                        self.line += 1;
                    }
                    self.advance();
                }
                None => {
                    return Err(LoxError::error(
                        self.line,
                        "Unterminated string".to_string(),
                    ));
                }
            }
        }
        self.advance();
        let string: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token_object(TokenType::String, Some(Object::Str(string)));
        Ok(())
    }

    fn number(&mut self) {
        while Scanner::is_digit(self.peek()) {
            self.advance();
        }

        match self.peek() {
            Some('.') if Scanner::is_digit(self.peek_next()) => {
                self.advance();
                while Scanner::is_digit(self.peek()) {
                    self.advance();
                }
            }
            _ => {}
        }

        let number = self.source[self.start..self.current]
            .iter()
            .collect::<String>()
            .parse()
            .unwrap();
        self.add_token_object(TokenType::Number, Some(Object::Num(number)));
    }

    fn identifier(&mut self) {
        while Scanner::is_alpha_numeric(self.peek()) {
            self.advance();
        }
        if let Some(typ) = Scanner::keywords(
            &self.source[self.start..self.current]
                .iter()
                .collect::<String>(),
        ) {
            self.add_token(typ);
        } else {
            self.add_token(TokenType::Identifier);
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        *self.source.get(self.current - 1).unwrap()
    }

    fn try_match(&mut self, expected: char) -> bool {
        match self.source.get(self.current) {
            Some(ch) if *ch == expected => {
                self.current += 1;
                true
            }
            _ => false,
        }
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.current).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.source.get(self.current + 1).copied()
    }

    fn add_token(&mut self, typ: TokenType) {
        self.add_token_object(typ, None);
    }

    fn add_token_object(&mut self, typ: TokenType, literal: Option<Object>) {
        let lexeme = self.source[self.start..self.current]
            .iter()
            .collect::<String>();
        self.tokens
            .push(Token::new(typ, lexeme, literal, self.line));
    }

    fn add_token_eof(&mut self) {
        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), None, self.line));
    }

    fn is_digit(ch: Option<char>) -> bool {
        matches!(ch, Some('0'..='9'))
    }

    fn is_alpha(ch: Option<char>) -> bool {
        matches!(ch, Some('a'..='z' | 'A'..='Z' | '_'))
    }

    fn is_alpha_numeric(ch: Option<char>) -> bool {
        Scanner::is_alpha(ch) || Scanner::is_digit(ch)
    }

    fn keywords(word: &str) -> Option<TokenType> {
        match word {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "fun" => Some(TokenType::Fun),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            _ => None,
        }
    }
}
