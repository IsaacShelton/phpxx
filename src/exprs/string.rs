
use super::*;

#[derive(Clone)]
pub struct StringExpr {
    pub value: String
}

impl StringExpr {
    pub fn new(contents: String) -> Box<dyn Expr> {
        Box::new(StringExpr { value: contents })
    }
}

impl Expr for StringExpr {
    fn evaluate(&self) -> Box<dyn Expr> {
        Box::new(self.clone())
    }

    fn stringify(&self) -> String {
        self.value.clone()
    }
}

impl std::fmt::Display for StringExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\"{}\"", snailquote::escape(&self.value))
    }
}
