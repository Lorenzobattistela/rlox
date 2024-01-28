use std::vec::Vec;

enum Opcode {
    OP_CONSTANT,
    OP_ADD,
    OP_SUBTRACT,
    OP_MULTIPLY,
    OP_DIVIDE,
    OP_NEGATE,
    OP_RETURN,
}

struct Chunk {
    count: usize,
    code: Vec<u8>,
    lines: Vec<i16>,
}

impl Chunk {
    fn init_chunk(&mut self) {
        self.count = 0;
        self.code = Vec::new();
        self.code = Vec::new();
    }

    fn write_chunk(&mut self, byte: u8, line: i16) {
        self.code[self.count] = byte;
        self.lines[self.count] = line;
        self.count += 1;
    }
}

