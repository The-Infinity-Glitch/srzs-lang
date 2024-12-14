use crate::types;

#[derive(Debug, Clone)]
pub enum Statement {
    Module {
        start: types::others::Position,
        body: Box<Vec<Statement>>,
    },
    VariableDeclaration {
        start: types::others::Position,
        name: String,
        kind: VarDeclarationKind,
        r#type: types::built_in_types::BuiltInTypes,
        value: Option<Expression>,
    },
    ConstantDeclaration {
        start: types::others::Position,
        name: Option<String>,
        r#type: types::built_in_types::BuiltInTypes,
        value: Expression,
    },
    FuctionDeclaration {
        start: types::others::Position,
        name: String,
        r#type: types::built_in_types::BuiltInTypes,
        params: Vec<FuncParam>,
        body: Box<Vec<Statement>>,
    },
    If {
        start: types::others::Position,
        condition: Expression,
        body: Option<Box<Vec<Statement>>>,
        alternate: Option<Box<Vec<Statement>>>,
    },
    ElseIf {
        start: types::others::Position,
        condition: Expression,
        body: Option<Box<Vec<Statement>>>,
        alternate: Option<Box<Vec<Statement>>>,
    },
    Else {
        start: types::others::Position,
        body: Option<Box<Vec<Statement>>>,
    },
    While {
        start: types::others::Position,
        condition: Expression,
        body: Option<Box<Vec<Statement>>>,
    },
    For {
        start: types::others::Position,
        variable: Option<Box<Statement>>,
        condition: Option<Expression>,
        variable_update: Option<Box<Statement>>,
        body: Option<Box<Vec<Statement>>>,
        alternate: Option<Box<Vec<Statement>>>,
    },
    Break {
        start: types::others::Position,
    },
    Continue {
        start: types::others::Position,
    },
    Return {
        start: types::others::Position,
        expression: Expression,
    },
    VariableAlteration {
        name: String,
        operator: types::tokens::TokenType,
        value: Expression,
    },
    FunctionCall(Expression),
}

#[derive(Debug, Clone)]
pub enum Loop {
    Yes,
    No,
}

#[derive(Debug, Clone)]
pub struct FuncParam {
    pub name: String,
    pub r#type: types::built_in_types::BuiltInTypes,
}

#[derive(Debug, Clone)]
pub enum VarDeclarationKind {
    Mutable,
    Immutable,
}

#[derive(Debug, Clone)]
pub enum ArrayAcess {
    Acess {
        name: String,
        index: Box<Expression>,
    },
    NestedAcess {
        acess: Box<ArrayAcess>,
        index: Box<Expression>,
    },
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(String),
    Binary {
        operator: types::tokens::TokenType,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Logical {
        operator: types::tokens::TokenType,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Unary {
        operator: types::tokens::TokenType,
        operand: Box<Expression>,
    },
    Literal {
        r#type: types::tokens::TokenType,
        value: String,
    },
    ArrayLiteral {
        elements: Option<Box<Vec<Expression>>>,
    },
    ArrayAcess(ArrayAcess),
    Call {
        name: String,
        arguments: Option<Box<Vec<Expression>>>,
    },
}

#[derive(Debug, Clone)]
pub enum LiteralType {
    Numeric,
    String,
    Boolean,
}
