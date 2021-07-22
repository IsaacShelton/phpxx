use super::*;

#[derive(Clone)]
pub struct VariableExpr {
    pub name: String,
}

impl VariableExpr {
    pub fn new(name: String) -> Expression {
        Box::new(Self { name })
    }
}

impl Expr for VariableExpr {
    fn as_any(&self) -> &dyn std::any::Any {
        return self
    }

    fn evaluate(&self, ctx: &mut Ctx) -> Expression {
        ctx.get_variable(&self.name)
    }

    fn stringify(&self) -> String {
        self.name.clone()
    }

    fn visualize(&self) -> String {
        self.name.clone()
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
