use std::{io::{self, Write}};

use calculator_rust::{lexer, parser, env::Env};


fn main() {
    let mut env = Env::new();
    loop {
        print!(">>> ");
        io::stdout().flush().expect("flush error");
        let mut line = String::with_capacity(8);
        io::stdin().read_line(&mut line).expect("fail to readline!");
        let mut scanner = lexer::Scanner::new(line.chars());
        let tokens = scanner.scan();
        let mut parser = parser::Parser::new(tokens.into_iter());
        let some = parser.parse();
        if let Some(stmt) = some {
            env.run(*stmt);
        }
    }
}
