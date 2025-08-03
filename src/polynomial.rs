use super::term::Term;
use std::fmt;

pub struct Polynomial {
    pub terms: Vec<Term>
}

impl Polynomial {
    pub fn new(terms: Vec<Term>) -> Polynomial {
        Polynomial{terms: terms}
    }
    pub fn derivative(&self) -> Polynomial {
        let mut new_terms = Vec::new();
        for term in &self.terms {
            new_terms.push(term.derivative());
        }
        Polynomial{terms: new_terms}
    }
    pub fn integral(&self) -> Polynomial {
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
