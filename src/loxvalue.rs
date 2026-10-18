use std::fmt::{Display, Write};

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum LoxValue<'a> {
    Nil,
    String(&'a str),
    Number(f64),
    Boolean(bool),
}

impl Display for LoxValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Self::Nil => f.write_str("null"),
            &Self::String(s) => f.write_str(s),
            &Self::Boolean(tf) => write!(f, "{tf}"),
            &Self::Number(n) => {
                if n.fract() < f64::EPSILON {
                    write!(f, "{}.0", n)
                } else {
                    write!(f, "{}", n)
                }
            }
        }
    }
}
