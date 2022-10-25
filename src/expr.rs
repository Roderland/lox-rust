use crate::error::{check_number_operands, check_string_operands, RuntimeError};
use crate::object::Object;
use crate::token::*;

pub trait Expr {
    fn eval(&self) -> Result<Object, RuntimeError>;
    fn to_string(&self) -> String;
}

/// BinaryExpr
pub struct BinaryExpr {
    left: Box<dyn Expr>,
    operator: Token,
    right: Box<dyn Expr>,
}

impl BinaryExpr {
    pub fn new(left: Box<dyn Expr>, operator: Token, right: Box<dyn Expr>) -> Box<Self> {
        return Box::new(BinaryExpr {
            left,
            operator,
            right
        });
    }
}

impl Expr for BinaryExpr {
    fn eval(&self) -> Result<Object, RuntimeError> {
        let left = &self.left.eval()?;
        let right = &self.right.eval()?;
        let operator = &self.operator;

        match operator.typ {
            TokenType::EqualEqual => Ok(Object::new_bool(left == right)),
            TokenType::BangEqual => Ok(Object::new_bool(left != right)),
            TokenType::Greater => {
                check_number_operands(operator, &[left, right])?;
                Ok(Object::new_bool(left.num() > right.num()))
            }
            TokenType::GreaterEqual => {
                check_number_operands(operator, &[left, right])?;
                Ok(Object::new_bool(left.num() >= right.num()))
            }
            TokenType::Less => {
                check_number_operands(operator, &[left, right])?;
                Ok(Object::new_bool(left.num() < right.num()))
            }
            TokenType::LessEqual => {
                check_number_operands(operator, &[left, right])?;
                Ok(Object::new_bool(left.num() <= right.num()))
            }
            TokenType::Minus => {
                check_number_operands(operator, &[left, right])?;
                Ok(Object::Num(left.num() - right.num()))
            },
            TokenType::Slash => {
                check_number_operands(operator, &[left, right])?;
                Ok(Object::Num(left.num() / right.num()))
            },
            TokenType::Star => {
                check_number_operands(operator, &[left, right])?;
                Ok(Object::Num(left.num() * right.num()))
            },
            TokenType::Plus => {
                if check_number_operands(operator, &[left, right]).is_ok() {
                    Ok(Object::Num(left.num() + right.num()))
                } else if check_string_operands(operator, &[left, right]).is_ok() {
                    Ok(Object::Str(left.str().to_string() + right.str()))
                } else {
                    Err(RuntimeError::new(
                        operator.clone(),
                        "Operands must be numbers or strings.".to_string()
                    ))
                }
            }
            _ => Ok(Object::Nil)
        }
    }

    fn to_string(&self) -> String {
        let left = self.left.to_string();
        let right = self.right.to_string();
        "( ".to_string() + &self.operator.lexeme + " " + &left + " " + &right + " )"
    }
}

/// GroupingExpr
pub struct GroupingExpr {
    expression: Box<dyn Expr>,
}

impl GroupingExpr {
    pub fn new(expression: Box<dyn Expr>) -> Box<Self> {
        Box::new(GroupingExpr { expression })
    }
}

impl Expr for GroupingExpr {
    fn eval(&self) -> Result<Object, RuntimeError> {
        self.expression.eval()
    }

    fn to_string(&self) -> String {
        "( ".to_string() + &self.expression.to_string() + " )"
    }
}

/// LiteralExpr
pub struct LiteralExpr {
    value: Object,
}

impl LiteralExpr {
    pub fn new(value: Object) -> Box<Self> {
        Box::new(LiteralExpr { value })
    }
}

impl Expr for LiteralExpr {
    fn eval(&self) -> Result<Object, RuntimeError> {
        Ok(self.value.clone())
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

/// UnaryExpr
pub struct UnaryExpr {
    operator: Token,
    right: Box<dyn Expr>,
}

impl UnaryExpr {
    pub fn new(operator: Token, right: Box<dyn Expr>) -> Box<Self> {
        return Box::new(UnaryExpr { operator, right })
    }
}

impl Expr for UnaryExpr {
    fn eval(&self) -> Result<Object, RuntimeError> {
        let right = self.right.eval()?;

        match self.operator.typ {
            TokenType::Minus => {
                check_number_operands(&self.operator, &[&right])?;
                Ok(Object::Num(-right.num()))
            },
            TokenType::Bang => Ok(Object::new_bool(!right.is_true())),
            _ => Ok(Object::Nil)
        }
    }

    fn to_string(&self) -> String {
        "( ".to_string() + &self.operator.lexeme + " " + &self.right.to_string() + " )"
    }
}

#[cfg(test)]
mod tests {
    use crate::expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr};
    use crate::object::Object;
    use crate::token::{Token, TokenType};

    fn create_binary() -> BinaryExpr {
        let expr = BinaryExpr {
            left: Box::new(UnaryExpr {
                operator: Token::new(TokenType::Minus, "-", None, 1),
                right: Box::new(LiteralExpr {
                    value: Object::Num(123 as f64)
                })
            }),
            operator: Token::new(TokenType::Star, "*", None, 1),
            right: Box::new(GroupingExpr {
                expression: Box::new(LiteralExpr {
                    value: Object::Num(45.67)
                }),
            }),
        };
        return expr;
    }

    #[test]
    fn print_ast() {
        println!("{}", create_binary().to_string())
    }

    #[test]
    fn test_eval() {
        let result = create_binary().eval();
        assert!(result.is_ok());
        println!("{}", result.ok().unwrap())
    }
}
