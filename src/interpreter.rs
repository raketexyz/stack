use std::collections::{HashMap, VecDeque};

use crate::{Builtin, Expression, Program, Statement, Value};

type Result<A> = std::result::Result<A, String>;

pub struct Interpreter {
    pub stack: Vec<Value>,
    pub statements: VecDeque<Statement>,
    pub definitions: HashMap<String, Value>,
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

    fn statement(&mut self, statement: Statement) -> Result<()> {
        match statement {
            Statement::Expression(e) => self.push(self.evaluate_expression(e)?),
            Statement::Builtin(b) => self.evaluate_builtin(b),
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
            Builtin::Eval => self.eval(),
            Builtin::Println => self.println(),
            Builtin::If => self.evaluate_if(),
            Builtin::Def => self.def(),
        }
    }

    fn def(&mut self) -> Result<()> {
        self.expect_args(2, "def")?;

        let (value, name) = (self.pop()?, self.pop()?);

        if let Value::Identifier(s) = name {
            self.definitions.insert(s, value);
            Ok(())
        } else {
            Err(format!("Expected string for definition, got {name}"))
        }
    }

    fn evaluate_if(&mut self) -> Result<()> {
        self.expect_args(3, "?")?;

        let (esle, then, cond) = (self.pop()?, self.pop()?, self.pop()?);
        let t = if cond == Value::Bool(true) { then } else { esle };

        match t {
            Value::Procedure(s) => {
                self.prepend_statements(s.0);
                Ok(())
            },
            _ => Err(format!("Can't evaluate {t}"))
        }
    }

    fn prepend_statements(&mut self, statements: Box<[Statement]>) {
        self.statements = VecDeque::from_iter(
            statements.to_vec().into_iter().chain(self.statements.iter().cloned())
        );
    }

    fn eq(&mut self) -> Result<()> {
        self.expect_args(2, "=")?;

        let (b, a) = (self.pop()?, self.pop()?);
        let s = self.resolve(a)? == self.resolve(b)?;

        self.push(Value::Bool(s))
    }

    fn neg(&mut self) -> Result<()> {
        self.expect_args(1, "!")?;

        let a = self.pop()?;

        self.push((!self.resolve(a)?)?)
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

    fn swap(&mut self) -> Result<()> {
        self.expect_args(2, "swap")?;

        let n = self.stack.len();

        self.stack.swap(n - 1, n - 2);

        Ok(())
    }

    /// Pops a value, dereferences it, takes it as a procedure and prepends the
    /// contained statements to the statement buffer.
    fn eval(&mut self) -> Result<()> {
        self.expect_args(1, "eval")?;

        let a = self.pop()?;

        let procedure = match self.resolve(a)? {
            Value::Procedure(s) => Ok(s),
            v => Err(format!("Can't evaluate {v}"))
        }?;

        self.prepend_statements(procedure.0);

        Ok(())
    }

    fn dup(&mut self) -> Result<()> {
        self.expect_args(1, "dup")?;

        let s = self.stack.last().unwrap();

        self.push(s.clone())
    }

    fn drop(&mut self) -> Result<()> {
        self.expect_args(1, "drop")?;

        self.pop()?;

        Ok(())
    }

    fn add(&mut self) -> Result<()> {
        self.expect_args(2, "+")?;

        let (b, a) = (self.pop()?, self.pop()?);
        let s = (self.resolve(a)? + self.resolve(b)?)?;

        self.push(s)
    }

    fn sub(&mut self) -> Result<()> {
        self.expect_args(2, "-")?;

        let (b, a) = (self.pop()?, self.pop()?);
        let s = (self.resolve(a)? - self.resolve(b)?)?;

        self.push(s)
    }

    fn mul(&mut self) -> Result<()> {
        self.expect_args(2, "*")?;

        let (b, a) = (self.pop()?, self.pop()?);
        let s = (self.resolve(a)? * self.resolve(b)?)?;

        self.push(s)
    }

    fn div(&mut self) -> Result<()> {
        self.expect_args(2, "/")?;

        let (b, a) = (self.pop()?, self.pop()?);

        self.push((self.resolve(a)? / self.resolve(b)?)?)
    }

    fn resolve(&self, value: Value) -> Result<Value> {
        match value {
            Value::Identifier(s) => match self.definitions.get(&s) {
                Some(v) => self.resolve(v.clone()),
                None => Err(format!("Couldn't resolve identifier {s:?}"))
            }
            v => Ok(v)
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

    fn println(&mut self) -> Result<()> {
        self.expect_args(1, "println")?;

        match self.pop()? {
            Value::String(s) => println!("{s}"),
            v => println!("{v}")
        }

        Ok(())
    }

    fn expect_args(&self, args: usize, name: &str) -> Result<()> {
        match self.stack.len() {
            n if n < args => Err(format!("Operation `{name}` expected {args} \
                                          argument(s), got {n}")),
            _ => Ok(())
        }
    }
}
