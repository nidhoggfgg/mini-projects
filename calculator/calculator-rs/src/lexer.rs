use std::collections::HashMap;

use crate::utils::{self, is_identifier_continue};

#[derive(Clone, Debug)]
pub(crate) enum Token {
    LeftParen,
    RightParen,
    Plus,
    Minus,
    Star,
    Slash,
    Bang,
    Square,
    Comma,
    Eq,
    Percent,
    Fun,
    Number(f64),
    Ident(u64),
    Unknown,
    Eof,
}

pub(crate) struct Scanner<T: Iterator<Item = char>> {
    source: T,
    next: Option<char>,
    kw: HashMap<&'static str, Token>,
    namespace: HashMap<u64, String>,
}

impl<T: Iterator<Item = char>> Scanner<T> {
    pub(crate) fn new(source: T) -> Self {
        let mut scanner = Scanner {
            source,
            next: None,
            kw: HashMap::from([("fun", Token::Fun)]),
            namespace: HashMap::new(),
        };
        scanner.eat();
        scanner
    }

    pub(crate) fn scan(&mut self) -> Vec<Token> {
        let mut tokens = Vec::with_capacity(16);
        while let Some(t) = self.scan_token() {
            tokens.push(t);
        }
        tokens.push(Token::Eof);
        tokens
    }

    pub(crate) fn pop_namespace(&mut self) -> HashMap<u64, String> {
        std::mem::take(&mut self.namespace)
    }

    fn scan_token(&mut self) -> Option<Token> {
        self.skip_space();
        self.skip_comment();
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
            ',' => Token::Comma,
            '%' => Token::Percent,
            '0'..='9' => self.number(c),
            _ => Token::Unknown,
        };

        Some(token)
    }

    fn ident_or_kw(&mut self, start: char) -> Token {
        let mut lexeme = String::with_capacity(4);
        lexeme.push(start);

        while let Some(c) = self.next {
            if is_identifier_continue(c) {
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

        let hash = utils::hash_it(&lexeme);
        self.namespace.insert(hash, lexeme);
        Token::Ident(hash)
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

    fn skip_comment(&mut self) {
        if Some('#') == self.next {
            while let Some(c) = self.next {
                match c {
                    '\n' => {
                        self.eat();
                        break;
                    }
                    _ => self.eat(),
                }
            }
        }
    }

    fn eat(&mut self) {
        self.next = self.source.next();
    }
}
