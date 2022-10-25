use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::LoxError::{Runtime, Syntax};
use crate::object::Object;
use crate::token::{Token};

#[derive(Debug)]
pub enum LoxError {
    Syntax(SyntaxError),
    Runtime(RuntimeError),
}

impl Display for LoxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Syntax(e) => write!(f, "{}", e),
            Runtime(e) => write!(f, "{}", e),
        }
    }
}

impl Error for LoxError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Syntax(e) => Some(e),
            Runtime(e) => Some(e),
        }
    }
}

#[derive(Debug)]
pub struct SyntaxError {
    line: usize,
    message: String,
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Syntax error: [line {}] {}.", self.line, self.message)
    }
}

impl Error for SyntaxError {
    // fn source(&self) -> Option<&(dyn Error + 'static)> {
    //     Some(self)
    // }
}

impl SyntaxError {
    pub fn new(line: usize, message: String) -> Self {
        SyntaxError { line, message }
    }
    // pub fn error(line: usize, message: String) -> SyntaxError {
    //     let error = SyntaxError { line, message };
    //     error.report("");
    //     error
    // }
    //
    // pub fn report(&self, loc: &str) {
    //     eprintln!("[line {}] Error{}: {}", self.line, loc, self.message);
    // }
}

#[derive(Debug)]
pub struct RuntimeError {
    token: Token,
    message: String,
}

impl RuntimeError {
    pub fn new(token: Token, message: String) -> Self {
        RuntimeError { token, message }
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Runtime error: [line {}] {}.", self.token.line, self.message)
    }
}

impl Error for RuntimeError {
    // fn source(&self) -> Option<&(dyn Error + 'static)> {
    //     Some(self)
    // }
}

pub fn check_number_operands(operator: &Token, nums: &[&Object]) -> Result<(), RuntimeError> {
    for num in nums {
        if !num.is_num() {
            return Err(RuntimeError::new(
                operator.clone(),
                "Operands must be numbers.".to_string()
            ));
        }
    }
    Ok(())
}

pub fn check_string_operands(operator: &Token, str_arr: &[&Object]) -> Result<(), RuntimeError> {
    for str in str_arr {
        if !str.is_str() {
            return Err(RuntimeError::new(
                operator.clone(),
                "Operands must be strings.".to_string()
            ));
        }
    }
    Ok(())
}
