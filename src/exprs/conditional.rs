use super::*;

pub struct ConditionalExpr {
    pub condition: Expression,
    pub when_true: Vec<Expression>,
    pub when_false: Vec<Expression>,
    pub is_while: bool
}

impl ConditionalExpr {
    pub fn new(
        condition: Expression,
        when_true: Vec<Expression>,
        when_false: Vec<Expression>,
        is_while: bool
    ) -> Expression {
        Box::new(Self {
            condition,
            when_true,
            when_false,
            is_while
        })
    }

    pub fn visualize_block(block: &Vec<Expression>) -> String {
        let mut result = String::new();

        for i in block.iter() {
            result.push_str(&i.visualize());
            result.push_str("\n");
        }

        result
    }
}

impl Expr for ConditionalExpr {
    fn as_any(&self) -> &dyn std::any::Any {
        return self;
    }

    fn evaluate(&self, ctx: &mut Ctx) -> Expression {
        let mut first_time: bool = true;

        loop  {
            let is_true = NumberExpr::is_true(&self.condition.evaluate(ctx));
            let chosen_statements = if is_true {
                Some(&self.when_true)
            } else {
                if first_time {
                    Some(&self.when_false)
                } else {
                    None
                }
            };

            ctx.push_scope(false);

            if chosen_statements.is_some() {
                for statement in chosen_statements.unwrap().iter() {
                    statement.evaluate(ctx);
                }
            }

            ctx.pop_scope();

            if !self.is_while || !is_true {
                break;
            }

            first_time = false;
        }
        
        VoidExpr::new()
    }

    fn stringify(&self) -> String {
        panic!();
    }

    fn visualize(&self) -> String {
        let name = if self.is_while {
            "while"
        } else {
            "if"
        };

        format!(
            "{} {} {{\n{}}} else {{\n{}}}",
            name,
            self.condition.visualize(),
            ConditionalExpr::visualize_block(&self.when_true),
            ConditionalExpr::visualize_block(&self.when_false)
        )
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

impl Clone for ConditionalExpr {
    fn clone(&self) -> Self {
        Self {
            condition: self.condition.clone(),
            when_true: self.when_true.clone(),
            when_false: self.when_false.clone(),
            is_while: self.is_while
        }
    }
}
