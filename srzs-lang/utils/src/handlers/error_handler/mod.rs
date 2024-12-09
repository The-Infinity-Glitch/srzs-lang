use crate::{handlers, types};

#[derive(Debug, Clone)]
pub enum ErrorCode {
    E001, // Invalid token
    E002, // Unexpected token
}

#[derive(Debug, Clone)]
pub struct BaseError {
    message: handlers::message_handler::Message,
    code: ErrorCode,
}

impl BaseError {
    pub fn new(message: handlers::message_handler::Message, code: ErrorCode) -> Self {
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

impl Error {
    pub fn expected_error(expected: &str, found: &types::tokens::Token) -> Error {
        // Create a new message for the error
        let message = handlers::message_handler::Message::new(
            format!("expected {} but found {}.", expected, found.token_value).as_str(),
            handlers::message_handler::Issuer::Lexer,
        );

        // Create a base error with the code [E002] <- Unexpected token
        let base_error = BaseError::new(message, ErrorCode::E002);

        // Create a script error
        let script_error = ScriptError::new(base_error, found.position.line, found.position.column);

        // Create the actual error
        Error::ScriptError(script_error)
    }

    pub fn expected_or_error(
        expected: &str,
        expected_type: &types::tokens::TokenType,
        found: &types::tokens::Token,
    ) -> Option<Error> {
        if !found.token_type.eq(expected_type) {
            // Create a new message for the error
            let message = handlers::message_handler::Message::new(
                format!("expected {} but found {}.", expected, found.token_value).as_str(),
                handlers::message_handler::Issuer::Lexer,
            );

            // Create a base error with the code [E002] <- Unexpected token
            let base_error = BaseError::new(message, ErrorCode::E002);

            // Create a script error
            let script_error =
                ScriptError::new(base_error, found.position.line, found.position.column);

            // Create the actual error
            Some(Error::ScriptError(script_error))
        } else {
            None
        }
    }
}
