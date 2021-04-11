use std::collections::HashMap;
use std::str::FromStr;

/// List of labels for a [SampleData](super::SampleData). See examples at [try_read_sample](super::try_read_sample) for more details.
#[derive(Debug, PartialEq)]
pub struct LabelList(HashMap<String, String>);

impl LabelList {
    pub fn new() -> Self {
        LabelList(HashMap::new())
    }

    pub fn from_map(h: HashMap<String, String>) -> Self {
        LabelList(h)
    }

    /// tries to read a label as string.
    pub fn get_string(&self, key: &str) -> Option<&String> {
        self.0.get(key)
    }

    /// tries to read a label as a [`f64`], returning [`None`] if the label doesn't exist or cannot be represented as a [`f64`]
    pub fn get_number(&self, key: &str) -> Option<f64> {
        self.0.get(key).and_then(|s| match s.as_str() {
            "+Inf" => Some(f64::INFINITY),
            "-Inf" => Some(f64::NEG_INFINITY),
            s => f64::from_str(&s).ok(),
        })
    }
}

impl Default for LabelList {
    fn default() -> Self {
        Self::new()
    }
}
impl From<Vec<(&str, String)>> for LabelList {
    fn from(input: Vec<(&str, String)>) -> Self {
        let mut map = HashMap::with_capacity(input.len());

        for (key, value) in input {
            map.insert(key.to_string(), value);
        }

        LabelList(map)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::label::LabelList;
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

        let l = LabelList::from_map(m);

        assert_eq!(l.get_number("test1"), Some(0.0015));
        assert_eq!(l.get_number("test2"), Some(-17560473.0));
        assert_eq!(l.get_number("test3"), Some(f64::INFINITY));
        assert_eq!(l.get_number("test4"), Some(f64::NEG_INFINITY));
        assert_eq!(l.get_number("test5"), None);
        assert_eq!(l.get_number("test6"), None);
    }
}
