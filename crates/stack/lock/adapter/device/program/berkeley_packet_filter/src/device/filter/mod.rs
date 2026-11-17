use crate::instruction::Instruction;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DeviceFilter {
    instructions: Vec<Instruction>,
}

impl Layer for DeviceFilter {
    type Layer = InfrastructureLayer;
}

impl DeviceFilter {
    const MAP_LOOKUP_HELPER: i32 = 1;
    const ANCESTOR_HELPER: i32 = 123;
    const TYPE_MASK: i32 = 0xFFFF;
    const OPEN_MASK: i32 = 0x60000;
    const DEVICE_KEY_OFFSET: i16 = -16;
    const PERMISSION_KEY_OFFSET: i16 = -32;
    const LOCK_OFFSET: i16 = -24;
    const PADDING_OFFSET: i16 = -20;
    const ALLOWANCE: i32 = 1;
    const DENIAL: i32 = 0;

    pub fn compose_bytes(&self) -> Vec<u8> {
        Vec::from_iter(
            self.instructions
                .iter()
                .flat_map(Instruction::compose_bytes),
        )
    }

    fn jump_to_allowance(operation: u8, offset: usize, position: usize) -> Instruction {
        let distance = i16::try_from(offset - position - 1).unwrap_or(i16::MAX);
        Instruction::compose(operation, 0, 0, distance, 0)
    }

    fn load_map(register: u8, descriptor: i32) -> [Instruction; 2] {
        [
            Instruction::compose(
                Instruction::LOAD_DOUBLE_WORD,
                register,
                Instruction::MAP_SOURCE,
                0,
                descriptor,
            ),
            Instruction::compose(0, 0, 0, 0, 0),
        ]
    }
}

impl From<(i32, i32, u32)> for DeviceFilter {
    fn from((device_descriptor, permission_descriptor, ancestry_depth): (i32, i32, u32)) -> Self {
        let context = Instruction::CONTEXT_REGISTER;
        let stack = Instruction::STACK_REGISTER;
        let mut instructions = vec![
            Instruction::compose(Instruction::MOVE_REGISTER, context, 1, 0, 0),
            Instruction::compose(Instruction::LOAD_WORD, 2, context, 0, 0),
            Instruction::compose(Instruction::MOVE_REGISTER, 0, 2, 0, 0),
            Instruction::compose(Instruction::AND_IMMEDIATE, 0, 0, 0, Self::OPEN_MASK),
        ];
        let mut jumps = vec![instructions.len()];
        instructions.push(Instruction::compose(Instruction::EQUALITY_JUMP, 0, 0, 0, 0));
        instructions.push(Instruction::compose(
            Instruction::AND_IMMEDIATE,
            2,
            0,
            0,
            Self::TYPE_MASK,
        ));
        instructions.push(Instruction::compose(
            Instruction::STORE_WORD,
            stack,
            2,
            Self::DEVICE_KEY_OFFSET,
            0,
        ));
        instructions.push(Instruction::compose(
            Instruction::LOAD_WORD,
            3,
            context,
            4,
            0,
        ));
        instructions.push(Instruction::compose(
            Instruction::STORE_WORD,
            stack,
            3,
            -12,
            0,
        ));
        instructions.push(Instruction::compose(
            Instruction::LOAD_WORD,
            4,
            context,
            8,
            0,
        ));
        instructions.push(Instruction::compose(
            Instruction::STORE_WORD,
            stack,
            4,
            -8,
            0,
        ));
        instructions.extend(Self::load_map(1, device_descriptor));
        instructions.push(Instruction::compose(
            Instruction::MOVE_REGISTER,
            2,
            stack,
            0,
            0,
        ));
        instructions.push(Instruction::compose(
            Instruction::ADD_IMMEDIATE,
            2,
            0,
            0,
            i32::from(Self::DEVICE_KEY_OFFSET),
        ));
        instructions.push(Instruction::compose(
            Instruction::CALL_HELPER,
            0,
            0,
            0,
            Self::MAP_LOOKUP_HELPER,
        ));
        jumps.push(instructions.len());
        instructions.push(Instruction::compose(Instruction::EQUALITY_JUMP, 0, 0, 0, 0));
        instructions.push(Instruction::compose(Instruction::LOAD_WORD, 7, 0, 0, 0));
        instructions.push(Instruction::compose(
            Instruction::STORE_WORD,
            stack,
            7,
            Self::LOCK_OFFSET,
            0,
        ));
        instructions.push(Instruction::compose(
            Instruction::STORE_WORD_IMMEDIATE,
            stack,
            0,
            Self::PADDING_OFFSET,
            0,
        ));
        for level in 1..=ancestry_depth {
            let level_operand = i32::try_from(level).unwrap_or(i32::MAX);
            instructions.push(Instruction::compose(
                Instruction::MOVE_IMMEDIATE,
                1,
                0,
                0,
                level_operand,
            ));
            instructions.push(Instruction::compose(
                Instruction::CALL_HELPER,
                0,
                0,
                0,
                Self::ANCESTOR_HELPER,
            ));
            instructions.push(Instruction::compose(
                Instruction::STORE_DOUBLE_WORD,
                stack,
                0,
                Self::PERMISSION_KEY_OFFSET,
                0,
            ));
            instructions.extend(Self::load_map(1, permission_descriptor));
            instructions.push(Instruction::compose(
                Instruction::MOVE_REGISTER,
                2,
                stack,
                0,
                0,
            ));
            instructions.push(Instruction::compose(
                Instruction::ADD_IMMEDIATE,
                2,
                0,
                0,
                i32::from(Self::PERMISSION_KEY_OFFSET),
            ));
            instructions.push(Instruction::compose(
                Instruction::CALL_HELPER,
                0,
                0,
                0,
                Self::MAP_LOOKUP_HELPER,
            ));
            jumps.push(instructions.len());
            instructions.push(Instruction::compose(
                Instruction::INEQUALITY_JUMP,
                0,
                0,
                0,
                0,
            ));
        }
        instructions.push(Instruction::compose(
            Instruction::MOVE_IMMEDIATE,
            0,
            0,
            0,
            Self::DENIAL,
        ));
        instructions.push(Instruction::compose(Instruction::EXIT, 0, 0, 0, 0));
        let allowance_position = instructions.len();
        instructions.push(Instruction::compose(
            Instruction::MOVE_IMMEDIATE,
            0,
            0,
            0,
            Self::ALLOWANCE,
        ));
        instructions.push(Instruction::compose(Instruction::EXIT, 0, 0, 0, 0));
        for jump_position in jumps {
            let operation = instructions[jump_position].compose_bytes()[0];
            instructions[jump_position] = Self::jump_to_allowance(
                operation,
                allowance_position,
                jump_position,
            );
        }
        Self { instructions }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CODE: &[u8] = &[
        0xbf, 0x16, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x61, 0x62, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0xbf, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x57, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x06, 0x00, 0x15, 0x00, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x57, 0x02, 0x00, 0x00, 0xff,
        0xff, 0x00, 0x00, 0x63, 0x2a, 0xf0, 0xff, 0x00, 0x00, 0x00, 0x00, 0x61, 0x63, 0x04, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x63, 0x3a, 0xf4, 0xff, 0x00, 0x00, 0x00, 0x00, 0x61, 0x64, 0x08,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x63, 0x4a, 0xf8, 0xff, 0x00, 0x00, 0x00, 0x00, 0x18, 0x11,
        0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbf,
        0xa2, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0x02, 0x00, 0x00, 0xf0, 0xff, 0xff, 0xff,
        0x85, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x15, 0x00, 0x17, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x61, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x63, 0x7a, 0xe8, 0xff, 0x00, 0x00,
        0x00, 0x00, 0x62, 0x0a, 0xec, 0xff, 0x00, 0x00, 0x00, 0x00, 0xb7, 0x01, 0x00, 0x00, 0x01,
        0x00, 0x00, 0x00, 0x85, 0x00, 0x00, 0x00, 0x7b, 0x00, 0x00, 0x00, 0x7b, 0x0a, 0xe0, 0xff,
        0x00, 0x00, 0x00, 0x00, 0x18, 0x11, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0xbf, 0xa2, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0x02,
        0x00, 0x00, 0xe0, 0xff, 0xff, 0xff, 0x85, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x55,
        0x00, 0x0b, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb7, 0x01, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
        0x85, 0x00, 0x00, 0x00, 0x7b, 0x00, 0x00, 0x00, 0x7b, 0x0a, 0xe0, 0xff, 0x00, 0x00, 0x00,
        0x00, 0x18, 0x11, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0xbf, 0xa2, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0x02, 0x00, 0x00, 0xe0,
        0xff, 0xff, 0xff, 0x85, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x55, 0x00, 0x02, 0x00,
        0x00, 0x00, 0x00, 0x00, 0xb7, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x95, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0xb7, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x95, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    #[test]
    fn the_filter_of_two_descriptors_and_a_depth_is_the_proven_program() {
        let device_filter = DeviceFilter::from((3, 4, 2));

        let bytes = device_filter.compose_bytes();

        assert_eq!(bytes.len(), 42 * 8);
        assert_eq!(bytes, CODE);
    }

    #[test]
    fn a_deeper_filter_holds_nine_instructions_for_every_level() {
        let device_filter = [
            DeviceFilter::from((3, 4, 1)),
            DeviceFilter::from((3, 4, 16)),
        ];

        let distance = device_filter[1].compose_bytes().len()
            - device_filter[0].compose_bytes().len();

        assert_eq!(distance, 15 * 9 * 8);
    }
}
