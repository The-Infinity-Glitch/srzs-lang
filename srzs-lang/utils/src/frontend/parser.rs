use crate::{handlers, types};
use std::iter::Peekable;

#[derive(Debug, Clone)]
pub struct Parser {
    tokens: Peekable<std::vec::IntoIter<types::tokens::Token>>,
    current_token: types::tokens::Token,
    pub errors: Vec<handlers::error_handler::Error>,
    pub ast: types::parse_nodes::Statement,
}

impl Parser {
    pub fn new(input: Vec<types::tokens::Token>) -> Self {
        let mut input_iter = input.into_iter().peekable();

        Self {
            tokens: input_iter.clone(),
            current_token: input_iter.next().unwrap(),
            errors: vec![],
            ast: types::parse_nodes::Statement::Module {
                start: types::others::Position::new(0, 0),
                body: Box::new(Vec::new()),
            },
        }
    }

    fn push_statement(&mut self, statement: types::parse_nodes::Statement) {
        match &mut self.ast {
            types::parse_nodes::Statement::Module { body, .. } => {
                body.push(statement);
            }
            _ => {}
        }
    }

    /// Return the current token
    fn current(&self) -> &types::tokens::Token {
        &self.current_token
    }

    /// Return the current token type
    fn current_type(&mut self) -> &types::tokens::TokenType {
        &self.current().token_type
    }

    /// If the current token it's the expected token return true, else return false
    fn peek_expect(&mut self, expected: &types::tokens::TokenType) -> bool {
        self.current_type().eq(expected)
    }

    /// Advance the index in token vector
    fn advance(&mut self) {
        match self.current().token_type {
            types::tokens::TokenType::Eof => {}
            _ => self.current_token = self.tokens.next().unwrap(),
        }
    }

    /// Parse and return a function statement -> fn function_name(arg1: type, arg2: type) -> type {function content}
    fn parse_function_satement(&mut self) -> Option<types::parse_nodes::Statement> {
        // fn <- token
        let fn_token = self.current().to_owned();
        self.advance();

        // "name" <- function name (identifier)
        let name: Option<String> = match handlers::error_handler::Error::expected_or_error(
            "a identifier",
            &types::tokens::TokenType::Identifier,
            self.current(),
        ) {
            Some(error) => {
                self.errors.insert(self.errors.len(), error);
                None
            }
            None => Some(self.current().token_value.to_owned()),
        };

        if name.is_none() {
            return None;
        }

        self.advance();

        match handlers::error_handler::Error::expected_or_error(
            "(",
            &types::tokens::TokenType::LParen,
            self.current(),
        ) {
            Some(error) => {
                self.errors.insert(self.errors.len(), error);
                return None;
            }
            None => {}
        };
    }

    /// While doesn't reaches EOF parse tokens
    pub fn parse_tokens(&mut self) {
        while !self.peek_expect(&types::tokens::TokenType::Eof) {
            // Parse and return a statement
            let ast_node: Option<types::parse_nodes::Statement> = match &self.current_type() {
                types::tokens::TokenType::KwFn => self.parse_function_satement(),
                _ => {
                    handlers::error_handler::Error::expected_error("a statement", self.current());
                    break;
                }
            };

            // If a parse function find a error, it will return nothing
            if ast_node.is_some() {
                self.push_statement(ast_node.unwrap());
            } else {
                break;
            }
        }
    }
}
