use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{
        alpha1, alphanumeric0, char, multispace0, multispace1, space0, space1
    },
    combinator::{all_consuming, cut, opt, recognize, value},
    error::{context, VerboseError},
    multi::{separated_list0, separated_list1},
    number::complete::float,
    sequence::{delimited, pair, preceded, terminated, tuple},
    Parser
};

use crate::{Builtin, Expression, Literal, Procedure, Program, Statement};

type IResult<I, O> = nom::IResult<I, O, VerboseError<I>>;

fn eol_comment(input: &str) -> IResult<&str, &str> {
    preceded(char('#'), is_not("\n\r"))(input)
}

fn statements(input: &str) -> IResult<&str, Box<[Statement]>> {
    let line = terminated(
        separated_list0(space1, statement),
        pair(space0, opt(eol_comment))
    );

    preceded(multispace0, separated_list1(multispace1, line))
        .map(|s| s.into_iter().flatten().collect())
        .parse(input)
}

pub fn program(input: &str) -> IResult<&str, Program> {
    context(
        "Program",
        all_consuming(statements).map(|statements| Program { statements })
    )(input)
}

pub fn identifier(input: &str) -> IResult<&str, &str> {
    context("Identifier", recognize(tuple((
        alt((alpha1, tag("_"))),
        alphanumeric0,
        opt(char('?')),
    ))))(input)
}

pub fn expression(input: &str) -> IResult<&str, Expression> {
    context("Expression", alt((
        literal.map(Expression::Literal),
        procedure.map(Expression::Procedure),
        Parser::into(identifier).map(Expression::Identifier),
    )))(input)
}

pub fn procedure(input: &str) -> IResult<&str, Procedure> {
    context("Procedure", delimited(
        char('{'),
        cut(Parser::into(statements).map(Procedure)),
        cut(char('}'))
    ))(input)
}

pub fn statement(input: &str) -> IResult<&str, Statement> {
    context("Statement", alt((
        builtin.map(Statement::Builtin),
        expression.map(Statement::Expression),
    )))(input)
}

pub fn builtin(input: &str) -> IResult<&str, Builtin> {
    context("Builtin", alt((
        value(Builtin::Add, tag("+")),
        value(Builtin::Sub, tag("-")),
        value(Builtin::Mul, tag("*")),
        value(Builtin::Div, tag("/")),
        value(Builtin::Eq, tag("=")),
        value(Builtin::Neg, tag("!")),
        value(Builtin::Le, tag("<=")),
        value(Builtin::Lt, tag("<")),
        value(Builtin::Ge, tag(">=")),
        value(Builtin::Gt, tag(">")),
        value(Builtin::Dup, tag("dup")),
        value(Builtin::Swap, tag("swap")),
        value(Builtin::Drop, tag("drop")),
        value(Builtin::Eval, tag("eval")),
        value(Builtin::Println, tag("println")),
        value(Builtin::Def, tag("def")),
        value(Builtin::If, tag("?")),
    )))(input)
}

pub fn literal(input: &str) -> IResult<&str, Literal> {
    context("Literal", alt((
        float.map(Literal::Number),
        Parser::into(string).map(Literal::String),
        bool.map(Literal::Bool),
    )))(input)
}

pub fn bool(input: &str) -> IResult<&str, bool> {
    context("bool", alt((
        value(false, tag("false")),
        value(true, tag("true")),
    )))(input)
}

pub fn string(input: &str) -> IResult<&str, &str> {
    context("String", delimited(char('"'), is_not("\\\""), char('"')))(input)
}

#[cfg(test)]
mod tests {
    use crate::{parser::statements, Expression, Literal, Statement};

    #[test]
    fn statements_empty() {
        assert_eq!(statements(""), Ok(("", [].into())));
        assert_eq!(statements("\n"), Ok(("", [].into())));
    }

    #[test]
    fn statements_one() {
        assert_eq!(
            statements("1"),
            Ok(("", [Statement::Expression(Expression::Literal(
                Literal::Number(1.0)
            ))].into()))
        );
    }

    #[test]
    fn comment() {
        assert_eq!(statements("# hello\n"), Ok(("", [].into())))
    }
}
