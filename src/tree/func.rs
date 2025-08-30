use crate::tree::expr::Expression;

// y = f(a1*x1, a2*x2, ..., an*xn)
pub struct Function {
    parameters: Vec<String>,
    coefficients: Vec<f64>,
    arguments: Vec<f64>,
    expr: fn(&Vec<f64>) -> f64
}

impl Expression for Function {
    type Args = Vec<f64>;
    type Solution = f64;

    fn simplify(&self) -> Self {
        todo!()
    }

    fn solve(&self, args: Self::Args) -> Self::Solution {
        (self.expr)(&args)
    }
}
