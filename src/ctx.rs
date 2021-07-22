use super::exprs::{Expression, VoidExpr};
use std::collections::HashMap;

pub struct Ctx<'a> {
    pub contents: &'a str,
    pub scopes: Vec<Scope>,
}

pub struct Scope {
    pub variables: HashMap<String, Expression>,
    pub is_hard: bool
}

impl Scope {
    pub fn new(is_hard: bool) -> Self {
        Self { variables: HashMap::new(), is_hard }
    }
}

impl Ctx<'_> {
    pub fn new(contents: &str) -> Ctx {
        Ctx { contents, scopes: vec![Scope::new(true)] }
    }

    pub fn set_variable(&mut self, variable: String, value: Expression) {
        let scope = match self.scopes.last_mut() {
            Some(scope) => scope,
            _ => return ()
        };

        scope.variables.insert(variable, value);
    }

    pub fn get_variable(&self, variable: &str) -> Expression {
        let mut depth: usize = 0;

        if self.scopes.len() == 0 {
            return VoidExpr::new();
        }
        
        loop  {
            let scope = &self.scopes[self.scopes.len() - depth - 1];

            match scope.variables.get(variable) {
                Some(value) => return dyn_clone::clone_box(&**value),
                _ => ()
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
            _ => false
        } {
            self.scopes.pop();
        }

        if self.scopes.len() > 1 && match self.scopes.last() {
            Some(scope) => scope.is_hard,
            _ => false
        } {
            self.scopes.pop();
        }
    }
}
