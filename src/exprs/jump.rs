use super::*;

#[derive(Clone)]
pub struct JumpExpr {
    pub address: usize
}

impl JumpExpr {
    pub fn new(address: usize) -> Expression {
        Box::new(Self { address })
    }
}

impl Expr for JumpExpr {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn evaluate(&self, ctx: &mut Ctx) -> Expression {
        ctx.jump = Some(self.address);
        VoidExpr::new()
    }

    fn stringify(&self) -> String {
        format!("JMP {}", self.address)
    }

    fn visualize(&self) -> String {
        format!("JMP {}", self.address)
    }

    fn plus(&self, _other: &Expression) -> Expression {
        panic!();
    }
    
    fn minus(&self, _other: &Expression) -> Expression {
        panic!();
    }

    fn multiply(&self, _other: &Expression) -> Expression {
        panic!();
    }
    
    fn divide(&self, _other: &Expression) -> Expression {
        panic!();
    }
}
