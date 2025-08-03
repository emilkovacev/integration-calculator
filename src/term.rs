use std::fmt;


pub struct Term {
    pub coef: f64,
    pub var: char,
    pub exp: f64
}

impl Term {
    pub fn new(coef: f64, var: char, exp: f64) -> Term {
        Term {coef: coef, var: var, exp: exp}
    }
    pub fn derivative(&self) -> Term {
        if self.exp == 0.0 {
            return Term {
                coef: 0.0,
                var: self.var,
                exp: 0.0
            }
        }
        Term {
            coef: self.coef * self.exp,
            var: self.var,
            exp: self.exp - 1.0
        }
    }
    pub fn integral(&self) -> Term {
        Term {
            coef: self.coef / (self.exp + 1.0),
            var: self.var,
            exp: self.exp + 1.0
        }
    }
}

impl Clone for Term {
    fn clone(&self) -> Term {
        Term{coef: self.coef, var: self.var, exp: self.exp}
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut repr = String::new();
        if self.coef >= 0.0 {
            repr.push('+');
        } else {
            repr.push('-');
        }
        if self.exp == 0.0 {
            repr.push_str(&self.coef.to_string());
        } 
        else if self.exp == 1.0 {
            repr.push_str(&self.coef.to_string());
            repr.push(self.var);
        } 
        else {
            repr.push_str(&self.coef.to_string());
            repr.push(self.var);
            repr.push('^');
            repr.push_str(&self.exp.to_string());
        }
        
        write!(f, "{}", repr)
    }
}
