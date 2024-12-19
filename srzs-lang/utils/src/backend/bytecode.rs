#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),
    Float(f32),
    String(String),
    Array(Vec<Value>),
}

#[derive(Debug, Clone)]
pub enum ValueScope {
    Global,
    Local,
    Args,
}

#[derive(Debug, Clone)]
pub enum ByteNode {
    Push(Value),
    Pull((ValueScope, usize)),
    Pop(usize),
    Mov(usize),
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
pub enum Scope {
    Global(Vec<ByteNode>),
    Function(Vec<ByteNode>),
}

#[derive(Debug, Clone)]
pub struct ByteCode {
    pub code: Vec<Scope>,
}
