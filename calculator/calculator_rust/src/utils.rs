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
