pub mod opcodes;
pub mod bytecode_reader;
pub mod memory;

use opcodes::Opcode;
use bytecode_reader::{BytecodeReader, ReaderError};
use memory::Stack;

pub struct VirtualMachine {
    stack: Stack<u8>,
    //constants: ConstantPool,
    bytecode: BytecodeReader,
}

impl VirtualMachine {
    pub fn new(/*constants: ConstantPool, */bytecode: BytecodeReader) -> VirtualMachine {
        Self {
            stack: Stack::new(),
            //constants,
            bytecode,
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.bytecode.read_u8() {
                Ok(byte) => {
                   match Opcode::try_from(byte) {
                       Ok(opcode) => {
                           self.run_opcode(&opcode);
                       }
                       Err(_) => {
                           dbg!("Invalid opcode");
                           break;
                       }
                   }
                }
                Err(ReaderError::EndOfStream) => {
                    dbg!("EndOfStream");
                    break;
                }
            }
        }
    }

    fn run_opcode(&mut self, opcode: &Opcode) {
        match opcode {
            Opcode::Nop => (),
            Opcode::Push => {
                let result = self.bytecode.read_u8();
                if let Ok(byte) = result {
                    self.stack.push(byte);
                }
            },
            Opcode::Pop => {
                self.stack.pop();
            },
            Opcode::Add => {
                let rhs = self.stack.pop().unwrap_or(0x00);
                let lhs = self.stack.pop().unwrap_or(0x00);
                self.stack.push(rhs + lhs);
            },
            Opcode::Sub => {
                let rhs = self.stack.pop().unwrap_or(0x00);
                let lhs = self.stack.pop().unwrap_or(0x00);
                self.stack.push(lhs - rhs);
            }
            Opcode::Mul => {
                let rhs = self.stack.pop().unwrap_or(0x00);
                let lhs = self.stack.pop().unwrap_or(0x00);
                self.stack.push(lhs * rhs);
            },
            Opcode::Div => {
                let rhs = self.stack.pop().unwrap_or(0x00);
                let lhs = self.stack.pop().unwrap_or(0x00);
                self.stack.push(lhs / rhs);
            },
            Opcode::Print => {
                let value = self.stack.pop().unwrap_or(0x00);
                println!("{}", value);
                self.stack.push(value);
            },
            Opcode::Halt => {}
        }
    }
}
