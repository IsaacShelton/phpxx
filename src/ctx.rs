use super::exprs::{Expression, VoidExpr, ArrayExpr};
use std::collections::HashMap;
use match_cast::match_cast;

pub struct Ctx<'a> {
    pub contents: &'a str,
    pub scopes: Vec<Scope>,
    pub functions: HashMap<String, Function>,
    pub statements: Option<&'a Vec<Expression>>,
    pub threw: bool,
    pub thrown: Expression,
    pub args: Vec<Expression>,
    pub parsing_function: Option<usize>,
    pub jump: Option<usize>,
}

pub struct Function {
    address: usize,
    args: Vec<String>,
}

pub struct Scope {
    pub variables: HashMap<String, Expression>,
    pub is_hard: bool,
}

impl Scope {
    pub fn new(is_hard: bool) -> Self {
        Self {
            variables: HashMap::new(),
            is_hard,
        }
    }
}

impl<'a> Ctx<'a> {
    pub fn new(contents: &str) -> Ctx {
        Ctx {
            contents,
            scopes: vec![Scope::new(true)],
            functions: HashMap::new(),
            statements: None,
            threw: false,
            thrown: VoidExpr::new(),
            args: vec![],
            parsing_function: None,
            jump: None,
        }
    }

    pub fn prep_run(&mut self) {}

    pub fn run(&mut self, statements: &'a Vec<Expression>) {
        let mut statement_index = 0;
        self.statements = Some(statements);

        while statement_index < statements.len() {
            statements[statement_index].evaluate(self);

            if self.threw {
                break;
            }

            match self.jump {
                Some(index) => {
                    statement_index = index;
                    self.jump = None;
                }
                _ => statement_index += 1,
            }
        }
    }

    pub fn run_function(&mut self, name: &str, raw_args: Vec<Expression>) -> Expression {
        let mut statement_index;

        let mut function_args = match self.functions.get(name) {
            Some(function) => {
                statement_index = function.address;
                function.args.clone()
            },
            None => {
                return VoidExpr::new()
            }
        };

        let mut args: Vec<Expression> = Vec::new();

        for arg in raw_args {
            let arg_any = arg.as_any();

            // TODO: Figure out a way to do this without *const
            let additional: Option<Option<*const [Expression]>> = match_cast!(arg_any {
                val as ArrayExpr => {
                    if val.spread {
                        Some(&val.value.borrow()[..] as *const [Expression])
                    } else {
                        None
                    }
                },
            });
            
            // TODO: Figure out a way to do this without "unsafe"
            match additional {
                Some(Some(raw_slice)) => {
                    args.extend_from_slice(unsafe {&*raw_slice} );
                },
                _ => {
                    args.push(arg);
                }
            }
        }

        // Bind function arguments to variables
        self.up();
        let bound = std::cmp::min(function_args.len(), args.len());
        for i in 0..bound {
            let name = function_args.remove(0);
            let value = args[i].clone();
            self.set_variable_here(name, value);
        }

        let statements = self.statements.unwrap();
        let previous_args = std::mem::replace(&mut self.args, args);

        while statement_index < statements.len() {
            statements[statement_index].evaluate(self);

            if self.threw {
                break;
            } else {
                statement_index += 1;
            }
        }

        let return_value = if self.threw {
            self.catch()
        } else {
            VoidExpr::new()
        };

        self.args = previous_args;
        self.down();
        return_value
    }

    pub fn throw(&mut self, value: Expression) {
        self.threw = true;
        self.thrown = value;
    }

    pub fn catch(&mut self) -> Expression {
        assert_eq!(self.threw, true);
        self.threw = false;
        std::mem::replace(&mut self.thrown, VoidExpr::new())
    }

    pub fn add_function(&mut self, name: String, address: usize, args: Vec<String>) {
        // Duplicate functions will be overwritten
        self.functions.insert(name, Function { address, args });
    }

    pub fn set_variable(&mut self, variable: String, value: Expression) {
        let mut depth: usize = 0;

        if self.scopes.len() == 0 {
            return;
        }

        loop {
            let scope_index = self.scopes.len() - depth - 1;
            let scope = &mut self.scopes[scope_index];

            if scope.variables.get(&variable).is_some() {
                scope.variables.insert(variable, value);
                return;
            }

            if scope.is_hard {
                // Don't continue down the scope stack
                break;
            } else {
                depth += 1;
            }
        }

        self.scopes.last_mut().unwrap().variables.insert(variable, value);
    }

    pub fn set_variable_here(&mut self, variable: String, value: Expression) {
        self.scopes.last_mut().unwrap().variables.insert(variable, value);
    }

    pub fn get_variable(&self, variable: &str) -> Expression {
        let mut depth: usize = 0;

        if self.scopes.len() == 0 {
            return VoidExpr::new();
        }

        loop {
            let scope = &self.scopes[self.scopes.len() - depth - 1];

            match scope.variables.get(variable) {
                Some(value) => return value.clone(),
                _ => (),
            }

            if scope.is_hard {
                // Don't continue down the scope stack
                break;
            } else {
                depth += 1;
            }
        }

        VoidExpr::new()
    }

    #[allow(dead_code)]
    pub fn push_scope(&mut self, is_hard: bool) {
        self.scopes.push(Scope::new(is_hard));
    }

    #[allow(dead_code)]
    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }
    #[allow(dead_code)]
    pub fn up(&mut self) {
        self.push_scope(true);
    }

    #[allow(dead_code)]
    pub fn down(&mut self) {
        while match self.scopes.last() {
            Some(scope) => !scope.is_hard,
            _ => false,
        } {
            self.scopes.pop();
        }

        if self.scopes.len() > 1
            && match self.scopes.last() {
                Some(scope) => scope.is_hard,
                _ => false,
            }
        {
            self.scopes.pop();
        }
    }
}
