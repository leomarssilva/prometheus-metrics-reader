use crate::parser::label::LabelList;

/// Represents a parsed metric sample. See examples at [try_read_sample](super::try_read_sample) for more details.
#[derive(Debug, PartialEq)]
pub struct SampleData {
    pub metric_name: String,
    pub labels: LabelList,
    pub value: f64,
    pub timestamp: Option<i64>,
}

impl SampleData {
    pub fn new(metric_name: String, labels: LabelList, value: f64, timestamp: Option<i64>) -> Self {
        SampleData {
            metric_name,
            labels,
            value,
            timestamp,
        }
    }
}
