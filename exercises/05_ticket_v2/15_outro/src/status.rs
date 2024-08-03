// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `Status` enum.
//  The parsing should be case-insensitive.

use thiserror::Error;

#[derive(Error, Debug)]
#[error("'{invalid_status}' is not a valid status")]
pub struct StatusError {
    invalid_status: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

fn parse_status(value: String) -> Result<Status, ()> {

    let lower_case = value.to_lowercase();

    if lower_case == "todo" {
        return Ok(Status::ToDo)
    }

    if lower_case == "inprogress" {
        return Ok(Status::InProgress)
    }

    if lower_case == "done" {
        return Ok(Status::Done)
    }

    Err(())
}

impl TryFrom<String> for Status {
    type Error = StatusError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match parse_status(value.clone()) {
            Ok(result) => Ok(result),
            Err(_) => Err(StatusError{invalid_status: value}),
        }
    }
}

impl TryFrom<&str> for Status {
    type Error = StatusError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match parse_status(value.to_string()) {
            Ok(result) => Ok(result),
            Err(_) => Err(StatusError{invalid_status: value.to_string()}),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("ToDO").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done").unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_invalid() {
        let status = Status::try_from("Invalid");
        assert!(status.is_err());
    }
}
