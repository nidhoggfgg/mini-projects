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
        ExprStmt { expr: Box<Expr> },
    }

    #[derive(Debug, Clone)]
    pub enum Valuable {
        Float(f64),
        Arg(usize),
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
                },
                Token::Eof => {
                    return None;
                },
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

            self.call(start)
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
                    let i = self.args.get(&name)?;
                    Some(Box::new(Expr::Literal {
                        value: Valuable::Arg(*i),
                    }))
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
    };

    pub struct Env {
        funtions: HashMap<String, Box<Expr>>,
        values: Vec<f64>,
    }

    impl Default for Env {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Env {
        pub fn new() -> Self {
            Env {
                funtions: HashMap::new(),
                values: Vec::new(),
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
                    self.funtions.insert(name, body);
                    None
                }
                Stmt::ExprStmt { expr } => expr.value(&mut self.values, &self.funtions),
            }
        }
    }

    trait Value {
        fn value(&self, args: &mut Vec<f64>, functions: &HashMap<String, Box<Expr>>)
            -> Option<f64>;
    }

    impl Value for Valuable {
        fn value(
            &self,
            args: &mut Vec<f64>,
            _functions: &HashMap<String, Box<Expr>>,
        ) -> Option<f64> {
            match self {
                Self::Float(v) => Some(*v),
                Self::Arg(i) => args.get(*i).copied(),
            }
        }
    }

    impl Value for Expr {
        fn value(
            &self,
            args: &mut Vec<f64>,
            functions: &HashMap<String, Box<Expr>>,
        ) -> Option<f64> {
            match self {
                Expr::Literal { value } => value.value(args, functions),
                Expr::Binary { left, op, right } => {
                    let lv = left.value(args, functions)?;
                    let rv = right.value(args, functions)?;
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
                    args.clear();
                    for v in values {
                        args.push(*v);
                    }
                    let body = functions.get(name)?;
                    body.value(args, functions)
                }
                Expr::Unary { op, operand } => {
                    let value = operand.value(args, functions)?;
                    let result = match op {
                        Unaryop::Sub => -value,
                    };
                    Some(result)
                }
                Expr::Group { body } => body.value(args, functions),
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
}
