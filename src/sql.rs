//! Module that implements a SQL compiler

pub enum Statement {
    Select,
    Insert,
}

impl Statement {
    pub fn prepare(input: &str) -> anyhow::Result<Self> {
        let mut words = input.split_whitespace().peekable();

        match words.peek() {
            Some(word) => match *word {
                "select" => Ok(Statement::Select),
                "insert" => Ok(Statement::Insert),
                _ => anyhow::bail!("Unrecognized keyword at start of '{}'", input),
            },
            None => anyhow::bail!("Empty input"),
        }
    }

    pub fn execute(self) -> anyhow::Result<()> {
        match self {
            Statement::Select => println!("executing select"),
            Statement::Insert => println!("executing insert"),
        }
        Ok(())
    }
}
