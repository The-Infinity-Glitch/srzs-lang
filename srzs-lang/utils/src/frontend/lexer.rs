use crate::handlers::{error_handler, message_handler};
use crate::types::*;
use logos::Logos;

/// Update the line count and the char index
pub fn new_line_callback(lex: &mut logos::Lexer<tokens::TokenType>) {
    lex.extras.0 += 1;
    lex.extras.1 = lex.span().end;
}

/// Compute the line and column position for the current word
pub fn word_callback(lex: &mut logos::Lexer<tokens::TokenType>) {
    let line = lex.extras.0;
    let column = lex.span().start - lex.extras.1;
}

#[derive(Debug, Clone)]
pub struct Lexer {
    pub tokens: Vec<tokens::Token>,
    pub errors: Vec<error_handler::Error>,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            tokens: vec![],
            errors: vec![],
        }
    }

    /// Lex the input and return a vector of tokens and errors
    pub fn lex(&mut self, input: &str) {
        let mut lex = tokens::TokenType::lexer(input);
        let mut token_position = others::Position { line: 0, column: 0 };

        while let Some(token_type) = lex.next() {
            match token_type {
                Ok(token) => {
                    // Update the line in token_position if the token is a new line, or just update the column
                    match token {
                        tokens::TokenType::NewLine => {
                            token_position.line += 1;
                            token_position.column = 0
                        }
                        _ => token_position.column = lex.span().start - lex.extras.1,
                    }

                    // A new token
                    let tk = tokens::Token::new(
                        token.clone(),
                        lex.slice(),
                        token_position.line,
                        token_position.column,
                    );

                    // Insert the new token in tokens field of the Lexer
                    self.tokens.insert(self.tokens.len(), tk);
                }
                Err(_) => {
                    // Create a new message for the error
                    let message = message_handler::Message::new(
                        format!("{:?} -> non existent token.", lex.slice()).as_str(),
                        message_handler::Issuer::Lexer,
                    );

                    // Create a base error with the code [E001] <- Non existent token
                    let base_error =
                        error_handler::BaseError::new(message, error_handler::ErrorCode::E001);

                    // Create a script error
                    let script_error = error_handler::ScriptError::new(
                        base_error,
                        lex.extras.0,
                        lex.span().start - lex.extras.1,
                    );

                    // Create the actual error
                    let error = error_handler::Error::ScriptError(script_error);

                    // Insert the error in the error vector
                    self.errors.insert(self.errors.len(), error);
                }
            }
        }

        // Insert a EOF in the end of the token vector
        self.tokens.insert(
            self.tokens.len(),
            tokens::Token::new(
                tokens::TokenType::Eof,
                "EOF",
                token_position.line,
                token_position.column,
            ),
        );

        self.lex_trim_result();
    }

    /// Remove unused spaces and new lines
    fn lex_trim_result(&mut self) {
        let mut result: Vec<tokens::Token> = Vec::new();

        for token in &self.tokens {
            match token.token_type {
                tokens::TokenType::NewLine => continue,
                tokens::TokenType::Space => continue,
                _ => result.insert(result.len(), token.to_owned()),
            }
        }

        self.tokens = result
    }
}
