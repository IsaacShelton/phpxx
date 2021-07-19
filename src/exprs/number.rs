use super::*;

#[derive(Clone)]
pub struct NumberExpr {
    pub value: f64,
}

impl NumberExpr {
    pub fn new(value: f64) -> Expression {
        Box::new(Self { value: value })
    }

    pub fn coerce_to_number(other: &Expression) -> f64 {
        let other = other.as_any();
        
        match_cast!( other {
            val as NumberExpr => {
                val.value
            },
            val as StringExpr => {
                val.value.parse::<f64>().unwrap_or(0.0)
            },
        }).unwrap_or(0.0)
    }
    
    pub fn coerce_to_number_from_any(other: &dyn std::any::Any) -> f64 {
        match_cast!( other {
            val as NumberExpr => {
                val.value
            },
            val as StringExpr => {
                val.value.parse::<f64>().unwrap_or(0.0)
            },
        }).unwrap_or(0.0)
    }
}

impl Expr for NumberExpr {
    fn as_any(&self) -> &dyn std::any::Any {
        return self
    }

    fn evaluate(&self) -> Expression {
        return Box::new(self.clone());
    }

    fn stringify(&self) -> String {
        self.value.to_string()
    }

    fn visualize(&self) -> String {
        return self.value.to_string()
    }

    fn plus(&self, other: &Expression) -> Expression {
        return NumberExpr::new(self.value + Self::coerce_to_number(other))
    }
    
    fn minus(&self, other: &Expression) -> Expression {
        return NumberExpr::new(self.value - Self::coerce_to_number(other))
    }

    fn multiply(&self, other: &Expression) -> Expression {
        return NumberExpr::new(self.value * Self::coerce_to_number(other))
    }
    
    fn divide(&self, other: &Expression) -> Expression {
        return NumberExpr::new(self.value / Self::coerce_to_number(other))
    }
}
