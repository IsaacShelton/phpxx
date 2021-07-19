use super::*;

#[derive(Clone)]
pub struct VoidExpr {}

impl VoidExpr {
    pub fn new() -> Expression {
        Box::new(Self {})
    }
}

impl Expr for VoidExpr {
    fn as_any(&self) -> &dyn std::any::Any {
        return self
    }

    fn evaluate(&self) -> Expression {
        return Box::new(VoidExpr {});
    }

    fn stringify(&self) -> String {
        String::from("<void>")
    }

    fn visualize(&self) -> String {
        String::from("void")
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

impl std::fmt::Display for VoidExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", "void")
    }
}
