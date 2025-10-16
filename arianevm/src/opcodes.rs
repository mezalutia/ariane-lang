use std::convert::TryFrom;
use std::fmt::Debug;

#[repr(u8)]
#[derive(Debug, Eq, PartialEq)]
pub enum Opcode {
    Nop   = 0x00,
    Push  = 0x01,
    Pop   = 0x02,
    Add   = 0x03,
    Sub   = 0x04,
    Mul   = 0x05,
    Div   = 0x06,
    Print = 0xFE,
    Halt  = 0xFF,
}

pub enum OpcodeError {
    InvalidOpcode,
}

impl TryFrom<u8> for Opcode {
    type Error = OpcodeError;

    fn try_from(value: u8) -> Result<Opcode, OpcodeError> {
        match value {
            0x00 => Ok(Opcode::Nop),
            0x01 => Ok(Opcode::Push),
            0x02 => Ok(Opcode::Pop),
            0x03 => Ok(Opcode::Add),
            0x04 => Ok(Opcode::Sub),
            0x05 => Ok(Opcode::Mul),
            0x06 => Ok(Opcode::Div),
            0xFE => Ok(Opcode::Print),
            0xFF => Ok(Opcode::Halt),
            _ => Err(OpcodeError::InvalidOpcode),
        }
    }
}

impl From<Opcode> for u8 {
    fn from(opcode: Opcode) -> u8 {
        opcode as u8
    }
}