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

    fn evaluate(&self, ctx: &mut Ctx) -> Expression {
        let l = self.lhs.evaluate(ctx);
        let r = self.rhs.evaluate(ctx);
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

    fn plus(&self, _other: &Expression) -> Expression {
        VoidExpr::new()
    }
    
    fn minus(&self, _other: &Expression) -> Expression {
        VoidExpr::new()
    }

    fn multiply(&self, _other: &Expression) -> Expression {
        VoidExpr::new()
    }
    
    fn divide(&self, _other: &Expression) -> Expression {
        VoidExpr::new()
    }
}

impl Clone for MathExpr {
    fn clone(&self) -> Self {
        Self {
            lhs: self.lhs.clone(),
            operator: self.operator,
            rhs: self.rhs.clone(),
        }
    }
}
