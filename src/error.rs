pub struct LoxError {
    line: usize,
    message: String,
}

impl LoxError {
    pub fn error(line: usize, message: String) -> LoxError {
        let error = LoxError { line, message };
        error.report("");
        error
    }

    pub fn report(&self, loc: &str) {
        eprintln!("[line {}] Error{}: {}", self.line, loc, self.message);
    }
}
