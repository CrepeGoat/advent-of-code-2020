use nom::IResult;
use nom::branch::alt;
use nom::character::complete::{char, digit1};
use nom::sequence::tuple;


// Break down a string into intermediate symbols
enum MathExpr<T: Copy> {
    Literal(T),
    BinaryOperator(Box<(MathExpr<T>, MathExpr<T>, dyn Fn(T, T) -> T)>),
}

#[derive(Debug)]
enum MathParseError {
    InvalidChar,
    InvalidGrammar,
    MismatchedParentheses,
}


fn space(s: &str) -> IResult<&str, char> {
	char(' ')(s)
}

fn paren_open(s: &str) -> IResult<&str, char> {
	char('(')(s)
}
fn paren_close(s: &str) -> IResult<&str, char> {
	char(')')(s)
}

fn plus(s: &str) -> IResult<&str, char> {
	char('+')(s)
}

fn times(s: &str) -> IResult<&str, char> {
	char('*')(s)
}

fn value(s: &str) -> IResult<&str, MathExpr<u64>> {
	if let Ok(result) = digit1(s) {
		return Ok(result)
	}
	if let Ok(result) = tuple((paren_open, value, paren_close)) {
		return Ok(result)
	}
	alt((
		digit1,
		tuple((
			value,
			space,
			alt((plus, times)),
			space,
			value,
		)),
		tuple((
			paren_open,
			value,
			paren_close,
		)),
	))(s)
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse() {
		assert_eq!(Some(value("1 + (2 * 3) + (4 * (5 + 6))")), None);
	}
}
