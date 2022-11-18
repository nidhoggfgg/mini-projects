#[derive(Debug)]
pub(crate) enum Stmt {
    Fun { idx: u64, body: Box<Expr> },
    Assign { idx: u64, expr: Box<Expr> },
    Expr { expr: Box<Expr> },
}

#[derive(Debug)]
pub(crate) enum Valuable {
    Value(f64), // normal number
    Arg(usize), // arg in function
    Var(u64),   // variable
}

#[derive(Debug)]
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

#[derive(Debug)]
pub(crate) enum UnaryOp {
    Minus,
    Ftl,
}

#[derive(Debug)]
pub(crate) enum BinaryOp {
    Plus,
    Sub,
    Mult,
    Div,
    Square,
}
