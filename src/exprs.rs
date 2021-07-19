mod echo;
mod math;
mod number;
mod string;
mod void;

pub type Expression = Box<dyn Expr>;

pub use super::lex::Token;
pub use self::echo::EchoExpr;
pub use self::number::NumberExpr;
pub use self::string::StringExpr;
pub use self::void::VoidExpr;
pub use self::math::MathExpr;

use dyn_clone::DynClone;
use match_cast::match_cast;

pub trait Expr: DynClone {
    fn as_any(&self) -> &dyn std::any::Any;
    fn evaluate(&self) -> Expression;
    fn stringify(&self) -> String;
    fn visualize(&self) -> String;
    fn plus(&self, other: &Expression) -> Expression;
    fn minus(&self, other: &Expression) -> Expression;
    fn multiply(&self, other: &Expression) -> Expression;
    fn divide(&self, other: &Expression) -> Expression;
}

impl std::fmt::Debug for dyn Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", self.visualize())
    }
}
