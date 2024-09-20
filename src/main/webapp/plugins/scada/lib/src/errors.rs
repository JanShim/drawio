// use thiserror::Error;
use std::fmt::Display;

use wasm_bindgen::JsValue;


#[derive(Debug, Clone)]
pub enum FetchError {
    // #[error("{0}")]
    RequestError(String),
    // #[error("{0}")]
    SerdeError(String),    
    InsertModelError(String),
    ParseXmlError(String),
}

impl Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            FetchError::RequestError(err) => err,
            FetchError::SerdeError(err) => err,
            FetchError::InsertModelError(err) => err,
            FetchError::ParseXmlError(err) => err,
        };
        write!(f, "{msg}")
    }
}


#[derive(Debug, Clone)]
pub enum CellStateError  {
    // NoCell,
    NoMeta,
    // MultyStateStateIndexError{len: usize, index: usize},
    NotMultystate,
    NotWidget,
}

impl Display for CellStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            // CellStateError::NoCell => "no cell error".to_owned(),
            CellStateError::NoMeta => "no meta data error".to_owned(),
            // CellStateError::MultyStateStateIndexError{len, index} => format!("vec index: {index} > length: {len}"),
            CellStateError::NotMultystate => "not multistate cell".to_owned(),
            CellStateError::NotWidget => "not widget cell".to_owned(),
        };
        write!(f, "{msg}")
    }
}

impl Into<JsValue> for CellStateError {
    fn into(self) -> JsValue {
        log::error!("{}", self.to_string());
        JsValue::from(self.to_string())
    }
}
