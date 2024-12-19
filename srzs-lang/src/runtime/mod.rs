use crate::module::Module;
use utils::backend::bytecode;

#[derive(Debug, Clone)]
enum Stack {
    Global(Vec<bytecode::Value>),
    Local((Vec<bytecode::Value>, Vec<bytecode::Value>)),
}

#[derive(Debug, Clone)]
pub struct Runtime {
    global_stack: Stack,
    local_stack: Stack,
    stack_memory: Vec<Stack>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            global_stack: Stack::Global(vec![]),
            local_stack: Stack::Local((vec![], vec![])),
            stack_memory: vec![],
        }
    }

    pub fn run_byte(&mut self, input: bytecode::ByteCode) {
        for scope in input.code {
            match scope {
                bytecode::Scope::Global(code) => {
                    for statement in code {
                        match statement {
                            bytecode::ByteNode::Push(value) => {
                                match &mut self.global_stack {
                                    Stack::Global(stack) => {
                                        stack.insert(stack.len(), value);
                                    }
                                    Stack::Local(_) => {}
                                };
                            }
                            bytecode::ByteNode::Pull(index) => {
                                match &mut self.global_stack {
                                    Stack::Global(stack) => {
                                        let value = stack.get(index.1).unwrap().to_owned();

                                        stack.insert(stack.len(), value);
                                    }
                                    Stack::Local(_) => {}
                                };
                            }
                            bytecode::ByteNode::Pop(index) => {
                                match &mut self.global_stack {
                                    Stack::Global(stack) => {
                                        stack.remove(index);
                                    }
                                    Stack::Local(_) => {}
                                };
                            }
                            bytecode::ByteNode::Mov(index) => {
                                match &mut self.global_stack {
                                    Stack::Global(stack) => {
                                        let value = stack.get(index).unwrap().to_owned();
                                        stack.remove(index);

                                        stack.insert(stack.len(), value);
                                    }
                                    Stack::Local(_) => {}
                                };
                            }
                            bytecode::ByteNode::Add => todo!(),
                            bytecode::ByteNode::Sub => todo!(),
                            bytecode::ByteNode::Mul => todo!(),
                            bytecode::ByteNode::Div => todo!(),
                        }
                    }
                }
                bytecode::Scope::Function(code) => {}
            }
        }
    }
}

/*
entry:
    push(Value(Int(1)))     # [1]
    push(Value(Int(10)))    # [1, 10]
    push(Value(Int(2)))     # [1, 10, 2]
    mul                     # [1, 20]
    call test
test:
    pull args.1
    ret
*/
