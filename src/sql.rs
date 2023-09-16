//! Module that implements a SQL compiler

#![allow(dead_code)]

pub enum Statement {
    Select,
    Insert,
}

impl Statement {
    pub fn prepare(input: &str) -> anyhow::Result<Self> {
        // "Lexing"
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

#[derive(Clone, Debug, PartialEq, Eq)]
enum Token<'a> {
    // Error
    Error,
    // Symbols
    Comma,
    Semicolon,
    // Operators
    Equal,
    Plus,
    Minus,
    // Keywords
    Select,
    Insert,
    As,
    From,
    Where,
    // Literals
    Number(&'a str),
    String(&'a str),
    // Identifier
    Identifier(&'a str),
    // Eof
    Eof,
}

#[derive(Clone, Debug)]
struct Lexer<'a> {
    input: &'a str,
    current_token: Token<'a>,
    next_token: Token<'a>,
    cursor: usize,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        let mut lexer = Self {
            input,
            cursor: 0,
            current_token: Token::Error,
            next_token: Token::Error,
        };
        lexer.advance();
        lexer.advance();
        lexer
    }

    pub fn next_token(&mut self) -> Option<Token<'a>> {
        let next_token = self.advance();
        if let Token::Eof = next_token {
            None
        } else {
            Some(next_token)
        }
    }

    /// Advance the cursor lexing the next token. Returns the token that
    /// was at `current_token`
    fn advance(&mut self) -> Token<'a> {
        assert!(!self.at_eof());

        // Skip any whitespace there may be
        self.skip_whitespace();
        if self.at_eof() {
            return Token::Eof;
        }

        // Consume a new token, advance the current and next
        if let Some(next_char) = self.remaining().chars().next() {
            match next_char {
                'a'..='z' | 'A'..='Z' => {
                    // Lex an identifier and or keyword
                    self.consume_identifier()
                }
                '"' => {
                    // Lex a string literal with '"'
                    self.consume_string_literal('"')
                }
                '\'' => {
                    // Lex a string literal with '''
                    self.consume_string_literal('\'')
                }
                c if c.is_ascii_digit() => {
                    // Lex a number
                    self.consume_number_literal()
                }
                c => {
                    // Lex single or multiple char operators
                    self.consume_op_or_symbol(c)
                }
            }
        } else {
            self.push_token(Token::Eof)
        }
    }

    fn remaining(&self) -> &'a str {
        &self.input[self.cursor..]
    }

    fn at_eof(&self) -> bool {
        matches!(self.current_token, Token::Eof)
    }

    fn skip_whitespace(&mut self) {
        if let Some(offset) = self.remaining().chars().position(|c| !c.is_whitespace()) {
            self.cursor += offset;
        }
    }

    fn consume_op_or_symbol(&mut self, ch: char) -> Token<'a> {
        let token = match ch {
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '=' => Token::Equal,
            ch => panic!("unknown single character: '{}'", ch),
        };

        self.cursor += 1;
        self.push_token(token)
    }

    fn consume_identifier(&mut self) -> Token<'a> {
        // Consume an alphanumeric char
        let ident_length = self
            .remaining()
            .chars()
            .take_while(|c| is_identifier_char(*c))
            .count();

        let ident = &self.remaining()[..ident_length];

        // Check if the ident is a keyword
        let token = match ident {
            "select" => Token::Select,
            "insert" => Token::Insert,
            "as" => Token::As,
            "from" => Token::From,
            "where" => Token::Where,
            ident => Token::Identifier(ident),
        };

        // Advance cursor including the delimiters
        self.cursor += ident_length;
        self.push_token(token)
    }

    fn consume_string_literal(&mut self, delim: char) -> Token<'a> {
        // TODO(alvaro): Handle escaping
        // Consume a string up to the next delimiter
        let string_length = self.remaining()[1..]
            .chars()
            .take_while(|c| *c != delim)
            .count();

        let literal = &self.remaining()[1..string_length + 1];
        let token = Token::String(literal);

        // Advance cursor including the delimiters
        self.cursor += string_length + 2;
        self.push_token(token)
    }

    fn consume_number_literal(&mut self) -> Token<'a> {
        // TODO(alvaro): Support other types of number literals: floats, hex, binary, scientific
        let number_len = self
            .remaining()
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .count();

        // Generate the token
        let token = Token::Number(&self.remaining()[..number_len]);

        // Push the token and advance the cursor
        self.cursor += number_len;
        self.push_token(token)
    }

    /// Push a new token into the `next_token` position, shifting the `next` to
    /// the `current`. Returns the token that was at `current_token`
    fn push_token(&mut self, mut token: Token<'a>) -> Token<'a> {
        std::mem::swap(&mut self.current_token, &mut self.next_token);
        std::mem::swap(&mut self.next_token, &mut token);
        token
    }

    /// Temporary helper to collect all tokens from a lexer
    pub fn collect_tokens(&mut self) -> Vec<Token<'_>> {
        let mut tokens = Vec::new();

        while !self.at_eof() {
            let next_token = self.next_token().expect("some token to exist");
            tokens.push(next_token);
        }
        tokens
    }
}

fn is_identifier_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_simple() {
        let input = "1, 2, 1000, \"foo\", 'bar', ident;";

        let mut lexer = Lexer::new(input);
        let tokens = lexer.collect_tokens();

        assert_eq!(
            &tokens,
            &vec![
                Token::Number("1"),
                Token::Comma,
                Token::Number("2"),
                Token::Comma,
                Token::Number("1000"),
                Token::Comma,
                Token::String("foo"),
                Token::Comma,
                Token::String("bar"),
                Token::Comma,
                Token::Identifier("ident"),
                Token::Semicolon,
            ]
        );
    }

    #[test]
    fn test_lexer() {
        let input = "select foo, bar, baz, 1 + 2 as foo2, 1 - 2 as foo3 from my_table where 1 = 1;";

        let mut lexer = Lexer::new(input);
        let tokens = lexer.collect_tokens();

        assert_eq!(
            &tokens,
            &vec![
                Token::Select,
                Token::Identifier("foo"),
                Token::Comma,
                Token::Identifier("bar"),
                Token::Comma,
                Token::Identifier("baz"),
                Token::Comma,
                Token::Number("1"),
                Token::Plus,
                Token::Number("2"),
                Token::As,
                Token::Identifier("foo2"),
                Token::Comma,
                Token::Number("1"),
                Token::Minus,
                Token::Number("2"),
                Token::As,
                Token::Identifier("foo3"),
                Token::From,
                Token::Identifier("my_table"),
                Token::Where,
                Token::Number("1"),
                Token::Equal,
                Token::Number("1"),
                Token::Semicolon,
            ]
        );
    }
}
