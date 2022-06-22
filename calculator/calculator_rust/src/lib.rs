mod lexer {
    use std::collections::HashMap;

    use crate::utils;

    #[derive(Clone, PartialEq, Debug)]
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
            let c = if let Some(c) = self.next {
                self.eat();
                c
            } else {
                return None;
            };

            if utils::is_identifier_start(c) {
                return Some(self.ident_or_builtin(c));
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

        fn ident_or_builtin(&mut self, start: char) -> Token {
            let mut lexeme = String::with_capacity(8);
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
                let builtin = self.kw.get(lexeme.as_str()).unwrap();
                return builtin.clone();
            }

            Token::Ident(lexeme)
        }

        fn number(&mut self, start: char) -> Token {
            let mut lexeme = String::with_capacity(8);
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
                        self.eat();
                        lexeme.push(c);
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
        FunStmt { name: String, body: Box<Expr> },
        AssignStmt { name: String, expr: Box<Expr> },
        ExprStmt { expr: Box<Expr> },
    }

    #[derive(Debug, Clone)]
    pub enum Valuable {
        Float(f64),
        Arg(usize),
        Var(String),
    }

    #[derive(Debug, Clone)]
    pub enum Expr {
        Literal {
            value: Valuable,
        },
        Group {
            body: Box<Expr>,
        },
        Unary {
            op: Unaryop,
            operand: Box<Expr>,
        },
        Binary {
            left: Box<Expr>,
            op: Binaryop,
            right: Box<Expr>,
        },
        Fun {
            name: String,
            values: Vec<f64>,
        },
    }

    #[derive(Debug, Clone)]
    pub enum Unaryop {
        Sub,
        Ftl,
    }

    #[derive(Debug, Clone)]
    pub enum Binaryop {
        Plus,
        Sub,
        Mult,
        Div,
        Square,
    }
}

mod parser {
    use std::collections::HashMap;

    use crate::utils::print_err;
    use crate::{lexer::Token, utils};

    use crate::ast::{Binaryop, Expr, Stmt, Unaryop, Valuable};

    pub struct Parser<T: Iterator<Item = Token>> {
        tokens: T,
        next: Option<Token>,
        args: HashMap<String, usize>,
        count: usize,
    }

    impl<T: Iterator<Item = Token>> Parser<T> {
        pub fn new(tokens: T) -> Self {
            let mut parser = Parser {
                tokens,
                next: None,
                args: HashMap::new(),
                count: 0,
            };

            parser.eat();
            parser
        }

        pub fn parse(&mut self) -> Option<Box<Stmt>> {
            let start = self.next.take().unwrap();
            self.eat();

            let stmt = match start {
                Token::Fun => self.fun(),
                Token::Unkown => {
                    print_err("invalid syntax");
                    None
                }
                Token::Eof => {
                    return None;
                }
                Token::Ident(_) => self.assign(start),
                _ => {
                    let expr = self.expr(start)?;
                    let stmt = Stmt::ExprStmt { expr };
                    Some(Box::new(stmt))
                }
            };

            if !self.expect(Token::Eof, "invalid syntax!") {
                return None;
            }

            stmt
        }

        fn assign(&mut self, start: Token) -> Option<Box<Stmt>> {
            if !self.check(Token::Eq) {
                let expr = self.expr(start)?;
                return Some(Box::new(Stmt::ExprStmt { expr }));
            }
            self.eat();

            let expr_start = self.next.take().unwrap();
            self.eat();

            let name = if let Token::Ident(name) = start {
                name
            } else {
                print_err("error");
                return None;
            };

            let expr = self.expr(expr_start)?;
            Some(Box::new(Stmt::AssignStmt { name, expr }))
        }

        fn fun(&mut self) -> Option<Box<Stmt>> {
            self.count = 0;

            let name = if let Some(Token::Ident(name)) = self.next.take() {
                self.eat();
                name
            } else {
                return None;
            };

            if !self.expect(Token::LeftParen, "expect '('") {
                return None;
            }

            while let Some(Token::Ident(_)) = &self.next {
                let name = if let Some(Token::Ident(name)) = self.next.take() {
                    self.eat();
                    name
                } else {
                    return None;
                };
                self.args.insert(name, self.count);
                self.count += 1;
            }

            if !self.expect(Token::RightParen, "expect ')'") {
                return None;
            }

            if !self.expect(Token::Eq, "expect '='") {
                return None;
            }

            let start = self.next.take()?;
            self.eat();

            let body = self.expr(start)?;
            let stmt = Stmt::FunStmt { name, body };
            Some(Box::new(stmt))
        }

        fn expr(&mut self, start: Token) -> Option<Box<Expr>> {
            self.plus_sub(start)
        }

        fn plus_sub(&mut self, start: Token) -> Option<Box<Expr>> {
            let mut left = self.mult_div(start)?;

            while self.check(Token::Plus) || self.check(Token::Minus) {
                let op = match self.next.take()? {
                    Token::Plus => Binaryop::Plus,
                    Token::Minus => Binaryop::Sub,
                    _ => Binaryop::Plus, // impassiable
                };
                self.eat();
                let start = self.next.take()?;
                self.eat();
                let right = self.mult_div(start)?;
                left = Box::new(Expr::Binary { left, op, right })
            }

            Some(left)
        }

        fn mult_div(&mut self, start: Token) -> Option<Box<Expr>> {
            let mut left = self.square(start)?;

            while self.check(Token::Star) || self.check(Token::Slash) {
                let op = match self.next.take()? {
                    Token::Star => Binaryop::Mult,
                    Token::Slash => Binaryop::Div,
                    _ => Binaryop::Mult, // impassiable
                };
                self.eat();
                let start = self.next.take()?;
                self.eat();
                let right = self.square(start)?;
                left = Box::new(Expr::Binary { left, op, right })
            }

            Some(left)
        }

        fn square(&mut self, start: Token) -> Option<Box<Expr>> {
            let mut left = self.minus(start)?;

            while self.check(Token::Square) {
                let op = Binaryop::Square;
                self.eat();
                let start = self.next.take()?;
                self.eat();
                let right = self.minus(start)?;
                left = Box::new(Expr::Binary { left, op, right })
            }

            Some(left)
        }

        fn minus(&mut self, start: Token) -> Option<Box<Expr>> {
            if let Token::Minus = start {
                let op = Unaryop::Sub;
                let start = self.next.take()?;
                self.eat();
                let operand = self.minus(start)?;
                return Some(Box::new(Expr::Unary { op, operand }));
            }

            self.factorial(start)
        }

        fn factorial(&mut self, start: Token) -> Option<Box<Expr>> {
            let mut operand = self.call(start)?;

            if self.check(Token::Bang) {
                let op = Unaryop::Ftl;
                self.eat();
                operand = Box::new(Expr::Unary { op, operand });
            }
            Some(operand)
        }

        fn call(&mut self, start: Token) -> Option<Box<Expr>> {
            if let (Token::Ident(_), Some(Token::LeftParen)) = (&start, &self.next) {
                let mut values = Vec::new();
                let name = if let Token::Ident(name) = start {
                    name
                } else {
                    return None;
                };

                self.eat();

                while let Some(Token::Number(_)) = &self.next {
                    let num = if let Some(Token::Number(num)) = self.next.take() {
                        self.eat();
                        num
                    } else {
                        return None;
                    };
                    values.push(num);
                }

                if !self.expect(Token::RightParen, "expect ')'") {
                    return None;
                }

                return Some(Box::new(Expr::Fun { name, values }));
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
                    value: Valuable::Float(num),
                })),
                Token::LeftParen => {
                    let start = self.next.take()?;
                    self.eat();
                    let v = self.expr(start)?;
                    if !self.expect(Token::RightParen, "expect ')'") {
                        return None;
                    }
                    Some(Box::new(Expr::Group { body: v }))
                }
                _ => {
                    print_err("invalid syntax");
                    None
                }
            }
        }

        fn eat(&mut self) {
            self.next = self.tokens.next();
        }

        fn check(&mut self, token: Token) -> bool {
            if let Some(t) = &self.next {
                if *t == token {
                    return true;
                }
                false
            } else {
                false
            }
        }

        fn expect(&mut self, token: Token, err: &str) -> bool {
            if self.check(token) {
                self.eat();
                return true;
            }

            utils::print_err(err);
            false
        }
    }
}

pub mod calculator {
    use std::collections::HashMap;

    use crate::{
        ast::{Binaryop, Expr, Stmt, Unaryop, Valuable},
        lexer::Scanner,
        parser::Parser,
        utils,
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
                ("ln", Box::new(|x| f64::ln(x)) as Box<_>),
                ("lg", Box::new(|x| f64::log10(x)) as Box<_>),
            ]);
            Env {
                functions: HashMap::new(),
                locals: Vec::new(),
                builtin,
                global: HashMap::new(),
            }
        }

        pub fn run(&mut self, s: &str) -> Option<f64> {
            let mut lexer = Scanner::new(s.chars());
            let tokens = lexer.scan();
            let mut parser = Parser::new(tokens.into_iter());
            let ast = parser.parse()?;
            self.run_impl(ast)
        }

        fn run_impl(&mut self, stmt: Box<Stmt>) -> Option<f64> {
            match *stmt {
                Stmt::FunStmt { name, body } => {
                    self.functions.insert(name, body);
                    None
                }
                Stmt::ExprStmt { expr } => expr.value(self),
                Stmt::AssignStmt { name, expr } => {
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
                Self::Float(v) => Some(*v),
                Self::Arg(i) => env.locals.get(*i).copied(),
                Self::Var(name) => env.global.get(name).copied(),
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
                        Binaryop::Plus => lv + rv,
                        Binaryop::Sub => lv - rv,
                        Binaryop::Mult => lv * rv,
                        Binaryop::Div => lv / rv,
                        Binaryop::Square => lv.powf(rv),
                    };
                    Some(result)
                }
                Expr::Fun { name, values } => {
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
                        None
                    }
                }
                Expr::Unary { op, operand } => {
                    let value = operand.value(env)?;
                    let result = match op {
                        Unaryop::Sub => -value,
                        Unaryop::Ftl => utils::factorial(value as u32),
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

    pub fn print_err(err: &str) {
        println!("{}", err);
    }

    pub fn factorial(num: u32) -> f64 {
        let mut result: u32 = 1;
        for i in 2..=num {
            result = result.wrapping_mul(i);
        }
        result as f64
    }
}
