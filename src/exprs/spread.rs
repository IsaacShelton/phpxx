use super::*;

pub struct SpreadExpr {
    pub value: Expression
}

impl SpreadExpr {
    pub fn new(value: Expression) -> Expression {
        Box::new(Self { value })
    }
}

impl Expr for SpreadExpr {
    fn as_any(&self) -> &dyn std::any::Any {
        return self
    }

    fn evaluate(&self, ctx: &mut Ctx) -> Expression {
        let original = self.value.evaluate(ctx);
        let value = original.as_any();

        match_cast!(value {
            val as ArrayExpr => {
                Box::new(ArrayExpr { value: val.value.clone(), spread: true }) as Expression
            },
        }).unwrap_or(original)
    }

    fn stringify(&self) -> String {
        panic!();
    }

    fn visualize(&self) -> String {
        format!(".. ({})", self.value.visualize())
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

impl Clone for SpreadExpr {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone()
        }
    }
}
