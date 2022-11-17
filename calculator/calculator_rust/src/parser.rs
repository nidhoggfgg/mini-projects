use std::collections::HashMap;
use std::mem::discriminant;

use crate::ast::{BinaryOp, Expr, Stmt, UnaryOp, Valuable};
use crate::lexer::Token;
use crate::utils::print_err;

pub struct Parser<T: Iterator<Item = Token>> {
    tokens: T,
    next: Option<Token>,
    args: HashMap<u64, usize>,
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
            Token::Unknown => {
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

        let idx = if let Token::Ident(idx) = start {
            idx
        } else {
            print_err!("expect a name but get {:?}, this is a bug!", start);
            return None;
        };

        Some(Box::new(Stmt::Assign { idx, expr }))
    }

    fn fun(&mut self) -> Option<Box<Stmt>> {
        if self.is_at_end() {
            print_err!("expect a name after 'fun'");
            return None;
        }

        let idx = if let Some(Token::Ident(idx)) = self.next() {
            idx
        } else {
            print_err!("expect a name after 'fun'");
            return None;
        };

        if !self.expect(Token::LeftParen) {
            print_err!("expect '(' after '{}'", idx);
            return None;
        }

        let mut count = 0;
        while self.check(Token::Ident(0)) {
            let idx = if let Some(Token::Ident(idx)) = self.next.take() {
                self.eat();
                idx
            } else {
                print_err!("expect a name, this is a bug!");
                return None;
            };

            if count == usize::MAX {
                print_err!("to many args");
                return None;
            }

            self.args.insert(idx, count);
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
        let stmt = Stmt::Fun { idx, body };

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
                _ => BinaryOp::Plus, // impossible
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
                _ => BinaryOp::Mult, // impossible
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
            let idx = if let Token::Ident(idx) = start {
                idx
            } else {
                print_err!("expect a name to call a function");
                return None;
            };
            self.eat();

            let mut values = Vec::new();

            while !self.check(Token::RightParen) {
                let t = self.next()?;
                let expr = self.expr(t)?;
                values.push(expr);
            }

            if !self.expect(Token::RightParen) {
                print_err!("missing ')'");
                return None;
            }

            return Some(Box::new(Expr::Fun { idx, args: values }));
        }

        self.primary(start)
    }

    fn primary(&mut self, start: Token) -> Option<Box<Expr>> {
        match start {
            Token::Ident(idx) => {
                if let Some(i) = self.args.get(&idx) {
                    Some(Box::new(Expr::Literal {
                        value: Valuable::Arg(*i),
                    }))
                } else {
                    Some(Box::new(Expr::Literal {
                        value: Valuable::Var(idx),
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
