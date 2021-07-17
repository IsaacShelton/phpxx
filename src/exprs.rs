
mod string;
mod number;
mod void;
mod echo;

pub use self::string::StringExpr;
pub use self::number::NumberExpr;
pub use self::void::VoidExpr;
pub use self::echo::EchoExpr;

use dyn_clone::DynClone;

pub trait Expr: DynClone {
    fn evaluate(&self) -> Box<dyn Expr>;
    fn stringify(&self) -> String;
}
