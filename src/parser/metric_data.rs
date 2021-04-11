use crate::parser::label::Label;

#[derive(Debug, PartialEq)]
pub struct MetricData<'a> {
    name: &'a str,
    label: Label,
    value: f64,
    timestamp: Option<i64>,
}

impl<'a> MetricData<'a> {
    pub fn new(name: &'a str, label: Label, value: f64, timestamp: Option<i64>) -> Self {
        MetricData {
            name,
            label,
            value,
            timestamp,
        }
    }
}
