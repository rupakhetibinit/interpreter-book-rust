use core::fmt;

#[derive(Debug, Clone)]
pub enum Object {
    Int(i64),
    Bool(bool),
    Null,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Int(i) => write!(f, "{}", i),
            Object::Bool(b) => write!(f, "{}", b),
            Object::Null => write!(f, "nil"),
        }
    }
}
