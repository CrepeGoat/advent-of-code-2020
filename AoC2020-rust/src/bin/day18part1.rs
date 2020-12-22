use std::error::Error;


enum MathValue<T: Copy> {
    Literal(T),
    Expr(MathExpr<T>),
}

struct MathExpr<T: Copy> {
    first: Box<MathValue<T>>,
    ops: Vec<(Box<dyn Fn(T, T) -> T>, MathValue<T>)>,
}


//struct MathParseError impl Error;


//impl FromStr for MathExpr {
//    type Err = MathParseError;
//
//    fn 
//}

impl<T: Copy> MathValue<T> {
    fn compute(&self) -> T {
        match self {
            Self::Literal(value) => *value,
            Self::Expr(expr) => expr.compute(),
        }
    }
}

impl<T: Copy> MathExpr<T> {
    fn compute(&self) -> T {
        self.ops.iter().fold(
            self.first.compute(),
            |val, (op, mathval)| op(val, mathval.compute())
        )
    }
}
