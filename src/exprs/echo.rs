use super::*;
use std::io::Write;

pub struct EchoExpr {
    pub value: Expression,
    pub newline: bool
}

impl EchoExpr {
    pub fn new(value: Expression, newline: bool) -> Expression {
        Box::new(Self { value, newline })
    }
}

impl Expr for EchoExpr {
    fn as_any(&self) -> &dyn std::any::Any {
        return self;
    }

    fn evaluate(&self, ctx: &mut Ctx) -> Expression {
        let message = self.value.evaluate(ctx).stringify();

        if self.newline {
            println!("{}", message);
        } else {
            print!("{}", message);
            std::io::stdout().flush().unwrap();
        }
        
        VoidExpr::new()
    }

    fn stringify(&self) -> String {
        panic!();
    }

    fn visualize(&self) -> String {
        if self.newline {
            format!("echo {}", self.value.visualize())
        } else {
            format!("echo -n {}", self.value.visualize()) 
        }
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

impl Clone for EchoExpr {
    fn clone(&self) -> Self {
        Self {
            value: dyn_clone::clone_box(&*self.value),
            newline: self.newline
        }
    }
}
