use std::fmt;

#[derive(Debug)]
pub enum Errcode{
    // ArgumentInvalid(&'static str),
}


impl Errcode{}

impl fmt::Display for Errcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self{
            _ => write!(f, "{:?}", self) // For now all variants are treated the same way
                                         // with this catch-all statement
        }
    }
}
