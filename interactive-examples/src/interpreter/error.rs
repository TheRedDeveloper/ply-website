use std::fmt;

/// All errors the interpreter can produce.
#[derive(Debug, Clone)]
pub enum InterpreterError {
    /// Lexer error with source position.
    Lex {
        message: String,
        line: usize,
        col: usize,
    },
    /// Parser error with source position.
    Parse {
        message: String,
        line: usize,
        col: usize,
    },
    /// The user tried to use a Rust feature that isn't supported in the demo.
    Unsupported { message: String },
    /// Runtime evaluation error.
    Eval { message: String },
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lex { message, line, col } => {
                write!(f, "Line {line}, col {col}: {message}")
            }
            Self::Parse { message, line, col } => {
                write!(f, "Line {line}, col {col}: {message}")
            }
            Self::Unsupported { message } => f.write_str(message),
            Self::Eval { message } => f.write_str(message),
        }
    }
}

impl InterpreterError {
    pub fn lex(message: impl Into<String>, line: usize, col: usize) -> Self {
        Self::Lex {
            message: message.into(),
            line,
            col,
        }
    }

    pub fn parse(message: impl Into<String>, line: usize, col: usize) -> Self {
        Self::Parse {
            message: message.into(),
            line,
            col,
        }
    }

    pub fn unsupported(message: impl Into<String>) -> Self {
        Self::Unsupported {
            message: message.into(),
        }
    }

    pub fn eval(message: impl Into<String>) -> Self {
        Self::Eval {
            message: message.into(),
        }
    }
}
