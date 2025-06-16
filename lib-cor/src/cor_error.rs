use std::fmt::Debug;
use thiserror::Error;

// Общее определение ошибки
#[derive(Error, Debug)]
pub struct CorError<E, F> {
    pub msg: String,
    pub code: E,
    pub field: F,
    pub error: Option<std::io::Error>,
}
