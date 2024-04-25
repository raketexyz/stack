use std::{fmt::Display, ops::{Add, Div, Mul, Not, Sub}};

use crate::{Expression, Literal, Procedure};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    /// A boolean value.
    Bool(bool),
    /// A numerical value represented as an `f64`.
    Number(f64),
    /// A textual value represented as a `String`.
    String(String),
    /// A procedure.
    Procedure(Procedure),
    /// A list.
    List(Box<[Value]>),
}

impl Add for Value {
    type Output = Result<Self, String>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(a), Self::Number(b)) => Ok(Self::Number(a + b)),
            (a, b) => Err(format!("Can't add {a} and {b}"))
        }
    }
}

impl Sub for Value {
    type Output = Result<Self, String>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(a), Self::Number(b)) => Ok(Self::Number(a - b)),
            (a, b) => Err(format!("Can't subtract {b} from {a}"))
        }
    }
}

impl Mul for Value {
    type Output = Result<Self, String>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(a), Self::Number(b)) => Ok(Self::Number(a * b)),
            (a, b) => Err(format!("Can't multiply {a} and {b}"))
        }
    }
}

impl Div for Value {
    type Output = Result<Self, String>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(a), Self::Number(b)) => Ok(Self::Number(a / b)),
            (a, b) => Err(format!("Can't divide {a} by {b}"))
        }
    }
}

impl Not for Value {
    type Output = Result<Self, String>;

    fn not(self) -> Self::Output {
        match self {
            Self::Bool(b) => Ok(Self::Bool(!b)),
            v => Err(format!("Can't negate {v}"))
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool(b) => write!(f, "{b}"),
            Self::Number(a) => write!(f, "{a}"),
            Self::String(s) => write!(f, "{s:?}"),
            Self::Procedure(s) => write!(f, "{s}"),
            Self::List(s) => {
                write!(f, "[")?;

                for x in s.iter() {
                    write!(f, " {x}")?;
                }

                if !s.is_empty() {
                    write!(f, " ")?;
                }

                write!(f, "]")
            }
        }
    }
}

impl From<Literal> for Value {
    fn from(literal: Literal) -> Self {
        match literal {
            Literal::Bool(b) => Self::Bool(b),
            Literal::Number(a) => Self::Number(a as f64),
            Literal::String(s) => Self::String(s),
        }
    }
}

impl From<Expression> for Value {
    fn from(value: Expression) -> Self {
        match value {
            Expression::Literal(l) => l.into(),
            Expression::Procedure(p) => Self::Procedure(p),
            Expression::List(l) =>
                Self::List(l.iter().cloned().map(Into::into).collect()),
        }
    }
}
