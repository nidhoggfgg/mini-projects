use std::collections::HashMap;

use crate::{
    ast::{BinaryOp, Expr, Stmt, UnaryOp, Valuable},
    lexer::Scanner,
    parser::Parser,
    utils::{factorial, hash_it, print_err},
};

pub struct Env {
    functions: HashMap<u64, Box<Expr>>,
    builtin: HashMap<u64, Box<dyn Fn(f64) -> f64>>,
    global: HashMap<u64, f64>,
    var_name: Option<HashMap<u64, String>>,
}

impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}

impl Env {
    pub fn new() -> Self {
        let builtin = HashMap::from([
            (hash_it(&"ln"), Box::new(f64::ln) as Box<_>),
            (hash_it(&"lg"), Box::new(f64::log10) as Box<_>),
            (hash_it(&"sin"), Box::new(f64::sin) as Box<_>),
            (hash_it(&"cos"), Box::new(f64::cos) as Box<_>),
            (hash_it(&"tan"), Box::new(f64::tan) as Box<_>),
            (hash_it(&"acos"), Box::new(f64::acos) as Box<_>),
            (hash_it(&"asin"), Box::new(f64::asin) as Box<_>),
            (hash_it(&"atan"), Box::new(f64::atan) as Box<_>),
            (hash_it(&"sqrt"), Box::new(f64::sqrt) as Box<_>),
            (hash_it(&"abs"), Box::new(f64::abs) as Box<_>),
            (hash_it(&"sinh"), Box::new(f64::sinh) as Box<_>),
            (hash_it(&"cosh"), Box::new(f64::cosh) as Box<_>),
            (hash_it(&"cosh"), Box::new(f64::cosh) as Box<_>),
            (hash_it(&"floor"), Box::new(f64::floor) as Box<_>),
        ]);

        let global = HashMap::from([
            (hash_it(&"PI"), std::f64::consts::PI),
            (hash_it(&"E"), std::f64::consts::E),
        ]);
        Env {
            functions: HashMap::new(),
            builtin,
            global,
            var_name: None,
        }
    }

    pub fn run(&mut self, s: &str) -> Option<f64> {
        let mut lexer = Scanner::new(s.chars());
        let tokens = lexer.scan();
        let var_name = lexer.pop_var_name();
        let mut parser = Parser::new(tokens.into_iter());
        let ast = parser.parse()?;
        self.push_var_name(var_name);
        self.run_impl(*ast)
    }

    fn run_impl(&mut self, stmt: Stmt) -> Option<f64> {
        match stmt {
            Stmt::Fun { idx, body } => {
                #[cfg(feature = "runtime_dev")]
                println!("visit fun define: {}", idx);
                self.functions.insert(idx, body);
                None
            }
            Stmt::Expr { expr } => {
                #[cfg(feature = "runtime_dev")]
                println!("visit expression");
                expr.value(self, None)
            }
            Stmt::Assign { idx, expr } => {
                #[cfg(feature = "runtime_dev")]
                println!("visit variable assign: {}", idx);
                let value = expr.value(self, None)?;
                self.global.insert(idx, value);
                None
            }
        }
    }

    fn push_var_name(&mut self, var_name: HashMap<u64, String>) {
        self.var_name = Some(var_name);
    }

    fn find_name(&self, idx: u64) -> Option<&str> {
        if let Some(namespace) = &self.var_name {
            namespace.get(&idx).map(|x| &**x)
        } else {
            None
        }
    }
}

trait Value {
    fn value(&self, env: &Env, locals: Option<&[f64]>) -> Option<f64>;
}

impl Value for Valuable {
    fn value(&self, env: &Env, locals: Option<&[f64]>) -> Option<f64> {
        match self {
            Self::Value(v) => {
                #[cfg(feature = "runtime_dev")]
                println!("visit value: {}", v);
                Some(*v)
            }
            Self::Arg(i) => {
                #[cfg(feature = "runtime_dev")]
                println!("visit arg: {}", i);
                if let Some(v) = locals?.get(*i) {
                    Some(*v)
                } else {
                    print_err!("too little values give");
                    None
                }
            }
            Self::Var(idx) => {
                #[cfg(feature = "runtime_dev")]
                println!("visit variable: {}", idx);
                if let Some(v) = env.global.get(idx) {
                    Some(*v)
                } else {
                    print_err!("can't find variable named '{}'", env.find_name(*idx).unwrap_or("Unknown"));
                    None
                }
            }
        }
    }
}

impl Value for Expr {
    fn value(&self, env: &Env, locals: Option<&[f64]>) -> Option<f64> {
        match self {
            Expr::Literal { value } => {
                #[cfg(feature = "runtime_dev")]
                println!("visit literal: {:?}", value);
                value.value(env, locals)
            }
            Expr::Binary { left, op, right } => {
                #[cfg(feature = "runtime_dev")]
                println!("visit binary: op: {:?}", op);
                let lv = left.value(env, locals)?;
                let rv = right.value(env, locals)?;
                let result = match op {
                    BinaryOp::Plus => lv + rv,
                    BinaryOp::Sub => lv - rv,
                    BinaryOp::Mult => lv * rv,
                    BinaryOp::Div => lv / rv,
                    BinaryOp::Square => lv.powf(rv),
                };
                Some(result)
            }
            Expr::Call { idx, args } => {
                #[cfg(feature = "runtime_dev")]
                println!("visit call: {:?}", idx);
                let mut ths_locals = Vec::new();
                for e in args {
                    let v = e.value(env, locals)?;
                    ths_locals.push(v);
                }
                if let Some((_, body)) = env.functions.get_key_value(idx) {
                    body.value(env, Some(&ths_locals))
                } else if let Some((_, f)) = env.builtin.get_key_value(idx) {
                    if ths_locals.len() != 1 {
                        print_err!("need and only need 1 argument");
                        return None;
                    }
                    let v = f(ths_locals[0]);
                    Some(v)
                } else {
                    print_err!("function {} is not defined", env.find_name(*idx).unwrap_or("Unknown"));
                    None
                }
            }
            Expr::Unary { op, operand } => {
                #[cfg(feature = "runtime_dev")]
                println!("visit unary: op: {:?}", op);
                let value = operand.value(env, locals)?;
                let result = match op {
                    UnaryOp::Minus => -value,
                    UnaryOp::Ftl => factorial(value as u32),
                };
                Some(result)
            }
            Expr::Group { body } => {
                #[cfg(feature = "runtime_dev")]
                println!("visit group");
                body.value(env, locals)
            }
        }
    }
}
