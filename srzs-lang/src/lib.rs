use std::sync::Arc;

use utils::frontend::*;

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
}
