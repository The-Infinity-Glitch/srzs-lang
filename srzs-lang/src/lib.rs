use std::sync::Arc;
use utils::backend::bytecode;
use utils::frontend::*;
mod module;
mod runtime;

pub fn load_source(source: &str) {
    use std::fs::File;
    use std::io::Read;

    let mut data_file = File::open(source)
        .map_err(|err| format!("Could not open `{}`, {}", source, err))
        .unwrap();
    let mut data = Arc::new(String::new());
    data_file
        .read_to_string(Arc::make_mut(&mut data))
        .map_err(|err| format!("Could not open `{}`, {}", source, err))
        .unwrap();

    let mut lexer = lexer::Lexer::new();
    lexer.lex(data.as_str());

    dbg!(&lexer);

    let mut parser = parser::Parser::new(lexer.tokens);
    parser.parse_tokens();

    dbg!(parser);

    let test_bytecode = bytecode::ByteCode {
        code: vec![bytecode::Scope::Global(vec![
            bytecode::ByteNode::Push(bytecode::Value::Int(10)),
            bytecode::ByteNode::Push(bytecode::Value::Int(2)),
            bytecode::ByteNode::Push(bytecode::Value::Int(3)),
            bytecode::ByteNode::Pull((bytecode::ValueScope::Global, 0)),
            bytecode::ByteNode::Pop(2),
        ])],
    };

    let mut test_run = runtime::Runtime::new();
    test_run.run_byte(test_bytecode);
    dbg!(test_run);
}
