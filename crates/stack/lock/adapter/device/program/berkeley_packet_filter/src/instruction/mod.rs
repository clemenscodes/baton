use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
pub struct Instruction {
    operation: u8,
    destination: u8,
    source: u8,
    offset: i16,
    operand: i32,
}

impl Layer for Instruction {
    type Layer = InfrastructureLayer;
}

impl Instruction {
    pub const LOAD_WORD: u8 = 0x61;
    pub const STORE_WORD: u8 = 0x63;
    pub const STORE_DOUBLE_WORD: u8 = 0x7B;
    pub const STORE_WORD_IMMEDIATE: u8 = 0x62;
    pub const MOVE_REGISTER: u8 = 0xBF;
    pub const MOVE_IMMEDIATE: u8 = 0xB7;
    pub const ADD_IMMEDIATE: u8 = 0x07;
    pub const AND_IMMEDIATE: u8 = 0x57;
    pub const LOAD_DOUBLE_WORD: u8 = 0x18;
    pub const CALL_HELPER: u8 = 0x85;
    pub const EQUALITY_JUMP: u8 = 0x15;
    pub const INEQUALITY_JUMP: u8 = 0x55;
    pub const EXIT: u8 = 0x95;

    pub const CONTEXT_REGISTER: u8 = 6;
    pub const STACK_REGISTER: u8 = 10;
    pub const MAP_SOURCE: u8 = 1;

    pub const fn compose(
        operation: u8,
        destination: u8,
        source: u8,
        offset: i16,
        operand: i32,
    ) -> Self {
        Self {
            operation,
            destination,
            source,
            offset,
            operand,
        }
    }

    pub fn compose_bytes(&self) -> [u8; 8] {
        let register = (self.source << 4) | self.destination;
        let offset_bytes = self.offset.to_le_bytes();
        let operand_bytes = self.operand.to_le_bytes();
        [
            self.operation,
            register,
            offset_bytes[0],
            offset_bytes[1],
            operand_bytes[0],
            operand_bytes[1],
            operand_bytes[2],
            operand_bytes[3],
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_instruction_is_eight_bytes_of_its_fields() {
        let instruction = Instruction::compose(Instruction::STORE_WORD, 10, 2, -16, 0);

        let bytes = instruction.compose_bytes();

        assert_eq!(bytes, [0x63, 0x2a, 0xf0, 0xff, 0x00, 0x00, 0x00, 0x00]);
    }
}
