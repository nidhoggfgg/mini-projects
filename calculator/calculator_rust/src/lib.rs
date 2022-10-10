mod ast;
mod lexer;
mod parser;
mod utils;

use std::collections::HashMap;

use crate::{
    ast::{BinaryOp, Expr, Stmt, UnaryOp, Valuable},
    lexer::Scanner,
    parser::Parser,
    utils::{factorial, hash_it, print_err},
};

pub struct Env {
    functions: HashMap<u64, Box<Expr>>,
    locals: Vec<f64>,
    builtin: HashMap<u64, Box<dyn Fn(f64) -> f64>>,
    global: HashMap<u64, f64>,
}

impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}

impl Env {
    pub fn new() -> Self {
        let builtin = HashMap::from([
            (hash_it("ln"), Box::new(f64::ln) as Box<_>),
            (hash_it("lg"), Box::new(f64::log10) as Box<_>),
            (hash_it("sin"), Box::new(f64::sin) as Box<_>),
            (hash_it("cos"), Box::new(f64::cos) as Box<_>),
            (hash_it("tan"), Box::new(f64::tan) as Box<_>),
            (hash_it("acos"), Box::new(f64::acos) as Box<_>),
            (hash_it("asin"), Box::new(f64::asin) as Box<_>),
            (hash_it("atan"), Box::new(f64::atan) as Box<_>),
            (hash_it("sqrt"), Box::new(f64::sqrt) as Box<_>),
            (hash_it("abs"), Box::new(f64::abs) as Box<_>),
            (hash_it("sinh"), Box::new(f64::sinh) as Box<_>),
            (hash_it("cosh"), Box::new(f64::cosh) as Box<_>),
            (hash_it("cosh"), Box::new(f64::cosh) as Box<_>),
            (hash_it("floor"), Box::new(f64::floor) as Box<_>),
        ]);

        let global = HashMap::from([
            (hash_it("PI"), std::f64::consts::PI),
            (hash_it("E"), std::f64::consts::E),
        ]);
        Env {
            functions: HashMap::new(),
            locals: Vec::new(),
            builtin,
            global,
        }
    }

    pub fn run(&mut self, s: &str) -> Option<f64> {
        let mut lexer = Scanner::new(s.chars());
        let tokens = lexer.scan();
        let mut parser = Parser::new(tokens.into_iter());
        let ast = parser.parse()?;
        self.run_impl(*ast)
    }

    fn run_impl(&mut self, stmt: Stmt) -> Option<f64> {
        match stmt {
            Stmt::Fun { idx, body } => {
                self.functions.insert(idx, body);
                None
            }
            Stmt::Expr { expr } => expr.value(self),
            Stmt::Assign { idx, expr } => {
                let value = expr.value(self)?;
                self.global.insert(idx, value);
                None
            }
        }
    }
}

trait Value {
    fn value(&self, env: &mut Env) -> Option<f64>;
}

impl Value for Valuable {
    fn value(&self, env: &mut Env) -> Option<f64> {
        match self {
            Self::Value(v) => Some(*v),
            Self::Arg(i) => {
                if let Some(v) = env.locals.get(*i) {
                    Some(v).copied()
                } else {
                    print_err!("too little values give");
                    None
                }
            }
            Self::Var(name) => {
                if let Some(v) = env.global.get(name) {
                    Some(v).copied()
                } else {
                    print_err!("can't find variable named '{}'", name);
                    None
                }
            }
        }
    }
}

impl Value for Expr {
    fn value(&self, env: &mut Env) -> Option<f64> {
        match self {
            Expr::Literal { value } => value.value(env),
            Expr::Binary { left, op, right } => {
                let lv = left.value(env)?;
                let rv = right.value(env)?;
                let result = match op {
                    BinaryOp::Plus => lv + rv,
                    BinaryOp::Sub => lv - rv,
                    BinaryOp::Mult => lv * rv,
                    BinaryOp::Div => lv / rv,
                    BinaryOp::Square => lv.powf(rv),
                };
                Some(result)
            }
            Expr::Fun { idx, args } => {
                env.locals.clear();
                for v in args {
                    env.locals.push(*v);
                }
                if let Some((name, body)) = env.functions.remove_entry(idx) {
                    let result = body.value(env);
                    env.functions.insert(name, body);
                    result
                } else if let Some(f) = env.builtin.get(idx) {
                    if args.len() != 1 {
                        print_err!("need and only need 1 argument");
                        return None;
                    }
                    Some(f(args[0]))
                } else {
                    print_err!("function is not defined");
                    None
                }
            }
            Expr::Unary { op, operand } => {
                let value = operand.value(env)?;
                let result = match op {
                    UnaryOp::Minus => -value,
                    UnaryOp::Ftl => factorial(value as u32),
                };
                Some(result)
            }
            Expr::Group { body } => body.value(env),
        }
    }
}
