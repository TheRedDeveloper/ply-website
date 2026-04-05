use crate::interpreter::error::InterpreterError;
use crate::interpreter::tokens::{Spanned, Token};

/// Lex source code into a vector of spanned tokens.
pub fn lex(source: &str) -> Result<Vec<Spanned>, InterpreterError> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = source.chars().collect();
    let mut pos = 0;
    let mut line = 1usize;
    let mut col = 1usize;

    while pos < chars.len() {
        let ch = chars[pos];

        // Skip whitespace
        if ch.is_ascii_whitespace() {
            if ch == '\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
            pos += 1;
            continue;
        }

        // Skip line comments
        if ch == '/' && pos + 1 < chars.len() && chars[pos + 1] == '/' {
            while pos < chars.len() && chars[pos] != '\n' {
                pos += 1;
            }
            continue;
        }

        let start_line = line;
        let start_col = col;

        // String literal
        if ch == '"' {
            pos += 1;
            col += 1;
            let mut s = String::new();
            loop {
                if pos >= chars.len() {
                    return Err(InterpreterError::lex(
                        "Unterminated string literal",
                        start_line,
                        start_col,
                    ));
                }
                let c = chars[pos];
                if c == '"' {
                    pos += 1;
                    col += 1;
                    break;
                }
                if c == '\\' {
                    pos += 1;
                    col += 1;
                    if pos >= chars.len() {
                        return Err(InterpreterError::lex(
                            "Unterminated escape in string",
                            start_line,
                            start_col,
                        ));
                    }
                    let escaped = chars[pos];
                    match escaped {
                        'n' => s.push('\n'),
                        't' => s.push('\t'),
                        '\\' => s.push('\\'),
                        '"' => s.push('"'),
                        _ => {
                            return Err(InterpreterError::lex(
                                format!("Unknown escape sequence: \\{escaped}"),
                                line,
                                col,
                            ));
                        }
                    }
                    pos += 1;
                    col += 1;
                    continue;
                }
                if c == '\n' {
                    line += 1;
                    col = 1;
                } else {
                    col += 1;
                }
                s.push(c);
                pos += 1;
            }
            tokens.push(Spanned {
                token: Token::Str(s),
                line: start_line,
                col: start_col,
            });
            continue;
        }

        // Numbers (integer, hex, float)
        if ch.is_ascii_digit() {
            let mut num_str = String::new();
            let is_hex = ch == '0' && pos + 1 < chars.len() && (chars[pos + 1] == 'x' || chars[pos + 1] == 'X');

            if is_hex {
                // Skip 0x
                pos += 2;
                col += 2;
                while pos < chars.len() && (chars[pos].is_ascii_hexdigit() || chars[pos] == '_') {
                    if chars[pos] != '_' {
                        num_str.push(chars[pos]);
                    }
                    pos += 1;
                    col += 1;
                }
                if num_str.is_empty() {
                    return Err(InterpreterError::lex(
                        "Expected hex digits after 0x",
                        start_line,
                        start_col,
                    ));
                }
                let value = i64::from_str_radix(&num_str, 16).map_err(|_| {
                    InterpreterError::lex(
                        format!("Invalid hex literal: 0x{num_str}"),
                        start_line,
                        start_col,
                    )
                })?;
                tokens.push(Spanned {
                    token: Token::Int(value),
                    line: start_line,
                    col: start_col,
                });
            } else {
                // Decimal number (possibly float)
                let mut is_float = false;
                while pos < chars.len() && (chars[pos].is_ascii_digit() || chars[pos] == '_') {
                    if chars[pos] != '_' {
                        num_str.push(chars[pos]);
                    }
                    pos += 1;
                    col += 1;
                }
                // Check for decimal point (but not method call like `16.method`)
                if pos < chars.len() && chars[pos] == '.' {
                    // Look ahead: if next char is a digit, it's a float
                    if pos + 1 < chars.len() && chars[pos + 1].is_ascii_digit() {
                        is_float = true;
                        num_str.push('.');
                        pos += 1;
                        col += 1;
                        while pos < chars.len() && (chars[pos].is_ascii_digit() || chars[pos] == '_') {
                            if chars[pos] != '_' {
                                num_str.push(chars[pos]);
                            }
                            pos += 1;
                            col += 1;
                        }
                    }
                }

                if is_float {
                    let value: f64 = num_str.parse().map_err(|_| {
                        InterpreterError::lex(
                            format!("Invalid float literal: {num_str}"),
                            start_line,
                            start_col,
                        )
                    })?;
                    tokens.push(Spanned {
                        token: Token::Float(value),
                        line: start_line,
                        col: start_col,
                    });
                } else {
                    let value: i64 = num_str.parse().map_err(|_| {
                        InterpreterError::lex(
                            format!("Invalid integer literal: {num_str}"),
                            start_line,
                            start_col,
                        )
                    })?;
                    tokens.push(Spanned {
                        token: Token::Int(value),
                        line: start_line,
                        col: start_col,
                    });
                }
            }
            continue;
        }

        // Identifiers and keywords
        if ch.is_ascii_alphabetic() || ch == '_' {
            let mut ident = String::new();
            while pos < chars.len() && (chars[pos].is_ascii_alphanumeric() || chars[pos] == '_') {
                ident.push(chars[pos]);
                pos += 1;
                col += 1;
            }

            // Check for unsupported keywords
            if let Some(err) = check_unsupported_keyword(&ident, start_line, start_col) {
                return Err(err);
            }

            tokens.push(Spanned {
                token: Token::Ident(ident),
                line: start_line,
                col: start_col,
            });
            continue;
        }

        // Punctuation
        let tok = match ch {
            '.' => Token::Dot,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            '|' => Token::Pipe,
            '!' => Token::Bang,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            ':' if pos + 1 < chars.len() && chars[pos + 1] == ':' => {
                return Err(InterpreterError::unsupported(
                    "Just use the variant name directly, like TopToBottom instead of LayoutDirection::TopToBottom!",
                ));
            }
            ':' => Token::Colon,
            '+' | '-' | '*' | '/' | '=' | '<' | '>' | '&' | '%' | '^' => {
                return Err(InterpreterError::unsupported(
                    "This is a little demo with a custom interpreter. Math and operators aren't supported!",
                ));
            }
            '#' => {
                return Err(InterpreterError::unsupported(
                    "This is a little demo with a custom interpreter. Attributes aren't supported!",
                ));
            }
            _ => {
                return Err(InterpreterError::lex(
                    format!("Unexpected character: '{ch}'"),
                    start_line,
                    start_col,
                ));
            }
        };

        tokens.push(Spanned {
            token: tok,
            line: start_line,
            col: start_col,
        });
        pos += 1;
        col += 1;
    }

    tokens.push(Spanned {
        token: Token::Eof,
        line,
        col,
    });
    Ok(tokens)
}

fn check_unsupported_keyword(
    name: &str,
    _line: usize,
    _col: usize,
) -> Option<InterpreterError> {
    let msg = match name {
        "let" | "mut" => {
            "This is a little demo with a custom interpreter. Variables aren't supported!"
        }
        "fn" => "This is a little demo. You can't define functions here!",
        "if" | "else" => "This is a little demo. Conditionals aren't supported!",
        "for" | "while" | "loop" => "This is a little demo. Loops aren't supported!",
        "match" => "This is a little demo. Pattern matching isn't supported!",
        "use" | "mod" | "pub" | "crate" | "extern" => {
            "This is a little demo. You don't need imports!"
        }
        "struct" | "enum" | "impl" | "trait" | "type" => {
            "This is a little demo. You can't define types here!"
        }
        "async" | "await" => "This is a little demo. Async isn't supported!",
        "return" | "break" | "continue" => {
            "This is a little demo. Control flow isn't supported!"
        }
        "println" | "dbg" | "print" | "eprintln" | "panic" | "todo" | "unimplemented" => {
            "This is a little demo. Printing and debugging macros aren't supported!"
        }
        "self" | "Self" | "super" => {
            "This is a little demo. Self references aren't supported!"
        }
        "unsafe" => "This is a little demo. Unsafe code isn't supported!",
        "const" | "static" => {
            "This is a little demo. Constant and static declarations aren't supported!"
        }
        "true" | "false" => return None, // booleans are fine as idents, but we don't use them
        _ => return None,
    };
    Some(InterpreterError::unsupported(msg))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_simple_chain() {
        let tokens = lex("ui.element().empty();").unwrap();
        let kinds: Vec<_> = tokens.iter().map(|t| &t.token).collect();
        assert_eq!(
            kinds,
            vec![
                &Token::Ident("ui".into()),
                &Token::Dot,
                &Token::Ident("element".into()),
                &Token::LParen,
                &Token::RParen,
                &Token::Dot,
                &Token::Ident("empty".into()),
                &Token::LParen,
                &Token::RParen,
                &Token::Semicolon,
                &Token::Eof,
            ]
        );
    }

    #[test]
    fn lex_hex_color() {
        let tokens = lex("0xFFC32C").unwrap();
        assert_eq!(tokens[0].token, Token::Int(0xFFC32C));
    }

    #[test]
    fn lex_float() {
        let tokens = lex("6.0").unwrap();
        assert_eq!(tokens[0].token, Token::Float(6.0));
    }

    #[test]
    fn lex_string() {
        let tokens = lex(r#""Hello, Ply!""#).unwrap();
        assert_eq!(tokens[0].token, Token::Str("Hello, Ply!".into()));
    }

    #[test]
    fn lex_macro_call() {
        let tokens = lex("grow!()").unwrap();
        let kinds: Vec<_> = tokens.iter().map(|t| &t.token).collect();
        assert_eq!(
            kinds,
            vec![
                &Token::Ident("grow".into()),
                &Token::Bang,
                &Token::LParen,
                &Token::RParen,
                &Token::Eof,
            ]
        );
    }

    #[test]
    fn lex_named_macro_args() {
        let tokens = lex("grow!(min: 100.0, weight: 2.0)").unwrap();
        let kinds: Vec<_> = tokens.iter().map(|t| &t.token).collect();
        assert_eq!(
            kinds,
            vec![
                &Token::Ident("grow".into()),
                &Token::Bang,
                &Token::LParen,
                &Token::Ident("min".into()),
                &Token::Colon,
                &Token::Float(100.0),
                &Token::Comma,
                &Token::Ident("weight".into()),
                &Token::Colon,
                &Token::Float(2.0),
                &Token::RParen,
                &Token::Eof,
            ]
        );
    }

    #[test]
    fn lex_rejects_let() {
        let err = lex("let x = 5;").unwrap_err();
        match err {
            InterpreterError::Unsupported { message } => {
                assert!(message.contains("Variables"), "got: {message}");
            }
            other => panic!("Expected Unsupported, got: {other:?}"),
        }
    }

    #[test]
    fn lex_rejects_path_syntax() {
        let err = lex("LayoutDirection::TopToBottom").unwrap_err();
        match err {
            InterpreterError::Unsupported { message } => {
                assert!(message.contains("variant name"), "got: {message}");
            }
            other => panic!("Expected Unsupported, got: {other:?}"),
        }
    }

    #[test]
    fn lex_closure() {
        let tokens = lex("|l| l.gap(8)").unwrap();
        let kinds: Vec<_> = tokens.iter().map(|t| &t.token).collect();
        assert_eq!(
            kinds,
            vec![
                &Token::Pipe,
                &Token::Ident("l".into()),
                &Token::Pipe,
                &Token::Ident("l".into()),
                &Token::Dot,
                &Token::Ident("gap".into()),
                &Token::LParen,
                &Token::Int(8),
                &Token::RParen,
                &Token::Eof,
            ]
        );
    }
}
