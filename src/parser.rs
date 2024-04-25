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
        list.map(Expression::List),
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

pub fn list(input: &str) -> IResult<&str, Box<[Expression]>> {
    let items = delimited(
        multispace0,
        separated_list0(multispace1, expression),
        multispace0
    );

    context("List", delimited(char('['), Parser::into(items), char(']')))(input)
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
        value(Builtin::Swap, tag("swap")),
        value(Builtin::Drop, tag("drop")),
        value(Builtin::Over, tag("over")),
        value(Builtin::Eval, tag("eval")),
        value(Builtin::Dupd, tag("dupd")),
        value(Builtin::Dup, tag("dup")),
        value(Builtin::Keep, tag("keep")),
        value(Builtin::Println, tag("println")),
        value(Builtin::Def, tag("def")),
        value(Builtin::If, tag("?")),
        value(Builtin::Nth, tag("nth")),
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
    context("String", delimited(char('"'), is_not("\\\""), cut(char('"'))))(input)
}

#[cfg(test)]
mod tests {
    use crate::{builtin, expression, parser::statements, Builtin, Expression, Literal, Statement};

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

    #[test]
    fn builtins() {
        assert_eq!(builtin("+"), Ok(("", Builtin::Add)));
        assert_eq!(builtin("-"), Ok(("", Builtin::Sub)));
        assert_eq!(builtin("*"), Ok(("", Builtin::Mul)));
        assert_eq!(builtin("/"), Ok(("", Builtin::Div)));
        assert_eq!(builtin("="), Ok(("", Builtin::Eq)));
        assert_eq!(builtin("!"), Ok(("", Builtin::Neg)));
        assert_eq!(builtin("<"), Ok(("", Builtin::Lt)));
        assert_eq!(builtin("<="), Ok(("", Builtin::Le)));
        assert_eq!(builtin(">"), Ok(("", Builtin::Gt)));
        assert_eq!(builtin(">="), Ok(("", Builtin::Ge)));
        assert_eq!(builtin("dup"), Ok(("", Builtin::Dup)));
        assert_eq!(builtin("swap"), Ok(("", Builtin::Swap)));
        assert_eq!(builtin("drop"), Ok(("", Builtin::Drop)));
        assert_eq!(builtin("over"), Ok(("", Builtin::Over)));
        assert_eq!(builtin("dupd"), Ok(("", Builtin::Dupd)));
        assert_eq!(builtin("keep"), Ok(("", Builtin::Keep)));
        assert_eq!(builtin("eval"), Ok(("", Builtin::Eval)));
        assert_eq!(builtin("println"), Ok(("", Builtin::Println)));
        assert_eq!(builtin("?"), Ok(("", Builtin::If)));
        assert_eq!(builtin("def"), Ok(("", Builtin::Def)));
        assert_eq!(builtin("nth"), Ok(("", Builtin::Nth)));
    }

    #[test]
    fn list() {
        assert_eq!(expression("[1 2 3]"), Ok(("", Expression::List([
            Expression::Literal(Literal::Number(1.0)),
            Expression::Literal(Literal::Number(2.0)),
            Expression::Literal(Literal::Number(3.0)),
        ].into()))));
        assert_eq!(expression("[1 2 3 ]"), Ok(("", Expression::List([
            Expression::Literal(Literal::Number(1.0)),
            Expression::Literal(Literal::Number(2.0)),
            Expression::Literal(Literal::Number(3.0)),
        ].into()))));
        assert_eq!(expression("[ 1 2 3]"), Ok(("", Expression::List([
            Expression::Literal(Literal::Number(1.0)),
            Expression::Literal(Literal::Number(2.0)),
            Expression::Literal(Literal::Number(3.0)),
        ].into()))));
        assert_eq!(expression("[ 1 2 3 ]"), Ok(("", Expression::List([
            Expression::Literal(Literal::Number(1.0)),
            Expression::Literal(Literal::Number(2.0)),
            Expression::Literal(Literal::Number(3.0)),
        ].into()))));
    }
}
