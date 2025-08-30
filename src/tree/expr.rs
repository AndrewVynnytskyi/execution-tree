pub trait Expression {
    type Args;
    type Solution;

    fn simplify(&self) -> Self;
    fn solve(&self, args: Self::Args) -> Self::Solution;
}

// TODO

// impl std::ops::Add for Expression {
//
// }

// and so on
