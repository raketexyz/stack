use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Program {
    pub statements: Box<[Statement]>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Statement {
    Expression(Expression),
    Builtin(Builtin),
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Expression(s) => write!(f, "{s}"),
            Self::Builtin(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Expression {
    Literal(Literal),
    Procedure(Procedure),
    Identifier(String),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(l) => write!(f, "{l}"),
            Self::Procedure(s) => write!(f, "{s}"),
            Self::Identifier(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Procedure(pub Box<[Statement]>);

impl Display for Procedure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;

        if !self.0.is_empty() {
            for s in self.0.iter() {
                write!(f, " {s}")?;
            }

            write!(f, " ")?;
        }

        write!(f, "}}")
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Literal {
    Bool(bool),
    Number(f32),
    String(String),
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool(b) => write!(f, "{b}"),
            Self::Number(a) => write!(f, "{a}"),
            Self::String(s) => write!(f, "{s:?}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Builtin {
    /// Add the top two elements on the stack.
    Add,
    /// Subtract the top element from the second element on the stack.
    Sub,
    /// Multiply the top two elements on the stack.
    Mul,
    /// Divide the second element by the top element on the stack.
    Div,
    /// Check if the top two elements are equal.
    Eq,
    /// Pops the top element on the stack and pushes its inverse.
    Neg,
    /// `a b -- a < b`
    Lt,
    /// `a b -- a <= b`
    Le,
    /// `a b -- a > b`
    Gt,
    /// `a b -- a >= b`
    Ge,
    /// Duplicate the top element on the stack.
    Dup,
    /// Swap the top two elements on the stack.
    Swap,
    /// Drop (or "pop") the top element on the stack.
    Drop,
    /// Evaluate the top element on the stack.
    Eval,
    /// Print the top element on the stack and append a newline.
    Println,
    /// Evaluate the second to top item on the stack if the third is true, else
    /// the top item.
    If,
    /// Define the second to top item (an identifier) as the top item.
    Def,
}

impl Builtin {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Add => "+",
            Self::Sub => "-",
            Self::Mul => "*",
            Self::Div => "/",
            Self::Eq => "=",
            Self::Neg => "!",
            Self::Lt => "<",
            Self::Le => "<=",
            Self::Gt => ">",
            Self::Ge => ">=",
            Self::Dup => "dup",
            Self::Swap => "swap",
            Self::Drop => "drop",
            Self::Eval => "eval",
            Self::Println => "println",
            Self::If => "?",
            Self::Def => "def",
        }
    }
}

impl Display for Builtin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}