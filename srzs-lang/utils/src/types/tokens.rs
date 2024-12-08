use crate::frontend::lexer::{new_line_callback, word_callback};
use crate::types;
use logos::Logos;

/// All types of token of the laguage
#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(extras = (usize, usize))]
pub enum TokenType {
    // Special symbols
    #[token("\n", new_line_callback)]
    NewLine,

    #[regex("[ \t]", word_callback)]
    Space,

    #[token("\0", word_callback)]
    Eof,

    // Punctuation
    #[token(".", word_callback)]
    Dot,

    #[token(",", word_callback)]
    Comma,

    #[token(":", word_callback)]
    Colon,

    #[token(";", word_callback)]
    SemiColon,

    #[token("'", word_callback)]
    SingleQuote,

    #[token("\"", word_callback)]
    DoubleQuotes,

    // Delimiters
    #[token("(", word_callback)]
    LParen,

    #[token(")", word_callback)]
    RParen,

    #[token("[", word_callback)]
    LBrace,

    #[token("]", word_callback)]
    RBrace,

    #[token("{", word_callback)]
    LBracket,

    #[token("}", word_callback)]
    RBracket,

    #[token("-#", word_callback)]
    OpenComment,

    #[token("#-", word_callback)]
    CloseComment,

    // Declaration keywords
    #[token("ns", word_callback)]
    KwNs,

    #[token("use", word_callback)]
    KwUse,

    #[token("let", word_callback)]
    KwLet,

    #[token("const", word_callback)]
    KwConst,

    #[token("mut", word_callback)]
    KwMut,

    #[token("struct", word_callback)]
    KwStruct,

    #[token("fn", word_callback)]
    KwFn,

    // Logical keywords
    #[token("if", word_callback)]
    KwIf,

    #[token("else", word_callback)]
    KwElse,

    #[token("elif", word_callback)]
    KwElif,

    #[token("is", word_callback)]
    KwIs,

    // Special keywords
    #[token("return", word_callback)]
    KwReturn,

    // Built-in types
    #[token("null", word_callback)]
    TypeNull,

    #[token("int", word_callback)]
    TypeInt,

    #[token("float", word_callback)]
    TypeFloat,

    #[token("bool", word_callback)]
    TypeBoll,

    #[token("char", word_callback)]
    TypeChar,

    #[token("str", word_callback)]
    TypeStr,

    #[token("void", word_callback)]
    TypeVoid,

    #[token("any", word_callback)]
    TypeAny,

    // Binary Operators
    #[token("+", word_callback)]
    OpPlus,

    #[token("-", word_callback)]
    OpMinus,

    #[token("*", word_callback)]
    OpMultiply,

    #[token("/", word_callback)]
    OpDivision,

    #[token("%", word_callback)]
    OpRest,

    // Logical operators
    #[token("!", word_callback)]
    #[token("not", word_callback)]
    OpNot,

    #[token("&", word_callback)]
    #[token("and", word_callback)]
    OpAnd,

    #[token("|", word_callback)]
    #[token("or", word_callback)]
    OpOr,

    // Logical ternary -> (true or false)
    #[token("<", word_callback)]
    OpSmallerThan,

    #[token(">", word_callback)]
    OpGreaterThan,

    #[token(">=", word_callback)]
    OpSmallerOrEqualsThan,

    #[token("<=", word_callback)]
    OpGreaterOrEqualsThan,

    #[token("==", word_callback)]
    OpEquals,

    #[token("!=", word_callback)]
    OpNotEquals,

    // Assignment operators
    #[token("=", word_callback)]
    OpAssign,

    #[token("+=", word_callback)]
    OpAssignPlus,

    #[token("-=", word_callback)]
    OpAssignMinus,

    #[token("*=", word_callback)]
    OpAssignMultiply,

    #[token("/=", word_callback)]
    OpAssignDivision,

    #[token("%=", word_callback)]
    OpAssignRest,

    // Special operators -> Assignment, acess or set something
    #[token("->", word_callback)]
    OpSetOrAcess,

    #[token("::", word_callback)]
    OpAcess,

    #[token(":=", word_callback)]
    OpInferredTypeAssing,

    // Literals
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", word_callback)]
    Identifier,

    #[regex("[0-9]+", word_callback)]
    Number,

    #[token("_", priority = 3, callback = word_callback)]
    SomehingElse,

    #[regex(r#"'([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*'"#, word_callback)]
    CharLiteral,

    #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#, word_callback)]
    StringLiteral,

    #[token("true", word_callback)]
    True,

    #[token("false", word_callback)]
    False,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub token_value: String,
    pub position: types::others::Position,
}

impl Token {
    pub fn new(r#type: TokenType, value: &str, line: usize, column: usize) -> Self {
        Self {
            token_type: r#type,
            token_value: value.to_string(),
            position: types::others::Position::new(line, column),
        }
    }
}
