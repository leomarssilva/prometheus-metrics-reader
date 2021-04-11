/// If it is a `type` or `help` comment.
/// More details: [comments, help text, and type information](https://prometheus.io/docs/instrumenting/exposition_formats/#comments-help-text-and-type-information).
#[derive(Debug, PartialEq)]
pub enum CommentType {
    TYPE,
    HELP,
    UNKNOWN,
}

/// Represents parsed `type` or `help` comments for each `metric`. See examples at [try_read_comment](super::try_read_comment) for more details.
#[derive(Debug, PartialEq)]
pub struct Comment {
    pub metric: String,
    pub comment_type: CommentType,
    pub description: String,
}

impl From<&str> for CommentType {
    fn from(s: &str) -> Self {
        match s {
            "TYPE" => CommentType::TYPE,
            "HELP" => CommentType::HELP,
            _ => CommentType::UNKNOWN,
        }
    }
}

impl Comment {
    pub fn new(metric: String, comment_type: CommentType, desc: String) -> Self {
        Comment {
            metric,
            comment_type,
            description: desc,
        }
    }
}
