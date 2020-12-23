use std::error::Error;



// Break down a string into intermediate symbols
#[derive(Debug)]
enum OperatorSymbol {
    Plus,
    Times,
}

#[derive(Debug)]
enum MathSymbol<T: Copy> {
    Literal(T),
    Operator(OperatorSymbol),
    Expr(Vec<MathSymbol<T>>),
}

#[derive(Debug)]
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
                MathStrToken::Char('+') => MathStrToken::Symbol(MathSymbol::Operator(OperatorSymbol::Plus),
                MathStrToken::Char('*') => MathStrToken::Symbol(MathSymbol::Operator(OperatorSymbol::Times),
                _ => _,
            })
            .collect();

        fn parse_flat_expr(tokens: &[MathStrToken]) -> Result<Self, Self::Err> {
            let digit_buffer = String::new();
            for token in tokens.iter() {
                match token {
                    MathStrToken::Char(c) if "0123456789".contains(c) => digit_buffer.push(c),
                    MathStrToken::Char(c) if "0123456789".contains(c) => return Err(MathParseError::InvalidChar),
                }
            }
        }

        while let Some(i2) = tokens.find(MathStrToken::Char(')')) {
            let i1 = tokens[..i2].rfind(MathStrToken::Char('('))
                .ok_or(MathParseError::BadParentheses)?;
            tokens[i1..=i2].replace(parse_flat_expr(tokens[i1+1..i2])?)
        }
        if let Some(i) = tokens.find(MathStrToken::Char('(')) {
            return Err(MathParseError::BadParentheses));
        }
        parse_flat_expr(tokens)
    }
}


// Build an AST from the intermediate symbols
enum CalcNode<T> {
    Literal(T),
    BinaryOperator(Box<dyn Fn(T, T) -> T>, (T, T))
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
