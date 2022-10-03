use std::fmt::{self, Formatter};

#[derive(Debug)]
pub struct Token {
    typ: TokenType,
    lexeme: String,
    literal: Option<Object>,
    line: usize,
}

impl Token {
    pub fn new(typ: TokenType, lexeme: String, literal: Option<Object>, line: usize) -> Token {
        Token {
            typ,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} {} {}",
            self.typ,
            self.lexeme,
            if let Some(literal) = &self.literal {
                literal.to_string()
            } else {
                "None".to_string()
            }
        )
    }
}

#[derive(Debug)]
pub enum Object {
    Num(f64),
    Str(String),
    Nil,
    True,
    False,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Object::Num(x) => write!(f, "{x}"),
            Object::Str(x) => write!(f, "\"{x}\""),
            Object::Nil => write!(f, "nil"),
            Object::True => write!(f, "true"),
            Object::False => write!(f, "false"),
        }
    }
}

#[derive(Debug)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    String,
    Number,
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}
