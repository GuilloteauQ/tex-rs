/// File defining the structure of an expression
use std::fmt;

#[derive(Debug)]
pub enum Expression {
    Str(String),
    Int(i32),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Expression::Str(s) => write!(f, "{}", s),
            &Expression::Int(i) => write!(f, "{}", i),
            _ => panic!("oops"),
        }
    }
}
