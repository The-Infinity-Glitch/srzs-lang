use utils::backend::bytecode;

pub struct Module {
    name: String,
    bytecode: bytecode::ByteCode,
}
