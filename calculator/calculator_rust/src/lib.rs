mod ast;
mod lexer;
mod parser;
mod utils;

pub mod calculator {
    use std::collections::HashMap;

    use crate::{
        ast::{BinaryOp, Expr, Stmt, UnaryOp, Valuable},
        lexer::Scanner,
        parser::Parser,
        utils::{self, hash_it, print_err},
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
                (hash_it("exp"), Box::new(f64::exp) as Box<_>),
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
                Expr::Fun {
                    idx,
                    locals: values,
                } => {
                    env.locals.clear();
                    for v in values {
                        env.locals.push(*v);
                    }
                    if let Some((name, body)) = env.functions.remove_entry(idx) {
                        let result = body.value(env);
                        env.functions.insert(name, body);
                        result
                    } else if let Some(f) = env.builtin.get(idx) {
                        Some(f(values[0]))
                    } else {
                        utils::print_err!("function is not defined");
                        None
                    }
                }
                Expr::Unary { op, operand } => {
                    let value = operand.value(env)?;
                    let result = match op {
                        UnaryOp::Minus => -value,
                        UnaryOp::Ftl => utils::factorial(value as u32),
                    };
                    Some(result)
                }
                Expr::Group { body } => body.value(env),
            }
        }
    }
}
