use std::error::Error;


// Break down a string into intermediate symbols
enum MathExpr<T: Copy> {
    Literal(T),
    BinaryOperator(Box<(dyn Fn(T, T) -> T, MathExpr<T>, MathExpr<T>)>),
}

enum OperatorDescriptor {
    Plus,
    Times,
}


#[derive(Debug)]
enum MathParseError {
    InvalidChar,
    InvalidGrammar,
    MismatchedParentheses,
}


impl<T> FromStr for MathExpr<T> {
    type Err = MathParseError;

    fn parse(&self) -> Result<Self, Self::Err> {
        // `None`'s denote parenthesis boundaries
        let mut buffer_ops = Vec::<Option<OperatorDescriptor>>::new();
        let mut buffer_vals = Vec::<Option<T>>::new();

        fn sublen<U>(seq: &Vec<Option<U>>) -> usize {
            // Get length of items in current grouping
            seq.iter().rev().enumerate().find(|&&opt| opt.is_none()).0
        }

        fn load_val(value: T) {
            match (sublen(&buffer_vals), sublen(&buffer_ops)) {
                (0, 0) => buffer_vals.push(Some(value)),
                (1, 1) => ,
                _ => {return Err(Self::Err::InvalidGrammar);},

            }
        }

        for substr in self.split(" ") {
            match substr {
                "(" => {
                    // add parenthesis boundary to buffers
                    buffer_vals.push(None);
                    buffer_ops.push(None);
                },
                ")" => {
                    // operations buffer should have one divider in front
                    buffer_ops.pop()
                        .ok_or(Self::Err::MismatchedParentheses)?
                        .map_or(|_| Err(Self::Err::InvalidGrammar), Ok())?;
                    // value buffer should have one value in front
                    let tmp_value = buffer_vals.pop()
                        .ok_or(Self::Err::MismatchedParentheses)?  // buffer shouldn't be empty
                        .ok_or(Self::Err::InvalidGrammar)?;  // boundary should have exactly one value
                    // value buffer should now have one divider in front
                    buffer_vals.pop()
                        .ok_or(Self::Err::MismatchedParentheses)?
                        .map_or(|_| Err(Self::Err::InvalidGrammar), Ok())?;
                    // re-add value to buffer
                    load_value(tmp_value);
                },
                "+" | "*" => {
                    buffer_ops.push(Some(if substr == "+" {OperatorDescriptor::Plus} else {OperatorDescriptor::Times}));
                    // handle consecutive operators
                    if sublen(&buffer_ops) > sublen(&buffer_vals) {
                        return Err(Self::Err::InvalidGrammar);
                    }
                },
                _ => {
                    load_val(substr.parse().map_err(|_| Self::Err::InvalidChar)?);
                },
            }
        }
    }
}


impl<T: Copy> MathExpr<T> {
    fn compute(&self) -> T {
        match self {
            Self::Literal(value) => *value,
            Self::BinaryOperator(expr) => expr.0(expr.1, expr.2),
        }
    }
}
