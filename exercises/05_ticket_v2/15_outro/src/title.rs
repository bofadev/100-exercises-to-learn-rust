// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `TicketTitle` type,
//   enforcing that the title is not empty and is not longer than 50 characters.
//   Implement the traits required to make the tests pass too.

use thiserror::Error;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Error, Debug)]
pub enum TicketTitleError {
    #[error("The title cannot be empty")]
    ZeroLength,
    #[error("The title cannot be longer than 50 bytes")]
    TooManyCharacters,
}

fn validate_title(value: String) -> Result<TicketTitle, TicketTitleError> {
    
    if value.len() == 0 {
        return Err(TicketTitleError::ZeroLength);
    }
    
    if value.graphemes(true).count() > 50 {
        return Err(TicketTitleError::TooManyCharacters);
    }

    Ok(TicketTitle(value))
}

#[derive(Debug, PartialEq, Clone)]
pub struct TicketTitle(String);

impl TryFrom<String> for TicketTitle {
    type Error = TicketTitleError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match validate_title(value) {
            Ok(result) => Ok(result),
            Err(error) => Err(error),
        }
    }
}

impl TryFrom<&str> for TicketTitle {
    type Error = TicketTitleError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match validate_title(value.to_string()) {
            Ok(result) => Ok(result),
            Err(error) => Err(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let title = TicketTitle::try_from("A title".to_string()).unwrap();
        assert_eq!(title.0, "A title");
    }

    #[test]
    fn test_try_from_empty_string() {
        let err = TicketTitle::try_from("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be empty");
    }

    #[test]
    fn test_try_from_long_string() {
        let title =
            "A title that's definitely longer than what should be allowed in a development ticket"
                .to_string();
        let err = TicketTitle::try_from(title).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be longer than 50 bytes");
    }

    #[test]
    fn test_try_from_str() {
        let title = TicketTitle::try_from("A title").unwrap();
        assert_eq!(title.0, "A title");
    }
}
