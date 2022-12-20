use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub(crate) fn is_identifier_start(c: char) -> bool {
    c == '_' || c.is_alphabetic()
}

pub(crate) fn is_identifier_continue(c: char) -> bool {
    is_identifier_start(c) || is_number(c)
}

pub(crate) fn is_number(c: char) -> bool {
    ('0'..='9').contains(&c)
}

macro_rules! print_err {
    ($($arg:tt)*) => {
        println!("{}", format_args!($($arg)*));
    };
}

pub(crate) use print_err;

pub(crate) fn factorial(num: u32) -> f64 {
    let mut result: u32 = 1;
    for i in 2..=num {
        result = result.wrapping_mul(i);
    }
    result as f64
}

pub(crate) fn hash_it<T>(v: &T) -> u64
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    v.hash(&mut hasher);
    hasher.finish()
}
