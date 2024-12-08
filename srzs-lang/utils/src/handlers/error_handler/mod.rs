use crate::{handlers::message_handler, types};

#[derive(Debug, Clone)]
pub enum ErrorCode {
    E001, // Non existent token
}

#[derive(Debug, Clone)]
pub struct BaseError {
    message: message_handler::Message,
    code: ErrorCode,
}

impl BaseError {
    pub fn new(message: message_handler::Message, code: ErrorCode) -> Self {
        Self { message, code }
    }
}

#[derive(Debug, Clone)]
pub struct ScriptError {
    base: BaseError,
    position: types::others::Position,
}

impl ScriptError {
    pub fn new(base: BaseError, line: usize, column: usize) -> Self {
        Self {
            base,
            position: types::others::Position::new(line, column),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    Error(BaseError),
    ScriptError(ScriptError),
}
