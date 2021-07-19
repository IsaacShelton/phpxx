use super::*;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone)]
pub struct StringExpr {
    pub value: String,
}

impl StringExpr {
    pub fn new(contents: String) -> Expression {
        Box::new(Self { value: contents })
    }

    pub fn coerce_to_string(other: &Expression) -> String {
        let other = other.as_any();

        match_cast!( other {
            val as StringExpr => {
                val.value.clone()
            },
            val as NumberExpr => {
                val.stringify()
            },
        }).unwrap_or(String::from(""))
    }

    pub fn multiply_string(string: &str, times: usize) -> String {
        let mut result = String::new();

        for _ in 0..(times as usize) {
            result.push_str(string);
        }
        
        result
    }
}

impl Expr for StringExpr {
    fn as_any(&self) -> &dyn std::any::Any {
        return self
    }

    fn evaluate(&self) -> Expression {
        Box::new(self.clone())
    }

    fn stringify(&self) -> String {
        self.value.clone()
    }

    fn visualize(&self) -> String {
        snailquote::escape(&self.value).to_string()
    }

    fn plus(&self, other: &Expression) -> Expression {
        return StringExpr::new(format!("{}{}", self.value, Self::coerce_to_string(other)))
    }
    
    fn minus(&self, original: &Expression) -> Expression {
        let other = original.as_any();

        let value = match_cast!( other {
            _val as StringExpr => {
                NumberExpr::new(NumberExpr::coerce_to_number_from_any(self.as_any()) - NumberExpr::coerce_to_number(original))
            },
            _val as NumberExpr => {
                let amount = NumberExpr::coerce_to_number(original) as i64;

                let result = if amount < 0 {
                    (&self.value).graphemes(true).rev().take((amount * -1) as usize).collect()
                } else if (amount as usize) < self.value.len() {
                    self.value[0..(self.value.len() - amount as usize)].to_string()
                } else {
                    String::new()
                };

                StringExpr::new(result)
            },
        });

        match value {
            Some(value) => return value,
            None => return StringExpr::new("".to_string())
        }
    }
    
    fn multiply(&self, other: &Expression) -> Expression {
        let count = NumberExpr::coerce_to_number(other) as i64;
        
        return StringExpr::new(if count < 0 {
            let seed: String = (&self.value).graphemes(true).rev().collect();
            let count = count.checked_neg().unwrap_or(std::i64::MAX);
            Self::multiply_string(&seed, count as usize)
        } else {
            Self::multiply_string(&self.value, count as usize)
        })
    }

    fn divide(&self, other: &Expression) -> Expression {
        let other = other.as_any();

        let value = match_cast!( other {
            val as StringExpr => {
                let count = self.value.matches(&val.value).count();
                NumberExpr::new(count as f64)
            },
            _val as NumberExpr => {
                NumberExpr::new(NumberExpr::coerce_to_number_from_any(self.as_any()) / NumberExpr::coerce_to_number_from_any(other))
            },
        });

        match value {
            Some(value) => value,
            None => NumberExpr::new(0.0)
        }
    }
}
