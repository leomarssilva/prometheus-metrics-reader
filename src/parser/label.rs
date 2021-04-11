use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Label(HashMap<String, String>);

impl Label {
    pub fn new() -> Self {
        Label(HashMap::new())
    }
    pub fn from_map(h: HashMap<String, String>) -> Self {
        Label(h)
    }
    pub fn get_string(&self, key: &str) -> Option<&String> {
        self.0.get(key)
    }
    pub fn get_float(&self, key: &str) -> Option<f64> {
        self.0.get(key).and_then(|s| match s.as_str() {
            "+Inf" => Some(f64::INFINITY),
            "-Inf" => Some(f64::NEG_INFINITY),
            s => f64::from_str(&s).ok(),
        })
    }
}

impl From<Vec<(&str, String)>> for Label {
    fn from(input: Vec<(&str, String)>) -> Self {
        let mut map = HashMap::with_capacity(input.len());

        for (key, value) in input {
            map.insert(key.to_string(), value);
        }

        Label(map)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::label::Label;
    use std::collections::HashMap;

    #[test]
    fn test_get_float() {
        let mut m = HashMap::new();
        m.insert("test1".into(), "1.5e-03".into());
        m.insert("test2".into(), "-1.7560473e+07".into());
        m.insert("test3".into(), "+Inf".into());
        m.insert("test4".into(), "-Inf".into());
        m.insert("test5".into(), "alfa".into());
        m.insert("test6".into(), "".into());

        let l = Label::from_map(m);

        assert_eq!(l.get_float("test1"), Some(0.0015));
        assert_eq!(l.get_float("test2"), Some(-17560473.0));
        assert_eq!(l.get_float("test3"), Some(f64::INFINITY));
        assert_eq!(l.get_float("test4"), Some(f64::NEG_INFINITY));
        assert_eq!(l.get_float("test5"), None);
        assert_eq!(l.get_float("test6"), None);
    }
}
