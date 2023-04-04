use std::fmt::{self, Display};

#[derive(PartialEq, PartialOrd, Clone, Debug)]
#[allow(unused)]
pub enum OneMore {
    One(f64),
    More(Vec<f64>),
}

impl OneMore { // bad, but i can do nothing
    pub fn one(&self) -> Option<f64> {
        match self {
            OneMore::One(v) => Some(*v),
            OneMore::More(_) => None,
        }
    }
    
    pub fn more(&self) -> Option<&[f64]> {
        match self {
            OneMore::One(_) => None,
            OneMore::More(v) => Some(v),
        }
    }
}

impl Display for OneMore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OneMore::One(v) => write!(f, "{}", v),
            OneMore::More(v) => write!(f, "{:?}", v),
        }
    }
}