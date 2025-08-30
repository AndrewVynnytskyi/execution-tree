mod tree;

use tree::expr::Expression;
use tree::ints::Integer64;


fn main() {
    let expr = Integer64::new(10);

    // y = 1 + x1 + 2*x2
    // let y = Integer64::new(1) + LinearFunction::new(vec!["x1", "x2"], vec![1, 2]))
    // let res = y.evaluate();

    println!("{}", expr.simplify());
}
