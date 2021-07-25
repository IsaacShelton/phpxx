use super::*;
use std::io::BufRead;

pub struct CallExpr {
    pub function: String,
    pub args: Vec<Expression>,
}

impl CallExpr {
    pub fn new(function: String, args: Vec<Expression>) -> Expression {
        Box::new(Self { function, args })
    }
}

impl Expr for CallExpr {
    fn as_any(&self) -> &dyn std::any::Any {
        return self;
    }

    fn evaluate(&self, ctx: &mut Ctx) -> Expression {
        let args: Vec<Expression> = self.args.iter().map(|x| x.evaluate(ctx)).collect();

        match &self.function[..] {
            "repr" => repr(&args),
            "readline" => readline(),
            "eq" => eq(&args),
            "lt" => lt(&args),
            "push" => push(ctx, &args),
            "pop" => pop(ctx, &args),
            "up" => up(ctx),
            "down" => down(ctx),
            "arr" => ArrayExpr::new(args),
            "aka" => aka(&args),
            "throw" => throw(ctx, args),
            "args" => args_impl(ctx),
            "get" => get(&args),
            "count" => count(&args),
            _ => ctx.run_function(&self.function, args),
        }
    }

    fn stringify(&self) -> String {
        panic!();
    }

    fn visualize(&self) -> String {
        let mut args_string = String::new();

        for expression in &self.args {
            if args_string.len() != 0 {
                args_string.push_str(", ");
                args_string.push_str(&expression.visualize());
            }
        }

        format!("{}({})", &self.function, &args_string)
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

impl Clone for CallExpr {
    fn clone(&self) -> Self {
        Self {
            function: self.function.clone(),
            args: self
                .args
                .iter()
                .map(|x| dyn_clone::clone_box(&**x))
                .collect(),
        }
    }
}

fn repr(args: &Vec<Expression>) -> Expression {
    let value = match args.first() {
        Some(value) => value,
        None => return VoidExpr::new(),
    };

    let value = value.as_any();

    match match_cast!(value {
        _val as StringExpr => {
            StringExpr::new("\"\"".to_string())
        },
        _val as NumberExpr => {
            StringExpr::new("0".to_string())
        },
        _val as VoidExpr => {
            StringExpr::new("void".to_string())
        },
        _val as ArrayExpr => {
            StringExpr::new("[]".to_string())
        },
    }) {
        Some(expression) => expression,
        None => VoidExpr::new(),
    }
}

fn readline() -> Expression {
    let mut line = String::new();
    let stdin = std::io::stdin();
    stdin.lock().read_line(&mut line).unwrap();

    if line.ends_with("\n") {
        line = line[0..line.len() - 1].to_string()
    }

    StringExpr::new(line)
}

fn eq(args: &Vec<Expression>) -> Expression {
    if args.len() > 2 {
        for i in 0..(args.len() - 1) {
            if !eq_impl(&args[i], &args[i + 1]) {
                return NumberExpr::new(0.0);
            }
        }
        return NumberExpr::new(1.0);
    } else if args.len() < 2 {
        return NumberExpr::new(1.0);
    }

    NumberExpr::new(if eq_impl(&args[0], &args[1]) {
        1.0
    } else {
        0.0
    })
}

fn eq_impl(a: &Expression, b: &Expression) -> bool {
    let a_any = a.as_any();

    match_cast!(a_any {
        val as StringExpr => {
            val.value == StringExpr::coerce_to_string(b)
        },
        val as NumberExpr => {
            val.value == NumberExpr::coerce_to_number(b)
        },
        _val as VoidExpr => {
            b.as_any().is::<VoidExpr>()
        },
        val as ArrayExpr => {
            if let Some(other) = b.as_any().downcast_ref::<ArrayExpr>() {
                eq_impl_arr(&val, &other)
            } else {
                false
            }
        },
    })
    .unwrap_or(false)
}

fn eq_impl_arr(a: &ArrayExpr, b: &ArrayExpr) -> bool {
    if a.uid() == b.uid() {
        // Same uid means same array and so equal
        return true;
    }

    // Strip off unnecessary abstraction, leave only &Vec<Expression> left
    let a = a.value.borrow();
    let b = b.value.borrow();

    if a.len() != b.len() {
        return false;
    }

    for i in 0..a.len() {
        if !eq_impl(&a[i], &b[i]) {
            return false;
        }
    }

    true
}

fn lt(args: &Vec<Expression>) -> Expression {
    if args.len() > 2 {
        for i in 0..(args.len() - 1) {
            if !lt_impl(&args[i], &args[i + 1]) {
                return NumberExpr::new(0.0);
            }
        }
        return NumberExpr::new(1.0);
    } else if args.len() < 2 {
        return NumberExpr::new(1.0);
    }

    NumberExpr::new(if lt_impl(&args[0], &args[1]) {
        1.0
    } else {
        0.0
    })
}

fn lt_impl(a: &Expression, b: &Expression) -> bool {
    let a_any = a.as_any();

    match_cast!(a_any {
        val as StringExpr => {
            val.value < StringExpr::coerce_to_string(b)
        },
        val as NumberExpr => {
            val.value < NumberExpr::coerce_to_number(b)
        },
        _val as VoidExpr => {
            0.0 < NumberExpr::coerce_to_number(b)
        },
        val as ArrayExpr => {
            (val.value.borrow().len() as f64) < NumberExpr::coerce_to_number(b)
        },
    })
    .unwrap_or(false)
}

fn push(ctx: &mut Ctx, args: &Vec<Expression>) -> Expression {
    if args.len() == 0 {
        ctx.push_scope(false);
    }

    VoidExpr::new()
}

fn pop(ctx: &mut Ctx, args: &Vec<Expression>) -> Expression {
    if args.len() == 0 {
        ctx.pop_scope();
    }

    VoidExpr::new()
}

fn up(ctx: &mut Ctx) -> Expression {
    ctx.up();
    VoidExpr::new()
}

fn down(ctx: &mut Ctx) -> Expression {
    ctx.down();
    VoidExpr::new()
}

fn aka(args: &Vec<Expression>) -> Expression {
    for i in 0..args.len() - 1 {
        let uid_a = uid_of(&args[i]);
        let uid_b = uid_of(&args[i + 1]);

        if uid_a.is_none() || uid_b.is_none() || uid_a.unwrap() != uid_b.unwrap() {
            return NumberExpr::new(0.0);
        }
    }

    NumberExpr::new(1.0)
}

fn uid_of(a: &Expression) -> Option<usize> {
    let a_any = a.as_any();

    match_cast!(a_any {
        val as ArrayExpr => {
            val.uid()
        },
    })
}

fn throw(ctx: &mut Ctx, args: Vec<Expression>) -> Expression {
    let mut args = args;

    ctx.throw(match args.len() {
        1 => std::mem::replace(&mut args[0], VoidExpr::new()),
        0 => VoidExpr::new(),
        _ => ArrayExpr::new(args),
    });

    VoidExpr::new()
}

fn args_impl(ctx: &mut Ctx) -> Expression {
    // Note that only one call to args() is allowed,
    // Any following calls will return an empty array
    ArrayExpr::new(std::mem::replace(&mut ctx.args, vec![]))
}

fn count(args: &Vec<Expression>) -> Expression {
    let collection = match args.first() {
        Some(arg) => arg.as_any(),
        None => return VoidExpr::new()
    };

    NumberExpr::new(match_cast!( collection {
        val as ArrayExpr => {
            val.value.borrow().len()
        },
        val as StringExpr => {
            val.value.len()
        },
    }).unwrap_or(0) as f64)
}

fn get(args: &Vec<Expression>) -> Expression {
    if args.len() != 2 {
        return VoidExpr::new()
    }

    let collection = args[0].as_any();

    match_cast!(collection {
        val as ArrayExpr => {
            let index = NumberExpr::coerce_to_number(&args[1]);

            match val.value.borrow().get(index as usize) {
                Some(value) => value.clone(),
                None => VoidExpr::new()
            }
        },
    }).unwrap_or_else(||
        VoidExpr::new()
    )
}
