#[derive(Debug, PartialEq)]
pub enum Type {
    TYPE,
    HELP,
    UNKNOWN,
}

impl From<&str> for Type {
    fn from(s: &str) -> Self {
        match s {
            "TYPE" => Type::TYPE,
            "HELP" => Type::HELP,
            _ => Type::UNKNOWN,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Comment {
    pub metric: String,
    pub ctype: Type,
    pub description: String,
}

impl Comment {
    pub fn new(metric: String, ctype: Type, desc: String) -> Self {
        Comment {
            metric,
            ctype,
            description: desc,
        }
    }
}
