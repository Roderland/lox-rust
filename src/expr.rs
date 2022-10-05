use crate::token::*;

pub trait Expr {
    fn eval(&self) -> Object;
    fn to_string(&self) -> String;
}

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
    fn eval(&self) -> Object {
        todo!()
    }

    fn to_string(&self) -> String {
        let left = self.left.to_string();
        let right = self.right.to_string();
        "( ".to_string() + &self.operator.lexeme + " " + &left + " " + &right + " )"
    }
}

pub struct GroupingExpr {
    expression: Box<dyn Expr>,
}

impl GroupingExpr {
    pub fn new(expression: Box<dyn Expr>) -> Box<Self> {
        Box::new(GroupingExpr { expression })
    }
}

impl Expr for GroupingExpr {
    fn eval(&self) -> Object {
        todo!()
    }

    fn to_string(&self) -> String {
        "( ".to_string() + &self.expression.to_string() + " )"
    }
}

pub struct LiteralExpr {
    value: Object,
}

impl LiteralExpr {
    pub fn new(value: Object) -> Box<Self> {
        Box::new(LiteralExpr { value })
    }
}

impl Expr for LiteralExpr {
    fn eval(&self) -> Object {
        todo!()
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

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
    fn eval(&self) -> Object {
        todo!()
    }

    fn to_string(&self) -> String {
        "( ".to_string() + &self.operator.lexeme + " " + &self.right.to_string() + " )"
    }
}

#[cfg(test)]
mod tests {
    use crate::expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr};
    use crate::token::{Object, Token, TokenType};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn print_ast() {
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
        println!("{}", expr.to_string())
    }
}
