use std::io::{self, Write};

use calculator::Env;

fn main() {
    let mut env = Env::new();
    loop {
        // prompt
        print!(">>> ");
        io::stdout().flush().expect("flush error");

        let mut line = String::with_capacity(8);
        io::stdin()
            .read_line(&mut line)
            .expect("fail to read input");

        // exit
        if line == "exit\n" || line == "exit" {
            break;
        }

        // run
        if let Some(v) = env.run(&line) {
            println!("{}", v);
        }
    }
}
