use crate::opcodes::Opcode;

pub struct BytecodeReader {
    data: Vec<u8>,
    idx: usize,
}

pub enum ReaderError {
    EndOfStream,
}

impl From<&[u8]> for BytecodeReader {
    fn from(bytes: &[u8]) -> BytecodeReader {
        Self {
            data: bytes.to_vec(),
            idx: 0,
        }
    }
}

impl From<Vec<u8>> for BytecodeReader {
    fn from(bytes: Vec<u8>) -> BytecodeReader {
        Self { data: bytes, idx: 0 }
    }
}

impl BytecodeReader {
    #[inline(always)]
    pub fn read_u8(&mut self) -> Result<u8, ReaderError> {
        if self.idx >= self.data.len() {
            Err(ReaderError::EndOfStream)
        } else {
            self.idx += 1;
            Ok(self.data[self.idx - 1])
        }
    }

    #[inline(always)]
    pub fn read_u16(&mut self) -> Result<u16, ReaderError> {
        if self.idx + 1 >= self.data.len() {
            Err(ReaderError::EndOfStream)
        } else {
            let mut data: u16 = self.data[self.idx] as u16;
            data = (data << 8) | (self.data[self.idx + 1] as u16);
            self.idx += 2;
            Ok(data)
        }
    }

    #[inline(always)]
    pub fn jump_to(&mut self, ip: usize) -> Result<(), ReaderError> {
        if ip > self.data.len() {
            Err(ReaderError::EndOfStream)
        } else {
            self.idx = ip;
            Ok(())
        }
    }
}
