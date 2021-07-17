
use super::*;

pub struct EchoExpr {
    pub value: Box<dyn Expr>
}

impl EchoExpr {
    pub fn new(value: Box<dyn Expr>) -> Box<dyn Expr> {
        Box::new(EchoExpr {value: value})
    }
}

impl Expr for EchoExpr {
    fn evaluate(&self) -> Box<dyn Expr> {
        let message = self.value.evaluate().stringify();
        println!("{}", message);
        VoidExpr::new()
    }

    fn stringify(&self) -> String {
        panic!("Cannot stringify EchoExpr");
    }
}

impl Clone for EchoExpr {
    fn clone(&self) -> Self {
        EchoExpr{ value: dyn_clone::clone_box(&*self.value) }
    }
}
