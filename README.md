# Integration Calculator

## Installation

```
cargo add --git https://github.com/emilkovacev/integration-calculator.git
```

## Usage

Integrating a polynomial:

```
use integration_calculator::{Polynomial, Term};

fn main() {
    let p1 = Polynomial::new(vec![
        Term::new(3.0, 'x', 2.0),
        Term::new(2.0, 'x', 1.0),
        Term::new(5.0, 'x', 0.0),
    ]);
    let p2 = p1.integral();
    println!("p1: {}", p1);
    println!("p2: {}", p2);
}
```

Output:

```
p1: +3x^2+2x+5
p2: +1x^3+1x^2+5x
```
