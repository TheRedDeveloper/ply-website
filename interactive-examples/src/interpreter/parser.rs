use crate::interpreter::ast::*;
use crate::interpreter::error::InterpreterError;
use crate::interpreter::tokens::{Spanned, Token};

/// Parse a token stream into a Program AST.
pub fn parse(tokens: Vec<Spanned>) -> Result<Program, InterpreterError> {
    let mut parser = Parser { tokens, pos: 0 };
    parser.parse_program()
}

struct Parser {
    tokens: Vec<Spanned>,
    pos: usize,
}

impl Parser {
    // ── Helpers ──────────────────────────────────────────────────────

    fn peek(&self) -> &Token {
        &self.tokens[self.pos].token
    }

    fn current_span(&self) -> (usize, usize) {
        let s = &self.tokens[self.pos];
        (s.line, s.col)
    }

    fn advance(&mut self) -> &Spanned {
        let tok = &self.tokens[self.pos];
        if self.pos + 1 < self.tokens.len() {
            self.pos += 1;
        }
        tok
    }

    fn expect(&mut self, expected: &Token) -> Result<(), InterpreterError> {
        if self.peek() == expected {
            self.advance();
            Ok(())
        } else {
            let (line, col) = self.current_span();
            Err(InterpreterError::parse(
                format!("Expected {expected:?}, got {:?}", self.peek()),
                line,
                col,
            ))
        }
    }

    fn expect_ident(&mut self) -> Result<String, InterpreterError> {
        match self.peek().clone() {
            Token::Ident(name) => {
                self.advance();
                Ok(name)
            }
            other => {
                let (line, col) = self.current_span();
                Err(InterpreterError::parse(
                    format!("Expected identifier, got {other:?}"),
                    line,
                    col,
                ))
            }
        }
    }

    // ── Grammar rules ──────────────────────────────────────────────

    fn parse_program(&mut self) -> Result<Program, InterpreterError> {
        let mut statements = Vec::new();
        while *self.peek() != Token::Eof {
            // Allow trailing `}` from enclosing scopes when parsing blocks
            if *self.peek() == Token::RBrace {
                break;
            }
            statements.push(self.parse_statement()?);
        }
        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement, InterpreterError> {
        let expr = self.parse_expr()?;
        self.expect(&Token::Semicolon)?;
        Ok(Statement::Expr(expr))
    }

    fn parse_expr(&mut self) -> Result<Expr, InterpreterError> {
        self.parse_call_expr()
    }

    /// call_expr = primary ("." ident "(" args? ")")*
    fn parse_call_expr(&mut self) -> Result<Expr, InterpreterError> {
        let mut expr = self.parse_primary()?;

        while *self.peek() == Token::Dot {
            self.advance(); // consume '.'
            let method = self.expect_ident()?;
            self.expect(&Token::LParen)?;
            let args = self.parse_args()?;
            self.expect(&Token::RParen)?;
            expr = Expr::MethodCall {
                receiver: Box::new(expr),
                method,
                args,
            };
        }

        Ok(expr)
    }

    /// primary = literal | macro_call | tuple_or_paren | closure | block | ident
    fn parse_primary(&mut self) -> Result<Expr, InterpreterError> {
        match self.peek().clone() {
            Token::Int(n) => {
                self.advance();
                Ok(Expr::IntLit(n))
            }
            Token::Float(f) => {
                self.advance();
                Ok(Expr::FloatLit(f))
            }
            Token::Str(s) => {
                self.advance();
                Ok(Expr::StringLit(s))
            }
            Token::Ident(name) => {
                self.advance();
                // Check: is this a macro call? ident "!" "("
                if *self.peek() == Token::Bang {
                    self.advance(); // consume '!'
                    self.expect(&Token::LParen)?;
                    let args = self.parse_args()?;
                    self.expect(&Token::RParen)?;
                    return Ok(Expr::MacroCall { name, args });
                }
                Ok(Expr::Ident(name))
            }
            Token::LParen => {
                self.advance(); // consume '('
                if *self.peek() == Token::RParen {
                    // Empty tuple / unit — shouldn't really appear, but handle it
                    self.advance();
                    return Ok(Expr::Tuple(Vec::new()));
                }
                let first = self.parse_expr()?;
                if *self.peek() == Token::Comma {
                    // It's a tuple
                    let mut elements = vec![first];
                    while *self.peek() == Token::Comma {
                        self.advance(); // consume ','
                        if *self.peek() == Token::RParen {
                            break; // trailing comma
                        }
                        elements.push(self.parse_expr()?);
                    }
                    self.expect(&Token::RParen)?;
                    Ok(Expr::Tuple(elements))
                } else {
                    // Parenthesized expression
                    self.expect(&Token::RParen)?;
                    Ok(first)
                }
            }
            Token::Pipe => {
                self.parse_closure()
            }
            Token::LBrace => {
                self.parse_block()
            }
            other => {
                let (line, col) = self.current_span();
                Err(InterpreterError::parse(
                    format!("Unexpected token: {other:?}"),
                    line,
                    col,
                ))
            }
        }
    }

    /// closure = "|" params "|" (block | expr)
    fn parse_closure(&mut self) -> Result<Expr, InterpreterError> {
        self.expect(&Token::Pipe)?;
        let mut params = Vec::new();
        if *self.peek() != Token::Pipe {
            params.push(self.expect_ident()?);
            while *self.peek() == Token::Comma {
                self.advance();
                params.push(self.expect_ident()?);
            }
        }
        self.expect(&Token::Pipe)?;

        let body = if *self.peek() == Token::LBrace {
            self.parse_block()?
        } else {
            self.parse_call_expr()?
        };

        Ok(Expr::Closure {
            params,
            body: Box::new(body),
        })
    }

    /// block = "{" statement* "}"
    fn parse_block(&mut self) -> Result<Expr, InterpreterError> {
        self.expect(&Token::LBrace)?;
        let mut statements = Vec::new();
        while *self.peek() != Token::RBrace && *self.peek() != Token::Eof {
            statements.push(self.parse_statement()?);
        }
        self.expect(&Token::RBrace)?;
        Ok(Expr::Block { statements })
    }

    /// args = expr ("," expr)* ","?
    fn parse_args(&mut self) -> Result<Vec<Expr>, InterpreterError> {
        if *self.peek() == Token::RParen {
            return Ok(Vec::new());
        }
        let mut args = vec![self.parse_expr()?];
        while *self.peek() == Token::Comma {
            self.advance();
            if *self.peek() == Token::RParen {
                break; // trailing comma
            }
            args.push(self.parse_expr()?);
        }
        Ok(args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lexer::lex;

    fn parse_code(code: &str) -> Result<Program, InterpreterError> {
        let tokens = lex(code)?;
        parse(tokens)
    }

    #[test]
    fn parse_simple_empty() {
        let prog = parse_code("ui.element().empty();").unwrap();
        assert_eq!(prog.statements.len(), 1);
        match &prog.statements[0] {
            Statement::Expr(Expr::MethodCall { method, .. }) => {
                assert_eq!(method, "empty");
            }
            other => panic!("Expected MethodCall, got {other:?}"),
        }
    }

    #[test]
    fn parse_macro_call() {
        let prog = parse_code("ui.element().width(grow!()).empty();").unwrap();
        assert_eq!(prog.statements.len(), 1);
    }

    #[test]
    fn parse_closure_with_block() {
        let prog = parse_code(
            r#"ui.element().children(|ui| {
                ui.text("hi", |t| t.font_size(16));
            });"#,
        )
        .unwrap();
        assert_eq!(prog.statements.len(), 1);
    }

    #[test]
    fn parse_tuple() {
        let prog = parse_code("ui.element().corner_radius((1.0, 2.0, 3.0, 4.0)).empty();").unwrap();
        assert_eq!(prog.statements.len(), 1);
    }

    #[test]
    fn parse_nested_children() {
        let code = r#"
            ui.element()
                .width(grow!())
                .height(grow!())
                .background_color(0x1E1E2E)
                .layout(|l| l.direction(TopToBottom).padding(24).gap(16))
                .children(|ui| {
                    ui.element()
                        .width(fixed!(200.0))
                        .height(fixed!(80.0))
                        .background_color(0xCBA6F7)
                        .corner_radius(12.0)
                        .empty();
                    ui.text("Hello!", |t| t.font_size(24).color(0xFFFFFF));
                });
        "#;
        let prog = parse_code(code).unwrap();
        assert_eq!(prog.statements.len(), 1);
    }

    #[test]
    fn parse_comprehensive_demo() {
        // This is the exact code from main.rs — exercises every supported feature
        let code = r#"
            // Root container: grow, hex color, layout, overflow
            ui.element()
                .width(grow!())
                .height(grow!())
                .background_color(0x1E1E2E)
                .layout(|l| l.direction(TopToBottom).padding(24).gap(16).align(CenterX, Top))
                .overflow(|o| o.scroll_y())
                .children(|ui| {
                    // Title text
                    ui.text("Interpreter Feature Test", |t| t.font_size(28).color(0xCDD6F4).alignment(CenterX));

                    // Row of cards
                    ui.element()
                        .width(grow!())
                        .height(fit!())
                        .layout(|l| l.direction(LeftToRight).gap(16))
                        .children(|ui| {
                            // border(color, all)
                            ui.element()
                                .width(fixed!(180.0))
                                .height(fixed!(120.0))
                                .background_color(0x313244)
                                .corner_radius(12.0)
                                .border(|b| b.color(0xCBA6F7).all(2))
                                .layout(|l| l.align(CenterX, CenterY))
                                .children(|ui| {
                                    ui.text("Border", |t| t.font_size(18).color(0xCBA6F7));
                                });

                            // 4-tuple corner_radius, id, padding tuple
                            ui.element()
                                .width(fixed!(180.0))
                                .height(fixed!(120.0))
                                .background_color(0x313244)
                                .corner_radius((20.0, 0.0, 20.0, 0.0))
                                .id("card-two")
                                .layout(|l| l.padding((16, 8, 16, 8)).align(CenterX, CenterY))
                                .children(|ui| {
                                    ui.text("4-Corner", |t| t.font_size(16).color(0xA6E3A1));
                                });

                            // overflow(scroll_y)
                            ui.element()
                                .width(fixed!(180.0))
                                .height(fixed!(120.0))
                                .background_color(0x313244)
                                .corner_radius(12.0)
                                .overflow(|o| o.scroll_y())
                                .layout(|l| l.direction(TopToBottom).padding(12).gap(8))
                                .children(|ui| {
                                    ui.text("Scroll me", |t| t.font_size(14).color(0x89B4FA));
                                    ui.text("Line 1", |t| t.font_size(14).color(0xBAC2DE));
                                    ui.text("Line 2", |t| t.font_size(14).color(0xBAC2DE));
                                    ui.text("Line 3", |t| t.font_size(14).color(0xBAC2DE));
                                    ui.text("Line 4", |t| t.font_size(14).color(0xBAC2DE));
                                    ui.text("Line 5", |t| t.font_size(14).color(0xBAC2DE));
                                });
                        });

                    // Floating: attach_parent, anchor, z_index
                    ui.element()
                        .id("float-parent")
                        .width(grow!())
                        .height(fixed!(80.0))
                        .background_color(0x45475A)
                        .corner_radius(8.0)
                        .layout(|l| l.align(CenterX, CenterY))
                        .children(|ui| {
                            ui.text("Floating child anchored top-right", |t| t.font_size(14).color(0xBAC2DE));
                            ui.element()
                                .width(fixed!(70.0))
                                .height(fixed!(26.0))
                                .background_color(0xF38BA8)
                                .corner_radius(6.0)
                                .floating(|f| f.attach_parent().anchor((Right, Top), (Right, Top)).z_index(1))
                                .layout(|l| l.align(CenterX, CenterY))
                                .children(|ui| {
                                    ui.text("Float!", |t| t.font_size(11).color(0x1E1E2E));
                                });
                        });

                    // Text: letter_spacing, line_height, wrap_mode
                    ui.element()
                        .width(grow!())
                        .height(fit!())
                        .background_color(0x313244)
                        .corner_radius(8.0)
                        .layout(|l| l.direction(TopToBottom).padding(16).gap(8))
                        .children(|ui| {
                            ui.text("Text Features:", |t| t.font_size(20).color(0xF9E2AF));
                            ui.text("Letter spacing 5", |t| t.font_size(16).color(0xBAC2DE).letter_spacing(5));
                            ui.text("Line height 40", |t| t.font_size(16).color(0xBAC2DE).line_height(40));
                            ui.text("This text uses wrap mode Words", |t| t.font_size(14).color(0xBAC2DE).wrap_mode(Words));
                        });

                    // border between_children
                    ui.element()
                        .width(grow!())
                        .height(fit!())
                        .background_color(0x313244)
                        .corner_radius(8.0)
                        .border(|b| b.color(0x585B70).between_children(1))
                        .layout(|l| l.direction(TopToBottom).padding(12))
                        .children(|ui| {
                            ui.text("Border between children", |t| t.font_size(16).color(0xF9E2AF));
                            ui.text("Item A", |t| t.font_size(14).color(0xBAC2DE));
                            ui.text("Item B", |t| t.font_size(14).color(0xBAC2DE));
                            ui.text("Item C", |t| t.font_size(14).color(0xBAC2DE));
                        });

                    // .empty() divider
                    ui.element()
                        .width(grow!())
                        .height(fixed!(4.0))
                        .background_color(0xCBA6F7)
                        .corner_radius(2.0)
                        .empty();

                    // percent!() and grow!(min)
                    ui.element()
                        .width(grow!())
                        .height(fit!())
                        .layout(|l| l.direction(LeftToRight).gap(12))
                        .children(|ui| {
                            ui.element()
                                .width(percent!(50.0))
                                .height(fixed!(40.0))
                                .background_color(0x89B4FA)
                                .corner_radius(8.0)
                                .layout(|l| l.align(CenterX, CenterY))
                                .children(|ui| {
                                    ui.text("percent!(50)", |t| t.font_size(14).color(0x1E1E2E));
                                });
                            ui.element()
                                .width(grow!(100.0))
                                .height(fixed!(40.0))
                                .background_color(0xA6E3A1)
                                .corner_radius(8.0)
                                .layout(|l| l.align(CenterX, CenterY))
                                .children(|ui| {
                                    ui.text("grow!(100)", |t| t.font_size(14).color(0x1E1E2E));
                                });
                        });

                    // aspect_ratio
                    ui.element()
                        .width(fixed!(80.0))
                        .height(fixed!(80.0))
                        .background_color(0xF9E2AF)
                        .corner_radius(40.0)
                        .aspect_ratio(1.0)
                        .layout(|l| l.align(CenterX, CenterY))
                        .children(|ui| {
                            ui.text("O", |t| t.font_size(24).color(0x1E1E2E));
                        });
                });
        "#;
        let prog = parse_code(code).unwrap();
        assert_eq!(prog.statements.len(), 1);
    }
}
