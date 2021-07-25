use super::*;

pub struct AssignExpr {
    pub variable: String,
    pub value: Expression
}

impl AssignExpr {
    #[allow(dead_code)]
    pub fn new(variable: String, value: Expression) -> Expression {
        Box::new(Self { variable, value })
    }
}

impl Expr for AssignExpr {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn evaluate(&self, ctx: &mut Ctx) -> Expression {
        let value = self.value.evaluate(ctx);
        ctx.set_variable(self.variable.to_string(), value);
        VoidExpr::new()
    }

    fn stringify(&self) -> String {
        panic!();
    }

    fn visualize(&self) -> String {
        format!("{} = {}", self.variable, self.value.visualize()) 
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

impl Clone for AssignExpr {
    fn clone(&self) -> Self {
        Self {
            variable: self.variable.clone(),
            value: dyn_clone::clone_box(&*self.value)
        }
    }
}
