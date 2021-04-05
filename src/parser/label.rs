use std::collections::HashMap;
use std::ops::Deref;

#[derive(Debug, PartialEq)]
pub struct Label(HashMap<String, String>);

impl Label {
    pub fn new() -> Self {
        Label(HashMap::new())
    }
    pub fn from_map(h: HashMap<String, String>) -> Self {
        Label(h)
    }
}

impl From<Vec<(&str, String)>> for Label {
    fn from(input: Vec<(&str, String)>) -> Self {
        let mut map = HashMap::with_capacity(input.len());

        for (key, value) in input {
            map.insert(key.to_owned(), value);
        }

        Label(map)
    }
}

impl Deref for Label {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
