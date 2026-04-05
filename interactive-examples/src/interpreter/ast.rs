/// A complete parsed program (all statements between ply.begin and ui.show).
#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

/// A single statement. Currently always an expression followed by `;`.
#[derive(Debug, Clone)]
pub enum Statement {
    Expr(Expr),
}

/// An expression in the Ply subset.
#[derive(Debug, Clone)]
pub enum Expr {
    /// An integer literal: `42`, `0xFFC32C`
    IntLit(i64),

    /// A float literal: `6.0`, `0.5`
    FloatLit(f64),

    /// A string literal: `"Hello, Ply!"`
    StringLit(String),

    /// A bare identifier: `ui`, `CenterX`, `TopToBottom`
    Ident(String),

    /// A tuple expression: `(1, 2, 3, 4)`
    Tuple(Vec<Expr>),

    /// A method call: `receiver.method(args...)`
    MethodCall {
        receiver: Box<Expr>,
        method: String,
        args: Vec<Expr>,
    },

    /// A macro call: `grow!()`, `fixed!(200.0)`
    MacroCall {
        name: String,
        args: Vec<Expr>,
    },

    /// A named argument expression used in macro calls, e.g. `min: 100.0`.
    NamedArg {
        name: String,
        value: Box<Expr>,
    },

    /// A closure: `|a, b| expr` or `|a| { stmts }`
    Closure {
        params: Vec<String>,
        body: Box<Expr>,
    },

    /// A block: `{ stmt; stmt; }`
    Block {
        statements: Vec<Statement>,
    },
}
