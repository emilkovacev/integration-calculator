use integration_calculator::{Polynomial, Term};

#[test]
fn test_polynomials() {
    // 3x^2 + 2x + 5
    let p1 = Polynomial::new(vec![
        Term::new(3.0, 'x', 2.0),
        Term::new(2.0, 'x', 1.0),
        Term::new(5.0, 'x', 0.0),
    ]);
    println!("{}", p1);
}
