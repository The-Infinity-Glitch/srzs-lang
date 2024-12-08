#[derive(Debug, Clone)]
pub enum Issuer {
    Lexer,
    Parser,
}

#[derive(Debug, Clone)]
pub struct Message {
    content: String,
    from: Issuer,
}

impl Message {
    pub fn new(content: &str, from: Issuer) -> Self {
        Self {
            content: content.to_string(),
            from,
        }
    }
}
