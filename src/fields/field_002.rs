use crate::error::ReaderError;
use crate::error::ReaderError::InvalidLength;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// # 5.2 Record Type
///
/// Definition/Description: The “Record Type” field content
/// indicates whether the record data are “standard,” i.e.,
/// suitable for universal application, or “tailored,” i.e.
/// included on the master file for a single user’s specific
/// purpose (Section 1.2 of this Specification refers).
///
/// Source/Content: The field contains the letter “S” when the
/// field data are “standard” and the letter “T” when they are
/// “tailored.”
#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub enum RecordType {
    #[default]
    Standard,
    Tailored,
    /// For stupid Navigraph.
    Other(char),
}

impl Display for RecordType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RecordType::Standard => "S".to_string(),
                RecordType::Tailored => "T".to_string(),
                RecordType::Other(c) => c.to_ascii_uppercase().to_string(),
            }
        )
    }
}

impl FromStr for RecordType {
    type Err = ReaderError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(InvalidLength(1, s.len()));
        }

        let c = s.chars().next().unwrap().to_ascii_uppercase();

        match c {
            'S' => Ok(RecordType::Standard),
            'T' => Ok(RecordType::Tailored),
            _ => Ok(RecordType::Other(c)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recd_type_display() {
        assert_eq!(RecordType::Standard.to_string(), "S");
        assert_eq!(RecordType::Tailored.to_string(), "T");
        assert_eq!(RecordType::Other('c').to_string(), "C");
    }

    #[test]
    fn test_recd_type_from_str() {
        assert_eq!("S".parse(), Ok(RecordType::Standard));
        assert_eq!("T".parse(), Ok(RecordType::Tailored));
        assert_eq!("C".parse(), Ok(RecordType::Other('C')));
    }

    #[test]
    fn test_recd_type_from_invalid_input() {
        assert_eq!("".parse::<RecordType>(), Err(InvalidLength(1, 0)));
        assert_eq!("SS".parse::<RecordType>(), Err(InvalidLength(1, 2)));
    }
}
