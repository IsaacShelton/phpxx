mod array;
mod assign;
mod call;
mod echo;
mod jump;
mod conditional;
mod math;
mod number;
mod string;
mod variable;
mod void;

pub type Expression = Box<dyn Expr>;

pub use super::ctx::Ctx;
pub use super::lex::Token;
pub use self::array::{ArrayValue, ArrayExpr};
pub use self::assign::AssignExpr;
pub use self::call::CallExpr;
pub use self::echo::EchoExpr;
pub use self::jump::JumpExpr;
pub use self::conditional::ConditionalExpr;
pub use self::number::NumberExpr;
pub use self::string::StringExpr;
pub use self::void::VoidExpr;
pub use self::math::MathExpr;
pub use self::variable::VariableExpr;

use dyn_clone::DynClone;
use match_cast::match_cast;
use gc::{Trace, Finalize, custom_trace};

pub trait Expr: DynClone {
    fn as_any(&self) -> &dyn std::any::Any;
    fn evaluate(&self, ctx: &mut Ctx) -> Expression;
    fn stringify(&self) -> String;
    fn visualize(&self) -> String;
    fn plus(&self, other: &Expression) -> Expression;
    fn minus(&self, other: &Expression) -> Expression;
    fn multiply(&self, other: &Expression) -> Expression;
    fn divide(&self, other: &Expression) -> Expression;
}

impl std::fmt::Debug for dyn Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.visualize())
    }
}

impl Finalize for dyn Expr {}

unsafe impl Trace for dyn Expr {
    custom_trace!(this, {
        let this = this.as_any();

        match_cast!(this {
            val as ArrayExpr => {
                // Since Vec<T> implements Trace, the items will automatically be marked
                mark(&val.value);
            },
        });
    });
}

impl Clone for Expression {
    fn clone(&self) -> Self {
        dyn_clone::clone_box(&**self)
    }
}
