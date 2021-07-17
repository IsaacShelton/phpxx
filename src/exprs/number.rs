
use super::*;

#[derive(Clone)]
pub struct NumberExpr {
    pub value: f64
}

impl NumberExpr {
    pub fn new(value: f64) -> Box<dyn Expr> {
        Box::new(NumberExpr { value: value })
    }
}

impl Expr for NumberExpr {
    fn evaluate(&self) -> Box<dyn Expr> {
        return Box::new(self.clone())
    }

    fn stringify(&self) -> String {
        self.value.to_string()
    }
}

impl std::fmt::Display for NumberExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
