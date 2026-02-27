/// Token produced by the lexer.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Int(i64),
    Float(f64),
    Str(String),

    // Identifiers
    Ident(String),

    // Punctuation
    Dot,
    Comma,
    Semicolon,
    Pipe,
    Bang,
    LParen,
    RParen,
    LBrace,
    RBrace,

    // End of file
    Eof,
}

/// A token with its position in the source code.
#[derive(Debug, Clone)]
pub struct Spanned {
    pub token: Token,
    pub line: usize,
    pub col: usize,
}
