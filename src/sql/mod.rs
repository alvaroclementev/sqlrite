//! Module that implements a SQL compiler

#![allow(dead_code)]

mod lexer;
mod token;

use anyhow::Context;

use crate::sql::lexer::Lexer;
use crate::sql::token::Token;
use crate::storage::Row;

pub enum Statement {
    Select(SelectStatement),
    Insert(InsertStatement),
}

impl Statement {
    pub fn prepare(input: &str) -> anyhow::Result<Self> {
        // "Lexing"
        let mut lexer = Lexer::new(input);

        // Stupid simple parsing
        if let Some(token) = lexer.peek_token() {
            match token {
                Token::Select => Statement::parse_select(lexer),
                Token::Insert => Statement::parse_insert(lexer),
                token => anyhow::bail!("Unrecognized keyword: {:?}", token),
            }
        } else {
            anyhow::bail!("Empty input");
        }
    }

    pub fn execute(self) -> anyhow::Result<()> {
        match self {
            Statement::Select(..) => println!("executing select"),
            Statement::Insert(stmt) => {
                println!("Trying to insert {} rows: {:?}", stmt.rows.len(), stmt.rows);
            }
        }
        Ok(())
    }

    fn parse_insert(mut lexer: Lexer) -> anyhow::Result<Self> {
        let insert_token = lexer.next_token();
        assert!(matches!(insert_token, Some(Token::Insert)));

        // TODO(alvaro): Parse multiple insert rows
        // NOTE(alvaro): For now insert statements look like
        //
        // insert 1 "cstack" "foo@bar.com"

        let id = lexer
            .next_token()
            .context("expected a number when parsing INSERT")?;

        if !matches!(id, Token::Number(..)) {
            anyhow::bail!("expected a number when parsing INSERT, got {:?}", id);
        }

        let username = lexer
            .next_token()
            .context("expected a string when parsing INSERT")?;

        if !matches!(username, Token::String(..)) {
            anyhow::bail!("expected a string when parsing INSERT, got {:?}", username);
        }

        let email = lexer
            .next_token()
            .context("expected a string when parsing INSERT")?;

        if !matches!(email, Token::String(..)) {
            anyhow::bail!("expected a string when parsing INSERT, got {:?}", email);
        }

        let rows = vec![Row::new(
            id.into_str().parse().expect("a valid numeric id"),
            username.into_str().to_string(),
            email.into_str().to_string(),
        )];

        // Consume the semicolon
        let end = lexer
            .next_token()
            .context("expected a semicolon when parsing INSERT")?;
        if !matches!(end, Token::Semicolon) {
            anyhow::bail!("expected a semicolon when parsing INSERT, got {:?}", end);
        }

        Ok(Statement::Insert(InsertStatement::new(rows)))
    }

    fn parse_select(mut _lexer: Lexer) -> anyhow::Result<Self> {
        todo!()
    }
}

pub struct SelectStatement {
    columns: Vec<String>,
}

impl SelectStatement {
    pub fn new(columns: Vec<String>) -> Self {
        Self { columns }
    }
}

pub struct InsertStatement {
    pub rows: Vec<Row>,
}

impl InsertStatement {
    pub fn new(rows: Vec<Row>) -> Self {
        Self { rows }
    }
}
