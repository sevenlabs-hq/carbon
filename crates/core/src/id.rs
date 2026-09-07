//! Identifiers.

use std::fmt;

/// An identifier.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Id(String);

impl Id {
    /// Creates an identifier.
    ///
    /// # Errors
    ///
    /// Returns [`IdError::Blank`] if the value is empty or contains only whitespace.
    pub fn new(value: impl Into<String>) -> Result<Self, IdError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(IdError::Blank);
        }

        Ok(Self(value))
    }

    /// Returns the original value.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Id {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// An invalid identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum IdError {
    #[error("ID must contain a non-whitespace character")]
    Blank,
}

#[cfg(test)]
mod tests {
    use super::{Id, IdError};

    #[test]
    fn rejects_blank_values() {
        for value in ["", " ", "\t\r\n", "\u{00a0}\u{2003}\u{3000}"] {
            assert_eq!(Id::new(value), Err(IdError::Blank));
        }
    }

    #[test]
    fn preserves_non_blank_values() {
        for value in ["indexer", " \tIndexer \n", "a b/c:1", "é", "e\u{0301}"] {
            let id = Id::new(value).expect("non-blank value");
            assert_eq!(id.as_str(), value);
        }
    }

    #[test]
    fn accepts_owned_strings() {
        let id = Id::new(String::from("indexer")).expect("non-blank value");
        assert_eq!(id.as_str(), "indexer");
    }

    #[test]
    fn display_preserves_original_value() {
        let id = Id::new(" Indexer ").expect("non-blank value");
        assert_eq!(id.to_string(), " Indexer ");
    }
}
