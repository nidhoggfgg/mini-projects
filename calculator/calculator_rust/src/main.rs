use std::io::{self, Write};

use calculator_rust::calculator::Env;

fn main() {
    let mut env = Env::new();
    loop {
        print!(">>> ");
        io::stdout().flush().expect("flush error");
        let mut line = String::with_capacity(8);
        io::stdin().read_line(&mut line).expect("fail to readline!");
        if let Some(v) = env.run(&line) {
            println!("{}", v);
        }
    }
}
