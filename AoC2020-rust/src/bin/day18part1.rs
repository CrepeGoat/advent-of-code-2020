use std::error::Error;


enum MathSymbol<T: Copy> {
    Literal(T),
    Operator(Box<dyn Fn(T, T) -> T>),
    Expr(Vec<MathSymbol<T>>),
}


enum MathParseError {
    InvalidChar,
    BadParentheses,
}


impl<T> FromStr for MathSymbol<T> {
    type Err = MathParseError;

    #[derive(Debug)]
    enum MathStrToken {
        Char(char),
        Symbol(MathSymbol),
    }

    fn parse(&self) -> Result<Self, Self::Err> {
        let mut tokens = self.replace(" ", "").chars()
            .map(MathStrToken::Char)
            .map(|token| match token {
                MathStrToken::Char('+') => MathStrToken::Symbol(MathSymbol::Operator(|x, y| x+y)),
                MathStrToken::Char('*') => MathStrToken::Symbol(MathSymbol::Operator(|x, y| x*y)),
                _ => _,
            })
            .collect();

        fn parse_ungrouped(tokens: &[MathStrToken]) -> Result<Self, Self::Err> {

        }

        while let Some(i2) = tokens.find(MathStrToken::Char(')')) {
            let i1 = tokens[..i2].rfind(MathStrToken::Char('('))
                .ok_or(MathParseError::BadParentheses)?;
            tokens[i1..=i2].replace(parse_ungrouped(tokens[i1+1..i2])?)
        }
        if let Some(i) = tokens.find(MathStrToken::Char('(')) {
            return Err(MathParseError::BadParentheses));
        }
        parse_ungrouped(tokens)
    }
}

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
