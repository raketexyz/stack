use std::collections::{HashMap, VecDeque};

use crate::{Builtin, Expression, Procedure, Program, Statement, Value};

type Result<A> = std::result::Result<A, String>;

pub struct Interpreter {
    pub stack: Vec<Value>,
    pub statements: VecDeque<Statement>,
    pub definitions: HashMap<String, Procedure>,
    pub verbose: bool,
}

impl Interpreter {
    #[allow(clippy::new_without_default)]
    pub fn new(verbose: bool) -> Self {
        Self {
            stack: vec![],
            statements: VecDeque::new(),
            definitions: HashMap::new(),
            verbose,
        }
    }

    pub fn run_program(&mut self, program: Program)
    -> Result<Option<Value>> {
        self.statements.append(&mut program.statements.to_vec().into());

        self.run_statements()?;

        Ok(self.stack.last().cloned())
    }

    fn run_statements(&mut self) -> Result<()> {
        while let Some(statement) = self.statements.pop_front() {
            if self.verbose {
                println!("DEBUG: Executing {statement}")
            }
            self.statement(statement)?;
            if self.verbose {
                println!(
                    "DEBUG: Stack: {}",
                    self.stack.iter().map(ToString::to_string)
                        .collect::<Vec<_>>().join(" ")
                )
            }
        }

        Ok(())
    }

    fn def(&mut self, identifier: String, procedure: Procedure) -> Result<()> {
        self.definitions.insert(identifier, procedure);

        Ok(())
    }

    fn word(&mut self, word: &str) -> Result<()> {
        let p = self.resolve(word)?.clone();

        self.prepend_statements(&p.0);

        Ok(())
    }

    fn statement(&mut self, statement: Statement) -> Result<()> {
        match statement {
            Statement::Expression(e) => self.push(self.evaluate_expression(e)?),
            Statement::Builtin(b) => self.evaluate_builtin(b),
            Statement::Value(v) => self.push(v),
            Statement::Definition { identifier, procedure } =>
                self.def(identifier, procedure),
            Statement::Word(w) => self.word(&w),
        }
    }

    fn resolve(&self, identifier: &str) -> Result<&Procedure> {
        match self.definitions.get(identifier) {
            Some(p) => Ok(p),
            None => Err(format!("Couldn't resolve identifier {identifier:?}"))
        }
    }

    fn evaluate_expression(&self, expression: Expression)
    -> Result<Value> {
        Ok(expression.into())
    }

    fn evaluate_builtin(&mut self, builtin: Builtin) -> Result<()> {
        match builtin {
            Builtin::Add => self.add(),
            Builtin::Sub => self.sub(),
            Builtin::Mul => self.mul(),
            Builtin::Div => self.div(),
            Builtin::Eq => self.eq(),
            Builtin::Neg => self.neg(),
            Builtin::Lt => self.lt(),
            Builtin::Le => self.le(),
            Builtin::Gt => self.gt(),
            Builtin::Ge => self.ge(),
            Builtin::Dup => self.dup(),
            Builtin::Swap => self.swap(),
            Builtin::Drop => self.drop(),
            Builtin::Over => self.over(),
            Builtin::Dupd => self.dupd(),
            Builtin::Keep => self.keep(),
            Builtin::Eval => self.eval(),
            Builtin::Println => self.println(),
            Builtin::If => self.evaluate_if(),
            Builtin::Nth => self.nth(),
        }
    }

    fn prepend_statements(&mut self, statements: &[Statement]) {
        self.statements = VecDeque::from_iter(
            statements.iter().cloned().chain(self.statements.iter().cloned())
        );
    }

    fn add(&mut self) -> Result<()> {
        self.expect_args(2, "+")?;

        let (b, a) = (self.pop()?, self.pop()?);
        let s = (a + b)?;

        self.push(s)
    }

    fn sub(&mut self) -> Result<()> {
        self.expect_args(2, "-")?;

        let (b, a) = (self.pop()?, self.pop()?);
        let s = (a - b)?;

        self.push(s)
    }

    fn mul(&mut self) -> Result<()> {
        self.expect_args(2, "*")?;

        let (b, a) = (self.pop()?, self.pop()?);
        let s = (a * b)?;

        self.push(s)
    }

    fn div(&mut self) -> Result<()> {
        self.expect_args(2, "/")?;

        let (b, a) = (self.pop()?, self.pop()?);

        self.push((a / b)?)
    }

    fn eq(&mut self) -> Result<()> {
        self.expect_args(2, "=")?;

        let (b, a) = (self.pop()?, self.pop()?);
        let s = a == b;

        self.push(Value::Bool(s))
    }

    fn neg(&mut self) -> Result<()> {
        self.expect_args(1, "!")?;

        let a = self.pop()?;

        self.push((!a)?)
    }

    fn lt(&mut self) -> Result<()> {
        self.expect_args(2, "<")?;

        let (b, a) = (self.pop()?, self.pop()?);

        self.push(Value::Bool(a < b))
    }

    fn le(&mut self) -> Result<()> {
        self.expect_args(2, "<=")?;

        let (b, a) = (self.pop()?, self.pop()?);

        self.push(Value::Bool(a <= b))
    }

    fn gt(&mut self) -> Result<()> {
        self.expect_args(2, ">")?;

        let (b, a) = (self.pop()?, self.pop()?);

        self.push(Value::Bool(a > b))
    }

    fn ge(&mut self) -> Result<()> {
        self.expect_args(2, ">=")?;

        let (b, a) = (self.pop()?, self.pop()?);

        self.push(Value::Bool(a >= b))
    }

    fn dup(&mut self) -> Result<()> {
        self.expect_args(1, "dup")?;

        let s = self.stack.last().unwrap();

        self.push(s.clone())
    }

    fn swap(&mut self) -> Result<()> {
        self.expect_args(2, "swap")?;

        let n = self.stack.len();

        self.stack.swap(n - 1, n - 2);

        Ok(())
    }

    fn drop(&mut self) -> Result<()> {
        self.expect_args(1, "drop")?;

        self.pop()?;

        Ok(())
    }

    fn over(&mut self) -> Result<()> {
        self.expect_args(2, "over")?;

        self.push(self.stack.iter().rev().nth(1).unwrap().clone())
    }

    fn dupd(&mut self) -> Result<()> {
        self.expect_args(2, "dupd")?;

        self.stack.insert(
            self.stack.len() - 1,
            self.stack.iter().rev().nth(1).unwrap().clone()
        );

        Ok(())
    }

    fn keep(&mut self) -> Result<()> {
        self.expect_args(2, "keep")?;

        let (b, a) = (self.pop()?, self.stack.last().unwrap().clone());

        match b {
            Value::Procedure(Procedure(s)) => {
                self.statements.push_front(Statement::Value(a));
                self.prepend_statements(&s);
                Ok(())
            }
            _ => Err(format!("Can't evaluate {b}"))
        }
    }

    /// Pops a value, dereferences it, takes it as a procedure and prepends the
    /// contained statements to the statement buffer.
    fn eval(&mut self) -> Result<()> {
        self.expect_args(1, "eval")?;

        let procedure = match self.pop()? {
            Value::Procedure(s) => Ok(s),
            v => Err(format!("Can't evaluate {v}"))
        }?;

        self.prepend_statements(&procedure.0);

        Ok(())
    }

    fn println(&mut self) -> Result<()> {
        self.expect_args(1, "println")?;

        match self.pop()? {
            Value::String(s) => println!("{s}"),
            v => println!("{v}")
        }

        Ok(())
    }

    fn evaluate_if(&mut self) -> Result<()> {
        self.expect_args(3, "?")?;

        let (esle, then, cond) = (self.pop()?, self.pop()?, self.pop()?);
        let t = if cond == Value::Bool(true) { then } else { esle };

        match t {
            Value::Procedure(s) => {
                self.prepend_statements(&s.0);
                Ok(())
            },
            _ => Err(format!("Can't evaluate {t}"))
        }
    }

    fn nth(&mut self) -> Result<()> {
        self.expect_args(2, "nth")?;

        let (b, a) = (self.pop()?, self.pop()?);

        match (a, b) {
            (Value::Number(n), Value::List(s)) if n.fract() == 0.0 => {
                match s.get(n as usize) {
                    Some(v) => self.push(v.clone()),
                    None => Err(format!("Index {n} out of bounds"))
                }
            }
            (a, b) => Err(format!("Can't index {b} by {a}"))
        }
    }

    fn push(&mut self, value: Value) -> Result<()> {
        self.stack.push(value);

        Ok(())
    }

    fn pop(&mut self) -> Result<Value> {
        self.expect_args(1, "pop")?;
        Ok(self.stack.pop().unwrap())
    }

    fn expect_args(&self, args: usize, name: &str) -> Result<()> {
        match self.stack.len() {
            n if n < args => Err(format!("Operation `{name}` expected {args} \
                                          argument(s), got {n}")),
            _ => Ok(())
        }
    }
}
