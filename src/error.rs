use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ReaderError {
    /// 长度不对，(应为,实际)
    InvalidLength(usize, usize),
    /// 不支持的数据
    Unsupported(String),
}

impl Error for ReaderError {}

impl Display for ReaderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ReaderError::InvalidLength(expected, actual) => {
                write!(
                    f,
                    "Invalid input length. Expected {}, but got {}.",
                    expected, actual
                )
            }
            ReaderError::Unsupported(description) => {
                write!(f, "Unsupported input: {}", description)
            }
        }
    }
}

impl From<strum::ParseError> for ReaderError {
    fn from(e: strum::ParseError) -> Self {
        ReaderError::Unsupported(e.to_string())
    }
}
