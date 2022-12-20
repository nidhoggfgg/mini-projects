use std::env;

use convert::convert;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        help(&args[0]);
        return;
    }

    let num: f64 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            help(&args[0]);
            return;
        },
    };

    match convert(num) {
        Some(s) => println!("{}", s),
        None => println!("转换失败")
    };

}

fn help(name: &str) {
    println!("用法: {} <数字>", name);
}
