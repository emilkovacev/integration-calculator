use integration_calculator::{Polynomial, Term};

fn check_derivative(poly: &Polynomial, derivative: &Polynomial) {
    assert_eq!(derivative.terms.len(), poly.terms.len());
    for i in 0..poly.terms.len() {
        assert_eq!(derivative.terms[i].coef, poly.terms[i].coef * poly.terms[i].exp, "Coefficient was not multiplied correctly");
        if poly.terms[i].exp > 0.0 {
            assert_eq!(derivative.terms[i].exp, poly.terms[i].exp - 1.0, "Exponent was not incremented correctly");
        }
        assert_eq!(derivative.terms[i].var, poly.terms[i].var, "Variable was mutated");
    }
}

#[test]
fn test_polynomials() {
    // 3x^2 + 2x + 5
    let p1 = Polynomial::new(vec![
        Term::new(3.0, 'x', 2.0),
        Term::new(2.0, 'x', 1.0),
        Term::new(5.0, 'x', 0.0),
    ]);
    let p2 = p1.derivative();
    check_derivative(&p1, &p2);
    let p3 = p2.derivative();
    check_derivative(&p2, &p3);
}
