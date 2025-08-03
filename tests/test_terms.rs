use integration_calculator::Term;

#[test]
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
