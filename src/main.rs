use std::fmt;

struct Polynomial {
    terms: Vec<Term>
}

impl Polynomial {
    fn new(terms: Vec<Term>) -> Polynomial {
        Polynomial{terms: terms}
    }
    fn derivative(&self) -> Polynomial {
        let mut new_terms = Vec::new();
        for term in &self.terms {
            new_terms.push(term.derivative());
        }
        Polynomial{terms: new_terms}
    }
    fn integral(&self) -> Polynomial {
        let mut new_terms = Vec::new();
        for term in &self.terms {
            new_terms.push(term.integral());
        }
        Polynomial{terms: new_terms}
    }
}


impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut repr = String::new();
        for term in &self.terms { 
            repr.push_str(&term.to_string());
        }
        write!(f, "{}", repr)
    }
}


struct Term {
    coef: f64,
    var: char,
    exp: f64
}

impl Term {
    fn new(coef: f64, var: char, exp: f64) -> Term {
        Term {coef: coef, var: var, exp: exp}
    }
    fn derivative(&self) -> Term {
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
    fn integral(&self) -> Term {
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
            let expr_str = String::new();
            repr.push_str(&self.coef.to_string());
            repr.push(self.var);
            repr.push('^');
            repr.push_str(&self.exp.to_string());
        }
        
        write!(f, "{}", repr)
    }
}

fn test_terms() {
    let t1 = Term::new(3.0, 'x', 2.0);
    let t2 = t1.derivative();
    assert_eq!(t2.coef, 6.0);
    assert_eq!(t2.exp, 1.0);

    let t3 = t2.derivative();
    assert_eq!(t3.coef, 6.0);
    assert_eq!(t3.exp, 0.0);

    let t4 = t3.derivative();
    assert_eq!(t4.coef, 0.0);
    assert_eq!(t4.exp, 0.0);

    for i in 0..5 {
        let mut t5 = t1.clone();
        for _ in 0..i {
            t5 = t5.integral();
        }
        println!("{} - {}", i, t5);
    }

}

fn test_polynomials() {
    // 3x^2 + 2x + 5
    let p1 = Polynomial::new(vec![
        Term::new(3.0, 'x', 2.0),
        Term::new(2.0, 'x', 1.0),
        Term::new(5.0, 'x', 0.0),
    ]);
    println!("{}", p1);
}

fn main() {
    test_terms();
    test_polynomials();
}
