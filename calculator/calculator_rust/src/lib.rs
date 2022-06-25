mod lexer {
    use std::collections::HashMap;

    use crate::utils;

    #[derive(Clone, Debug)]
    pub enum Token {
        LeftParen,
        RightParen,
        Plus,
        Minus,
        Star,
        Slash,
        Bang,
        Square,
        Eq,
        Fun,
        Number(f64),
        Ident(String),
        Unkown,
        Eof,
    }

    pub struct Scanner<T: Iterator<Item = char>> {
        source: T,
        next: Option<char>,
        kw: HashMap<&'static str, Token>,
    }

    impl<T: Iterator<Item = char>> Scanner<T> {
        pub fn new(source: T) -> Self {
            let mut scanner = Scanner {
                source,
                next: None,
                kw: HashMap::from([("fun", Token::Fun)]),
            };
            scanner.eat();
            scanner
        }

        pub fn scan(&mut self) -> Vec<Token> {
            let mut tokens = Vec::with_capacity(16);
            while let Some(t) = self.scan_token() {
                tokens.push(t);
            }
            tokens.push(Token::Eof);
            tokens
        }

        fn scan_token(&mut self) -> Option<Token> {
            self.skip_space();
            let c = self.next.take()?;
            self.eat();

            if utils::is_identifier_start(c) {
                return Some(self.ident_or_kw(c));
            }

            let token = match c {
                '(' => Token::LeftParen,
                ')' => Token::RightParen,
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Star,
                '/' => Token::Slash,
                '!' => Token::Bang,
                '^' => Token::Square,
                '=' => Token::Eq,
                '0'..='9' => self.number(c),
                _ => Token::Unkown,
            };

            Some(token)
        }

        fn ident_or_kw(&mut self, start: char) -> Token {
            let mut lexeme = String::with_capacity(4);
            lexeme.push(start);

            while let Some(c) = self.next {
                if c.is_alphanumeric() {
                    lexeme.push(c);
                    self.eat();
                } else {
                    break;
                }
            }

            if self.kw.contains_key(lexeme.as_str()) {
                let kw = self.kw.get(lexeme.as_str()).unwrap();
                return kw.clone();
            }

            Token::Ident(lexeme)
        }

        fn number(&mut self, start: char) -> Token {
            let mut lexeme = String::with_capacity(4);
            lexeme.push(start);
            while let Some(c) = self.next {
                if utils::is_number(c) {
                    lexeme.push(c);
                    self.eat();
                } else {
                    break;
                }
            }

            if let Some('.') = self.next {
                self.eat();
                lexeme.push('.');

                while let Some(c) = self.next {
                    if utils::is_number(c) {
                        lexeme.push(c);
                        self.eat();
                    } else {
                        break;
                    }
                }
            }

            let value = lexeme.parse().unwrap();
            Token::Number(value)
        }

        fn skip_space(&mut self) {
            while let Some(c) = self.next {
                match c {
                    ' ' | '\t' | '\r' | '\n' => self.eat(),
                    _ => break,
                }
            }
        }

        fn eat(&mut self) {
            self.next = self.source.next();
        }
    }
}

mod ast {
    #[derive(Debug)]
    pub enum Stmt {
        Fun { name: String, body: Box<Expr> },
        Assign { name: String, expr: Box<Expr> },
        Expr { expr: Box<Expr> },
    }

    #[derive(Debug)]
    pub enum Valuable {
        Value(f64),
        Arg(usize),
        Var(String),
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
            name: String,
            locals: Vec<f64>,
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
}

mod parser {
    use std::collections::HashMap;
    use std::mem::discriminant;

    use crate::ast::{BinaryOp, Expr, Stmt, UnaryOp, Valuable};
    use crate::lexer::Token;
    use crate::utils::print_err;

    pub struct Parser<T: Iterator<Item = Token>> {
        tokens: T,
        next: Option<Token>,
        args: HashMap<String, usize>,
    }

    impl<T: Iterator<Item = Token>> Parser<T> {
        pub fn new(tokens: T) -> Self {
            let mut parser = Parser {
                tokens,
                next: None,
                args: HashMap::new(),
            };

            parser.eat();
            parser
        }

        pub fn parse(&mut self) -> Option<Box<Stmt>> {
            let start = self.next().unwrap();

            let stmt = match start {
                Token::Fun => self.fun(),
                Token::Ident(_) => self.assign(start),
                Token::Eof => {
                    return None;
                }
                Token::Unkown => {
                    print_err!("invalid char");
                    None
                }
                _ => {
                    let expr = self.expr(start)?;
                    let stmt = Stmt::Expr { expr };
                    Some(Box::new(stmt))
                }
            };

            if !self.expect(Token::Eof) {
                print_err!("invalid syntax");
                return None;
            }

            stmt
        }

        fn assign(&mut self, start: Token) -> Option<Box<Stmt>> {
            if !self.check(Token::Eq) {
                let expr = self.expr(start)?;
                return Some(Box::new(Stmt::Expr { expr }));
            }
            self.eat();

            if self.is_at_end() {
                return None;
            }

            let expr_start = self.next().unwrap();
            let expr = self.expr(expr_start)?;

            let name = if let Token::Ident(name) = start {
                name
            } else {
                print_err!("expect a name but get {:?}, this is a bug!", start);
                return None;
            };

            Some(Box::new(Stmt::Assign { name, expr }))
        }

        fn fun(&mut self) -> Option<Box<Stmt>> {
            if self.is_at_end() {
                print_err!("expect a name after 'fun'");
                return None;
            }

            let name = if let Some(Token::Ident(name)) = self.next() {
                name
            } else {
                print_err!("expect a name after 'fun'");
                return None;
            };

            if !self.expect(Token::LeftParen) {
                print_err!("expect '(' after '{}'", name);
                return None;
            }

            let mut count = 0;
            while self.check(Token::Ident("".into())) {
                let name = if let Some(Token::Ident(name)) = self.next.take() {
                    self.eat();
                    name
                } else {
                    print_err!("expect a name, this is a bug!");
                    return None;
                };

                if count == usize::MAX {
                    print_err!("to many args");
                    return None;
                }

                self.args.insert(name, count);
                count += 1;
            }

            if !self.expect(Token::RightParen) {
                print_err!("missing ')'");
                return None;
            }

            if !self.expect(Token::Eq) {
                print_err!("expect '='");
                return None;
            }

            if self.is_at_end() {
                print_err!("expect a expression after '='");
                return None;
            }

            let start = self.next().unwrap();

            let body = self.expr(start)?;
            let stmt = Stmt::Fun { name, body };

            // dont forget clear the args!
            self.args.clear();
            Some(Box::new(stmt))
        }

        fn expr(&mut self, start: Token) -> Option<Box<Expr>> {
            self.plus_sub(start)
        }

        fn plus_sub(&mut self, start: Token) -> Option<Box<Expr>> {
            let mut left = self.mult_div(start)?;

            while self.check(Token::Plus) || self.check(Token::Minus) {
                let op = match self.next().unwrap() {
                    Token::Plus => BinaryOp::Plus,
                    Token::Minus => BinaryOp::Sub,
                    _ => BinaryOp::Plus, // impassiable
                };

                if self.is_at_end() {
                    print_err!("expect a expression after '+' or '-'");
                    return None;
                }

                let start = self.next().unwrap();
                let right = self.mult_div(start)?;
                left = Box::new(Expr::Binary { left, op, right })
            }

            Some(left)
        }

        fn mult_div(&mut self, start: Token) -> Option<Box<Expr>> {
            let mut left = self.square(start)?;

            while self.check(Token::Star) || self.check(Token::Slash) {
                let op = match self.next().unwrap() {
                    Token::Star => BinaryOp::Mult,
                    Token::Slash => BinaryOp::Div,
                    _ => BinaryOp::Mult, // impassiable
                };

                if self.is_at_end() {
                    print_err!("expect a expression after '*' or '/'");
                    return None;
                }

                let start = self.next().unwrap();
                let right = self.square(start)?;
                left = Box::new(Expr::Binary { left, op, right })
            }

            Some(left)
        }

        fn square(&mut self, start: Token) -> Option<Box<Expr>> {
            let mut left = self.minus(start)?;

            while self.check(Token::Square) {
                self.eat();
                let op = BinaryOp::Square;

                if self.is_at_end() {
                    print_err!("expect a expression after '^'");
                    return None;
                }

                let start = self.next().unwrap();
                let right = self.minus(start)?;
                left = Box::new(Expr::Binary { left, op, right })
            }

            Some(left)
        }

        fn minus(&mut self, start: Token) -> Option<Box<Expr>> {
            if let Token::Minus = start {
                let op = UnaryOp::Minus;

                if self.is_at_end() {
                    print_err!("expect a expression after '-'");
                    return None;
                }

                let start = self.next().unwrap();
                let operand = self.minus(start)?;
                return Some(Box::new(Expr::Unary { op, operand }));
            }

            self.factorial(start)
        }

        fn factorial(&mut self, start: Token) -> Option<Box<Expr>> {
            let mut operand = self.call(start)?;

            if self.check(Token::Bang) {
                let op = UnaryOp::Ftl;
                self.eat();
                operand = Box::new(Expr::Unary { op, operand });
            }
            Some(operand)
        }

        fn call(&mut self, start: Token) -> Option<Box<Expr>> {
            if let (Token::Ident(_), Some(Token::LeftParen)) = (&start, &self.next) {
                let name = if let Token::Ident(name) = start {
                    name
                } else {
                    print_err!("expect a name to call a function");
                    return None;
                };
                self.eat();

                let mut values = Vec::new();

                while let Some(Token::Number(_)) = &self.next {
                    let num = if let Some(Token::Number(num)) = self.next() {
                        num
                    } else {
                        print_err!("expect number in function call");
                        return None;
                    };
                    values.push(num);
                }

                if !self.expect(Token::RightParen) {
                    print_err!("missing ')'");
                    return None;
                }

                return Some(Box::new(Expr::Fun {
                    name,
                    locals: values,
                }));
            }

            self.primary(start)
        }

        fn primary(&mut self, start: Token) -> Option<Box<Expr>> {
            match start {
                Token::Ident(name) => {
                    if let Some(i) = self.args.get(&name) {
                        Some(Box::new(Expr::Literal {
                            value: Valuable::Arg(*i),
                        }))
                    } else {
                        Some(Box::new(Expr::Literal {
                            value: Valuable::Var(name),
                        }))
                    }
                }
                Token::Number(num) => Some(Box::new(Expr::Literal {
                    value: Valuable::Value(num),
                })),
                Token::LeftParen => {
                    let start = self.next()?;
                    let v = self.expr(start)?;
                    if !self.expect(Token::RightParen) {
                        print_err!("missing ')'");
                        return None;
                    }
                    Some(Box::new(Expr::Group { body: v }))
                }
                _ => {
                    print_err!("invalid syntax");
                    None
                }
            }
        }

        fn next(&mut self) -> Option<Token> {
            let next = self.next.take();
            self.eat();
            next
        }

        fn eat(&mut self) {
            self.next = self.tokens.next();
        }

        fn check(&self, token: Token) -> bool {
            if let Some(t) = &self.next {
                if discriminant(t) == discriminant(&token) {
                    return true;
                }
                false
            } else {
                false
            }
        }

        fn is_at_end(&self) -> bool {
            self.check(Token::Eof)
        }

        fn expect(&mut self, token: Token) -> bool {
            if self.check(token) {
                self.eat();
                return true;
            }

            false
        }
    }
}

pub mod calculator {
    use std::collections::HashMap;

    use crate::{
        ast::{BinaryOp, Expr, Stmt, UnaryOp, Valuable},
        lexer::Scanner,
        parser::Parser,
        utils::{self, print_err},
    };

    pub struct Env {
        functions: HashMap<String, Box<Expr>>,
        locals: Vec<f64>,
        builtin: HashMap<&'static str, Box<dyn Fn(f64) -> f64>>,
        global: HashMap<String, f64>,
    }

    impl Default for Env {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Env {
        pub fn new() -> Self {
            let builtin = HashMap::from([
                ("ln", Box::new(f64::ln) as Box<_>),
                ("lg", Box::new(f64::log10) as Box<_>),
                ("exp", Box::new(f64::exp) as Box<_>),
            ]);

            let global = HashMap::from([
                ("PI".into(), std::f64::consts::PI),
                ("E".into(), std::f64::consts::E),
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
                Stmt::Fun { name, body } => {
                    self.functions.insert(name, body);
                    None
                }
                Stmt::Expr { expr } => expr.value(self),
                Stmt::Assign { name, expr } => {
                    let value = expr.value(self)?;
                    self.global.insert(name, value);
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
                Self::Arg(i) => if let Some(v) = env.locals.get(*i) {
                    Some(v).copied()
                } else {
                    print_err!("too little values give");
                    None
                },
                Self::Var(name) => if let Some(v) = env.global.get(name) {
                    Some(v).copied()
                } else {
                    print_err!("can't find variable named '{}'", name);
                    None
                },
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
                    name,
                    locals: values,
                } => {
                    env.locals.clear();
                    for v in values {
                        env.locals.push(*v);
                    }
                    if let Some((name, body)) = env.functions.remove_entry(name) {
                        let result = body.value(env);
                        env.functions.insert(name, body);
                        result
                    } else if let Some(f) = env.builtin.get(name.as_str()) {
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

mod utils {
    pub fn is_identifier_start(c: char) -> bool {
        c == '_' || c.is_alphabetic()
    }

    pub fn is_number(c: char) -> bool {
        ('0'..='9').contains(&c)
    }

    macro_rules! print_err {
        ($($arg:tt)*) => {
            println!("{}", format_args!($($arg)*));
        };
    }

    pub(crate) use print_err;

    pub fn factorial(num: u32) -> f64 {
        let mut result: u32 = 1;
        for i in 2..=num {
            result = result.wrapping_mul(i);
        }
        result as f64
    }
}
