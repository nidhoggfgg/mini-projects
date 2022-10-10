#[derive(Debug)]
pub enum Stmt {
    Fun { idx: u64, body: Box<Expr> },
    Assign { idx: u64, expr: Box<Expr> },
    Expr { expr: Box<Expr> },
}

#[derive(Debug)]
pub enum Valuable {
    Value(f64),
    Arg(usize),
    Var(u64),
}

#[derive(Debug)]
pub enum Expr {
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
    Fun {
        idx: u64,
        args: Vec<f64>,
    },
}

#[derive(Debug)]
pub enum UnaryOp {
    Minus,
    Ftl,
}

#[derive(Debug)]
pub enum BinaryOp {
    Plus,
    Sub,
    Mult,
    Div,
    Square,
}