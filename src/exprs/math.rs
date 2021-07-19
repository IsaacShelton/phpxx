use super::*;

pub struct MathExpr {
    pub lhs: Expression,
    pub operator: Token,
    pub rhs: Expression,
}

impl MathExpr {
    pub fn new(lhs: Expression, operator: &Token, rhs: Expression) -> Expression {
        Box::new(Self { lhs, operator: *operator, rhs })
    }
}

impl Expr for MathExpr {
    fn as_any(&self) -> &dyn std::any::Any {
        return self
    }

    fn evaluate(&self) -> Expression {
        let l = self.lhs.evaluate();
        let r = self.rhs.evaluate();
        match self.operator {
            Token::Plus => l.plus(&r),
            Token::Minus => l.minus(&r),
            Token::Multiply => l.multiply(&r),
            Token::Divide => l.divide(&r),
            _ => {
                unimplemented!();
            }
        }
    }

    fn stringify(&self) -> String {
        panic!();
    }

    fn visualize(&self) -> String {
        let binary_op = match self.operator {
            Token::Plus => "+",
            Token::Multiply => "*",
            _ => "<?>"
        };
        format!("({} {} {})", self.lhs.visualize(), binary_op, self.rhs.visualize())
    }

    fn plus(&self, other: &Expression) -> Expression {
        self.evaluate().plus(&other.evaluate())
    }
    
    fn minus(&self, other: &Expression) -> Expression {
        self.evaluate().minus(&other.evaluate())
    }

    fn multiply(&self, other: &Expression) -> Expression {
        self.evaluate().minus(&other.evaluate())
    }
    
    fn divide(&self, other: &Expression) -> Expression {
        self.evaluate().minus(&other.evaluate())
    }
}

impl Clone for MathExpr {
    fn clone(&self) -> Self {
        Self {
            lhs: dyn_clone::clone_box(&*self.lhs),
            operator: self.operator,
            rhs: dyn_clone::clone_box(&*self.rhs),
        }
    }
}
