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
            current_token: input_iter.next().unwrap(),
            tokens: input_iter,
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

    fn get_type(
        &mut self,
    ) -> Result<types::built_in_types::BuiltInTypes, handlers::error_handler::Error> {
        match self.current_type() {
            types::tokens::TokenType::TypeNull => Ok(types::built_in_types::BuiltInTypes::Null),
            types::tokens::TokenType::TypeInt => Ok(types::built_in_types::BuiltInTypes::Int),
            types::tokens::TokenType::TypeFloat => Ok(types::built_in_types::BuiltInTypes::Float),
            types::tokens::TokenType::TypeBool => Ok(types::built_in_types::BuiltInTypes::Bool),
            types::tokens::TokenType::TypeChar => Ok(types::built_in_types::BuiltInTypes::Char),
            types::tokens::TokenType::TypeStr => Ok(types::built_in_types::BuiltInTypes::Str),
            types::tokens::TokenType::TypeVoid => Ok(types::built_in_types::BuiltInTypes::Void),
            types::tokens::TokenType::TypeAny => Ok(types::built_in_types::BuiltInTypes::Any),
            _ => Err(handlers::error_handler::Error::expected_error(
                "a type",
                &self.current(),
                handlers::message_handler::Issuer::Parser,
            )),
        }
    }

    /// Advance the index in token vector
    fn advance(&mut self) {
        match self.current().token_type {
            types::tokens::TokenType::Eof => {}
            _ => self.current_token = self.tokens.next().unwrap(),
        }
    }

    // Parse and return a let statement
    fn parse_let_statement(&mut self) -> Option<types::parse_nodes::Statement> {
        // "let" token <- used to get the start of the statement
        let let_token = self.current().to_owned();
        self.advance();

        // The let kind: mutable or immutable <- immutable default
        let mut kind: types::parse_nodes::LetDeclarationKind =
            types::parse_nodes::LetDeclarationKind::Immutable;

        match self.current().token_type {
            types::tokens::TokenType::KwMut => {
                kind = types::parse_nodes::LetDeclarationKind::Mutable;
                self.advance();
            }
            types::tokens::TokenType::Identifier => {}
            _ => {
                self.errors.insert(
                    self.errors.len(),
                    handlers::error_handler::Error::expected_error(
                        "\"mut\" or a identifier",
                        &self.current(),
                        handlers::message_handler::Issuer::Parser,
                    ),
                );
                return None;
            }
        };

        // The let statement name
        let name = match handlers::error_handler::Error::expected_or_error(
            "a identifier",
            &types::tokens::TokenType::Identifier,
            self.current(),
            handlers::message_handler::Issuer::Parser,
        ) {
            Ok(name) => name.token_value,
            Err(error) => {
                self.errors.insert(self.errors.len(), error);
                return None;
            }
        };

        self.advance();

        // ":" or ";" after the name
        match self.current().token_type {
            types::tokens::TokenType::Colon => {
                self.advance();
            }
            types::tokens::TokenType::SemiColon => {
                return Some(types::parse_nodes::Statement::LetDeclaration {
                    start: let_token.position,
                    name,
                    kind,
                    r#type: types::built_in_types::BuiltInTypes::Any,
                    value: None,
                });
            }
            _ => {
                self.errors.insert(
                    self.errors.len(),
                    handlers::error_handler::Error::expected_error(
                        "a explicity type or the end of let statement",
                        &self.current(),
                        handlers::message_handler::Issuer::Parser,
                    ),
                );
                return None;
            }
        };

        // The let statement type
        let r#type = match self.get_type() {
            Ok(r#type) => r#type,
            Err(error) => {
                self.errors.insert(self.errors.len(), error);
                return None;
            }
        };

        self.advance();

        // A expression attribution or the end of statement
        match self.current().token_type {
            types::tokens::TokenType::SemiColon => {
                return Some(types::parse_nodes::Statement::LetDeclaration {
                    start: let_token.position,
                    name,
                    kind,
                    r#type,
                    value: None,
                });
            }
            _ => {
                self.errors.insert(
                    self.errors.len(),
                    handlers::error_handler::Error::expected_error(
                        "a explicity type or the end of let statement",
                        &self.current(),
                        handlers::message_handler::Issuer::Parser,
                    ),
                );
                return None;
            }
        };
    }

    /// Parse and return a paremeter statement vector <- (param_name: type, ...)
    fn parse_params_statement(&mut self) -> Option<Vec<types::parse_nodes::FuncParam>> {
        // "("
        self.advance();

        // param_name: type
        let mut param: types::parse_nodes::FuncParam;
        let mut params: Vec<types::parse_nodes::FuncParam> = vec![];

        // If after the paremeter statement start, have a ")", so return nothing <- "(" -> ")"
        if self.peek_expect(&types::tokens::TokenType::RParen) {
            return Some(params);
        }

        // While doesn't reaches ")" or Eof
        while !self.peek_expect(&types::tokens::TokenType::RParen)
            || !self.peek_expect(&types::tokens::TokenType::Eof)
        {
            // "param_name": type
            let name = match handlers::error_handler::Error::expected_or_error(
                "a identifier",
                &types::tokens::TokenType::Identifier,
                self.current(),
                handlers::message_handler::Issuer::Parser,
            ) {
                Ok(name) => name.token_value,
                Err(error) => {
                    self.errors.insert(self.errors.len(), error);
                    return None;
                }
            };

            self.advance();

            // param_name ":" type <- ":" after the identifier
            match handlers::error_handler::Error::expected_or_error(
                "Colon",
                &types::tokens::TokenType::Colon,
                self.current(),
                handlers::message_handler::Issuer::Parser,
            ) {
                Ok(_) => {}
                Err(error) => {
                    self.errors.insert(self.errors.len(), error);
                    return None;
                }
            };

            self.advance();

            // param_name: "type" <- The type of the parameter
            let r#type = match self.get_type() {
                Ok(r#type) => r#type,
                Err(error) => {
                    self.errors.insert(self.errors.len(), error);
                    return None;
                }
            };

            self.advance();

            // The glue
            param = types::parse_nodes::FuncParam { name, r#type };

            // The end of the parameters statement or another parameter
            if self.peek_expect(&types::tokens::TokenType::RParen) {
                params.push(param.to_owned());
                break;
            } else if self.peek_expect(&types::tokens::TokenType::Comma) {
                params.push(param.to_owned());
                self.advance();
            } else {
                self.errors.insert(
                    self.errors.len(),
                    handlers::error_handler::Error::expected_error(
                        "',' or ')'",
                        self.current(),
                        handlers::message_handler::Issuer::Parser,
                    ),
                );
                return None;
            }
        }

        // This is because the loop above can exit with Eof too
        match handlers::error_handler::Error::expected_or_error(
            "end of paramenters statement",
            &types::tokens::TokenType::RParen,
            self.current(),
            handlers::message_handler::Issuer::Parser,
        ) {
            Ok(_) => {}
            Err(error) => {
                self.errors.insert(self.errors.len(), error);
                return None;
            }
        };

        Some(params)
    }

    fn parse_block_statement(
        &mut self,
        is_loop: &types::parse_nodes::Loop,
    ) -> Option<Vec<types::parse_nodes::Statement>> {
        // "{" <- Start of code block
        self.advance();

        // Block content
        let mut block_statements: Vec<types::parse_nodes::Statement> = vec![];
        let mut current_statement: types::parse_nodes::Statement;

        while !self.peek_expect(&types::tokens::TokenType::RBracket)
            || !self.peek_expect(&types::tokens::TokenType::Eof)
        {
            if self.peek_expect(&types::tokens::TokenType::RBracket) {
                break;
            } else if self.peek_expect(&types::tokens::TokenType::Eof) {
                self.errors.insert(
                    self.errors.len(),
                    handlers::error_handler::Error::expected_error(
                        "end of block statement",
                        self.current(),
                        handlers::message_handler::Issuer::Parser,
                    ),
                );
                return None;
            }
        }

        match handlers::error_handler::Error::expected_or_error(
            "end of code block statement",
            &types::tokens::TokenType::RBracket,
            self.current(),
            handlers::message_handler::Issuer::Parser,
        ) {
            Ok(_) => {}
            Err(error) => {
                self.errors.insert(self.errors.len(), error);
                return None;
            }
        };

        Some(block_statements)
    }

    /// Parse and return a function statement -> fn function_name(param_name: type, ...) -> type {...}
    fn parse_function_satement(&mut self) -> Option<types::parse_nodes::Statement> {
        // "fn" token <- used to get the start of the statement
        let fn_token = self.current().to_owned();
        self.advance();

        // "name" <- function name (identifier)
        let name: String = match handlers::error_handler::Error::expected_or_error(
            "a identifier",
            &types::tokens::TokenType::Identifier,
            self.current(),
            handlers::message_handler::Issuer::Parser,
        ) {
            Ok(token) => token.token_value,
            Err(error) => {
                self.errors.insert(self.errors.len(), error);
                return None;
            }
        };

        self.advance();

        // "(" <- Start of function parameters
        match handlers::error_handler::Error::expected_or_error(
            "'('",
            &types::tokens::TokenType::LParen,
            self.current(),
            handlers::message_handler::Issuer::Parser,
        ) {
            Ok(_) => {}
            Err(error) => {
                self.errors.insert(self.errors.len(), error);
                return None;
            }
        };

        // Function parameters
        let params = match self.parse_params_statement() {
            Some(params) => params,
            None => return None,
        };

        // ")" <- End of function parameters
        self.advance();

        // Return type of the function
        let r#type: types::built_in_types::BuiltInTypes;

        // '->' <- Define function return type
        // The explicity return type is optional
        if self.peek_expect(&types::tokens::TokenType::OpSetOrAcess) {
            // "->"
            self.advance();

            // Return type
            r#type = match self.get_type() {
                Ok(r#type) => r#type,
                Err(error) => {
                    self.errors.insert(self.errors.len(), error);
                    return None;
                }
            };

            // "{"
            self.advance();
        } else if self.peek_expect(&types::tokens::TokenType::LBracket) {
            // If no explicity declared return type
            r#type = types::built_in_types::BuiltInTypes::Any;
        } else {
            self.errors.insert(
                self.errors.len(),
                handlers::error_handler::Error::expected_error(
                    "'->' or '{'",
                    self.current(),
                    handlers::message_handler::Issuer::Parser,
                ),
            );
            return None;
        }

        // "{" <- Start of function code block
        match handlers::error_handler::Error::expected_or_error(
            "'{'",
            &types::tokens::TokenType::LBracket,
            self.current(),
            handlers::message_handler::Issuer::Parser,
        ) {
            Ok(_) => {}
            Err(error) => {
                self.errors.insert(self.errors.len(), error);
                return None;
            }
        };

        let body = match self.parse_block_statement(&types::parse_nodes::Loop::No) {
            Some(body) => body,
            None => return None,
        };

        Some(types::parse_nodes::Statement::FuctionDeclaration {
            start: fn_token.position,
            name,
            r#type,
            params,
            body: Box::new(body),
        })
    }

    /// While doesn't reaches EOF parse tokens
    pub fn parse_tokens(&mut self) {
        while !self.peek_expect(&types::tokens::TokenType::Eof) {
            // Parse and return a statement
            let ast_node: Option<types::parse_nodes::Statement> = match &self.current_type() {
                types::tokens::TokenType::KwFn => self.parse_function_satement(),
                types::tokens::TokenType::KwLet => self.parse_let_statement(),
                _ => {
                    handlers::error_handler::Error::expected_error(
                        "a statement",
                        self.current(),
                        handlers::message_handler::Issuer::Parser,
                    );
                    break;
                }
            };

            // If a parse function find a error, it will return nothing
            if ast_node.is_some() {
                self.push_statement(ast_node.unwrap());
            } else {
                break;
            }

            self.advance();
        }
    }
}
