#[derive(Debug, Clone)]
pub(crate) enum Stmt {
    Fun { idx: u64, body: Box<Expr> },
    Assign { idx: u64, expr: Box<Expr> },
    Expr { expr: Box<Expr> },
    Magic { kind: MagicKind },
}

#[derive(Debug, Clone)]
pub(crate) enum MagicKind {
    Plot(u64, Box<Expr>, Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone)]
pub(crate) enum MagicArg {
    Idx,
    Expr
}

#[derive(Debug, Clone)]
pub(crate) enum Valuable {
    Value(f64), // normal number
    Arg(usize), // arg in function
    Var(u64),   // variable
}

#[derive(Debug, Clone)]
pub(crate) enum Expr {
    Literal {
        value: Valuable,
    },
    Group {
        body: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        operand: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Call {
        idx: u64,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone)]
pub(crate) enum UnaryOp {
    Minus,
    Ftl,
}

#[derive(Debug, Clone)]
pub(crate) enum BinaryOp {
    Plus,
    Sub,
    Mult,
    Div,
    Square,
}
