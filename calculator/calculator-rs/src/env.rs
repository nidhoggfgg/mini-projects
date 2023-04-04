// todo: the impl of vm-like env is really complicated.
use std::collections::HashMap;

use drawille::Canvas;

use crate::{
    ast::{BinaryOp, Expr, Stmt, UnaryOp, Valuable, MagicKind},
    lexer::Scanner,
    parser::Parser,
    utils::{factorial, hash_it, print_err},
    onemore::OneMore,
};


macro_rules! f64method_to_native {
    ($name:tt) => {
        NativeFun {
            fun: Box::new(|arg:&[f64]| OneMore::One(f64::$name(arg[0]))),
            arg_num: 1,
            return_num: 1,
        }    
    };
}

#[allow(unused)]
struct NativeFun {
    fun: Box<dyn Fn(&[f64]) -> OneMore>,
    arg_num: usize,
    return_num: usize,
}

pub struct Env {
    functions: HashMap<u64, Box<Expr>>,
    builtin: HashMap<u64, NativeFun>,
    global: HashMap<u64, f64>,
    name_space: Option<HashMap<u64, String>>,
}

impl Env {
    pub fn new() -> Self {
        let builtin = HashMap::from([
            (hash_it(&"ln"), f64method_to_native!(ln)),
            (hash_it(&"lg"), f64method_to_native!(log10)),
            (hash_it(&"sin"), f64method_to_native!(sin)),
            (hash_it(&"cos"), f64method_to_native!(cos)),
            (hash_it(&"tan"), f64method_to_native!(tan)),
            (hash_it(&"acos"), f64method_to_native!(acos)),
            (hash_it(&"asin"), f64method_to_native!(asin)),
            (hash_it(&"atan"), f64method_to_native!(atan)),
            (hash_it(&"sqrt"), f64method_to_native!(sqrt)),
            (hash_it(&"abs"),   f64method_to_native!(abs)),
            (hash_it(&"sinh"),  f64method_to_native!(sinh)),
            (hash_it(&"cosh"),  f64method_to_native!(cosh)),
            (hash_it(&"floor"), f64method_to_native!(floor)),
            (hash_it(&"to_rad"),f64method_to_native!(to_radians)),
        ]);
        let global = HashMap::from([
            (hash_it(&"PI"), std::f64::consts::PI),
            (hash_it(&"E"), std::f64::consts::E),
        ]);
        Env {
            functions: HashMap::new(),
            builtin,
            global,
            name_space: None,
        }
    }

    pub fn run(&mut self, s: &str) -> Option<OneMore> {
        let mut lexer = Scanner::new(s.chars());
        let tokens = lexer.scan();
        let namespace = lexer.pop_namespace();
        let mut parser = Parser::new(tokens.into_iter());
        parser.push_namespace(namespace);
        let ast = parser.parse()?;
        let namespace = parser.pop_namespace();
        self.push_namespace(namespace);
        self.run_impl(*ast)
    }

    fn run_impl(&mut self, stmt: Stmt) -> Option<OneMore> {
        match stmt {
            Stmt::Fun { idx, body } => {
                self.functions.insert(idx, body);
                None
            }
            Stmt::Expr { expr } => expr.value(self, None),
            Stmt::Assign { idx, expr } => {
                let value = expr.value(self, None)?.one()?;
                self.global.insert(idx, value);
                None
            }
            Stmt::Magic { kind } => {
                match kind {
                    MagicKind::Plot2d(idx, e1, e2, e3) => {
                        if let Some((_, body)) = self.functions.get_key_value(&idx) {
                            let mut c = Canvas::new();
                            let mut x = e1.value(self, None)?.one()?;
                            let end = e2.value(self, None)?.one()?;
                            let step = e3.value(self, None)?.one()?;
                            while x < end {
                                let y = body.value(self, Some(&[x]))?.one()?;
                                c.set(x, y);
                                x += step;
                            }
                            println!("{}", c.frame());
                            return None;
                        } else {
                            print_err!("can't find function {}", self.find_name(idx).unwrap_or("Unknown"));
                            return None;
                        }
                    }
                }
            }
        }
    }

    fn push_namespace(&mut self, namespace: HashMap<u64, String>) {
        self.name_space = Some(namespace);
    }

    fn find_name(&self, idx: u64) -> Option<&str> {
        if let Some(namespace) = &self.name_space {
            namespace.get(&idx).map(|x| &**x)
        } else {
            None
        }
    }
}

trait Value {
    fn value(&self, env: &Env, locals: Option<&[f64]>) -> Option<OneMore>;
}

impl Value for Valuable {
    fn value(&self, env: &Env, locals: Option<&[f64]>) -> Option<OneMore> {
        match self {
            Self::Value(v) => Some(OneMore::One(*v)),
            Self::Arg(i) => {
                if let Some(v) = locals?.get(*i) {
                    Some(OneMore::One(*v))
                } else {
                    print_err!("too little values give");
                    None
                }
            }
            Self::Var(idx) => {
                if let Some(v) = env.global.get(idx) {
                    Some(OneMore::One(*v))
                } else {
                    print_err!(
                        "can't find variable named '{}'",
                        env.find_name(*idx).unwrap_or("Unknown")
                    );
                    None
                }
            }
        }
    }
}

impl Value for Expr {
    fn value(&self, env: &Env, locals: Option<&[f64]>) -> Option<OneMore> {
        match self {
            Expr::Literal { value } => value.value(env, locals),
            Expr::Binary { left, op, right } => {
                let lv = left.value(env, locals)?.one()?;
                let rv = right.value(env, locals)?.one()?;
                let result = match op {
                    BinaryOp::Plus => lv + rv,
                    BinaryOp::Sub => lv - rv,
                    BinaryOp::Mult => lv * rv,
                    BinaryOp::Div => lv / rv,
                    BinaryOp::Square => lv.powf(rv),
                };
                Some(OneMore::One(result))
            }
            Expr::Call { idx, args } => {
                let mut this_locals = Vec::new();
                for e in args {
                    let v = e.value(env, locals)?.one()?;
                    this_locals.push(v);
                }
                if let Some((_, body)) = env.functions.get_key_value(idx) {
                    body.value(env, Some(&this_locals))
                } else if let Some((_, f)) = env.builtin.get_key_value(idx) {
                    if this_locals.len() != f.arg_num {
                        print_err!("expect {} arguments, but get {}", f.arg_num, this_locals.len());
                        return None;
                    }
                    let v = (f.fun)(&this_locals);
                    Some(v)
                } else {
                    print_err!(
                        "function {} is not defined",
                        env.find_name(*idx).unwrap_or("Unknown")
                    );
                    None
                }
            }
            Expr::Unary { op, operand } => {
                let value = operand.value(env, locals)?.one()?;
                let result = match op {
                    UnaryOp::Minus => -value,
                    UnaryOp::Ftl => factorial(value as u32),
                };
                Some(OneMore::One(result))
            }
            Expr::Group { body } => body.value(env, locals),
        }
    }
}

impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}
