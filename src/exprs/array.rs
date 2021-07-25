use gc::{Gc, GcCell};
use super::*;

pub type ArrayValue = Gc<GcCell<Vec<Expression>>>;

#[derive(Clone)]
pub struct ArrayExpr {
    pub value: ArrayValue
}

impl ArrayExpr {
    pub fn new(items: Vec<Expression>) -> Expression {
        Box::new(Self { value: Gc::new(GcCell::new(items)) })
    }

    pub fn uid(&self) -> usize {
        &*self.value.borrow() as *const Vec<Expression> as usize
    }
}

impl Expr for ArrayExpr {
    fn as_any(&self) -> &dyn std::any::Any {
        return self
    }

    fn evaluate(&self, _ctx: &mut Ctx) -> Expression {
        return Box::new(Self { value: self.value.clone() });
    }

    fn stringify(&self) -> String {
        format!("[{}]", self.value.borrow().iter().map(|x| x.visualize()).collect::<Vec<String>>().join(", "))
    }

    fn visualize(&self) -> String {
        format!("[{}]", self.value.borrow().iter().map(|x| x.visualize()).collect::<Vec<String>>().join(", "))
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
