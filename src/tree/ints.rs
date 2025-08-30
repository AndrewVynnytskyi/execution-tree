use std::fmt::Display;
use crate::tree::expr::Expression;

pub struct Integer64 {
    value: i64,
}

impl Integer64 {
    pub fn new(value: i64) -> Self { Integer64 { value } }
}

impl Expression for Integer64 {
    type Args = ();
    type Solution = i64;

    fn simplify(&self) -> Self { Integer64::new(self.value) }

    fn solve(&self, _args: Self::Args) -> Self::Solution { self.value }
}

impl Display for Integer64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
