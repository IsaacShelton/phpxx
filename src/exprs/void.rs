
use super::*;

#[derive(Clone)]
pub struct VoidExpr {}

impl VoidExpr {
    pub fn new() -> Box<dyn Expr> {
        Box::new(VoidExpr {})
    }
}

impl Expr for VoidExpr {
    fn evaluate(&self) -> Box<dyn Expr> {
        return Box::new(VoidExpr {})
    }

    fn stringify(&self) -> String {
        panic!("Cannot stringify VoidExpr");
    }
}

impl std::fmt::Display for VoidExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", "void")
    }
}
