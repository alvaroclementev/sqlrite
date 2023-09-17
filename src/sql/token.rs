#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token<'a> {
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

impl<'a> Token<'a> {
    pub fn into_str(self) -> &'a str {
        // FIXME(alvaro): This is a temporary API, we want something more general
        // like a `Value` that can be unwrapped
        match self {
            Token::String(value) => value,
            Token::Identifier(value) => value,
            Token::Number(value) => value,
            token => panic!("the token {:?} cannot be turned into a string", token),
        }
    }
}
