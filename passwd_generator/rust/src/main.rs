use clap::Parser;

use generator::gen_passwd;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(long = "auth", help = "the target for generate passwd")]
    auth: String,

    #[clap(
        long = "target",
        help = "the unique string, pls remember it and don't tell anybody!"
    )]
    target: String,

    #[clap(
        short = 'd',
        default_value_t = 16,
        help = "the number of digits of the passwd"
    )]
    digits: u32,

    #[clap(short = 'u', help = "add uppercase char to passwd [default: false]")]
    uppercase: bool,

    #[clap(short = 'n', help = "add number to passwd [default: false]")]
    number: bool,

    #[clap(
        short = 's',
        default_value = ".@_-:!",
        help = "the symbols used in passwd"
    )]
    symbols: String,
}

fn main() {
    let args = Cli::parse();
    let symbols: Vec<char> = args.symbols.chars().collect();
    let passwd = gen_passwd(
        &args.auth,
        &args.target,
        args.digits,
        args.uppercase,
        args.number,
        &symbols,
    );
    println!("{}", passwd);
}
