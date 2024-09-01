// use thiserror::Error;
use std::fmt::Display;

// #[derive(Error, Debug)]
// #[error("{msg}")]
// struct RequestError {
//     source: Option<anyhow::Error>,
//     msg: String
// }

// #[derive(Error, Debug)]
// #[error("{msg}")]
// struct DeserializeError {
//     source: Option<anyhow::Error>,
//     msg: String
// }

// #[derive(Clone, Debug, PartialEq)]
// enum Error {
//     RequestError,
//     DeserializeError,
//     // etc.
// }



#[derive(Debug, Clone)]
pub enum FetchError {
    // #[error("{0}")]
    RequestError(String),
    // #[error("{0}")]
    SerdeError(String),    
    InsertModelError(String),
}

impl Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            FetchError::RequestError(err) => err,
            FetchError::SerdeError(err) => err,
            FetchError::InsertModelError(err) => err,
        };
        write!(f, "{msg}")
    }
}
