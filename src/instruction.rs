#[derive(Debug, PartialEq)]
pub enum NoArgInstructionType {
    ClearDisplay, // 00E0 - CLS
    Return, // 00EE - RET
}

#[derive(Debug, PartialEq)]
pub enum AddressInstructionType {
    SYS, // 0nnn - SYS
    JumpDirect, // 1nnn - JP
    Call, // 2nnn - CALL
    SetI, // Annn - LD I
    JumpAddV0, // Bnnn JP V0
}

#[derive(Debug, PartialEq)]
pub struct AddressInstruction {
    instruction_type: AddressInstructionType,
    address: u16,
}

#[derive(Debug, PartialEq)]
pub enum RegisterByteInstructionType {
    SkipEqual, // 3xkk - SE
    SkipNotEqual, // 4xkk - SNE
    Set, // 6xkk - LD
    Add, // 7xkk - ADD
    RandAnd, // cxkk - RND
}

#[derive(Debug, PartialEq)]
pub struct RegisterByteInstruction {
    instruction_type: RegisterByteInstructionType,
    register: u8,
    byte: u8
}

#[derive(Debug, PartialEq)]
pub enum SingleRegisterInstructionType {
    SkipPressed, // Ex9E - SKP Vx
    SkipNotPressed, // ExA1 - SKNP Vx
    ReadDelayTimer, // Fx07 - LD Vx DT
    WaitForKeyPress, // Fx0A - LD Vx K
    SetDelayTimer, // Fx15 - LD DT Vx
    SetSoundTimer, // Fx18 - LD ST Vx
    AddI, // Fx1E - ADD I Vx
    LoadSprite, // Fx29 - LD F Vx
    StoreBCD, // Fx33 - LD B Vx
    StoreRegisters, // Fx55 - LD [I] Vx
    ReadToRegisters, // Fx65 - LD Vx [I]
}

#[derive(Debug, PartialEq)]
pub struct SingleRegisterInstruction {
    instruction_type: SingleRegisterInstructionType,
    register: u8,
}

#[derive(Debug, PartialEq)]
pub enum TwoRegisterInstructionType {
    SkipEqual, // 5xy0 - SE Vx, Vy
    Set, // 8xy0 - LD Vx, Vy
    Or, // 8xy1 - OR Vx, Vy
    And, // 8xy2 - AND Vx, Vy
    ExclusiveOr, // 8xy3 - XOR Vx, Vy
    Add, // 8xy4 - ADD Vx, Vy
    SubtractBorrow, // 8xy5 - SUB Vx, Vy
    ShiftRight, // 8xy6 - SHR Vx
    SubtractNotBorrow, // 8xy7 - SUBN Vx, Vy
    ShiftLeft, // 8xyE - SHL Vx
    SkipNotEqual, // 9xy0 - SNE Vx, Vy
}

#[derive(Debug, PartialEq)]
pub struct TwoRegisterInstruction {
    instruction_type: TwoRegisterInstructionType,
    Vx: u8,
    Vy: u8,
}

#[derive(Debug, PartialEq)]
pub struct DrawInstruction {
    Vx: u8,
    Vy: u8,
    height: u8,
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    NoArgInstruction(NoArgInstructionType),
    AddressInstruction(AddressInstruction),
    RegisterByteInstruction(RegisterByteInstruction),
    SingleRegisterInstruction(SingleRegisterInstruction),
    TwoRegisterInstruction(TwoRegisterInstruction),
    DrawInstruction(DrawInstruction),
}



impl Instruction {
    pub fn parse(raw: (u8, u8)) -> Instruction {
        let (upper_byte, lower_byte) = raw;

        // first 4 bits are easiest to group instructions by
        let first_four_bit_value: u8 = (upper_byte & (0b1111 << 4)) >> 4;

        // address instructions use the last 12 bits as the address value
        let last_twelve_bit_value: u16 = ((upper_byte as u16 & 0b1111u16) << 8) | lower_byte as u16;

        let second_four_bit_values: u8 = upper_byte & 0b1111;

        let third_four_bit_values: u8 = (lower_byte & (0b1111 << 4)) >> 4;

        let last_four_bit_values: u8 = lower_byte & 0b1111;

        match first_four_bit_value {
            0x0 => {
                match lower_byte {
                    0xe0 => Instruction::NoArgInstruction(NoArgInstructionType::ClearDisplay),
                    0xee => Instruction::NoArgInstruction(NoArgInstructionType::Return),
                    _ => Instruction::AddressInstruction(AddressInstruction {
                        instruction_type: AddressInstructionType::SYS,
                        address: last_twelve_bit_value,
                    })
                }
            },
            0x1 => Instruction::AddressInstruction(AddressInstruction {
                instruction_type: AddressInstructionType::JumpDirect,
                address: last_twelve_bit_value,
            }),
            0x2 => Instruction::AddressInstruction(AddressInstruction {
                instruction_type: AddressInstructionType::Call,
                address: last_twelve_bit_value,
            }),
            0x3 => Instruction::RegisterByteInstruction(RegisterByteInstruction {
                instruction_type: RegisterByteInstructionType::SkipEqual,
                register: second_four_bit_values,
                byte: lower_byte,
            }),
            0x4 => Instruction::RegisterByteInstruction(RegisterByteInstruction {
                instruction_type: RegisterByteInstructionType::SkipNotEqual,
                register: second_four_bit_values,
                byte: lower_byte,
            }),
            0x5 => {
                match last_four_bit_values {
                    0x0 => Instruction::TwoRegisterInstruction(TwoRegisterInstruction {
                        instruction_type: TwoRegisterInstructionType::SkipEqual,
                        Vx: second_four_bit_values,
                        Vy: third_four_bit_values,
                    }),
                    _ => panic!("Invalid Instruction")
                }
            }
            0x6 => Instruction::RegisterByteInstruction(RegisterByteInstruction {
                instruction_type: RegisterByteInstructionType::Set,
                register: second_four_bit_values,
                byte: lower_byte,
            }),
            0x7 => Instruction::RegisterByteInstruction(RegisterByteInstruction {
                instruction_type: RegisterByteInstructionType::Add,
                register: second_four_bit_values,
                byte: lower_byte,
            }),
            0x8 => {
                match last_four_bit_values {
                    0x0 => Instruction::TwoRegisterInstruction(TwoRegisterInstruction {
                        instruction_type: TwoRegisterInstructionType::Set,
                        Vx: second_four_bit_values,
                        Vy: third_four_bit_values,
                    }),
                    0x1 => Instruction::TwoRegisterInstruction(TwoRegisterInstruction {
                        instruction_type: TwoRegisterInstructionType::Or,
                        Vx: second_four_bit_values,
                        Vy: third_four_bit_values,
                    }),
                    0x2 => Instruction::TwoRegisterInstruction(TwoRegisterInstruction {
                        instruction_type: TwoRegisterInstructionType::And,
                        Vx: second_four_bit_values,
                        Vy: third_four_bit_values,
                    }),
                    0x3 => Instruction::TwoRegisterInstruction(TwoRegisterInstruction {
                        instruction_type: TwoRegisterInstructionType::ExclusiveOr,
                        Vx: second_four_bit_values,
                        Vy: third_four_bit_values,
                    }),
                    0x4 => Instruction::TwoRegisterInstruction(TwoRegisterInstruction {
                        instruction_type: TwoRegisterInstructionType::Add,
                        Vx: second_four_bit_values,
                        Vy: third_four_bit_values,
                    }),
                    0x5 => Instruction::TwoRegisterInstruction(TwoRegisterInstruction {
                        instruction_type: TwoRegisterInstructionType::SubtractBorrow,
                        Vx: second_four_bit_values,
                        Vy: third_four_bit_values,
                    }),
                    0x6 => Instruction::TwoRegisterInstruction(TwoRegisterInstruction {
                        instruction_type: TwoRegisterInstructionType::ShiftRight,
                        Vx: second_four_bit_values,
                        Vy: third_four_bit_values,
                    }),
                    0x7 => Instruction::TwoRegisterInstruction(TwoRegisterInstruction {
                        instruction_type: TwoRegisterInstructionType::SubtractNotBorrow,
                        Vx: second_four_bit_values,
                        Vy: third_four_bit_values,
                    }),
                    0xe => Instruction::TwoRegisterInstruction(TwoRegisterInstruction {
                        instruction_type: TwoRegisterInstructionType::ShiftLeft,
                        Vx: second_four_bit_values,
                        Vy: third_four_bit_values,
                    }),
                    _ => panic!("Invalid Instruction")
                }
            },
            0x9 => {
                match last_four_bit_values {
                    0x0 => Instruction::TwoRegisterInstruction(TwoRegisterInstruction {
                        instruction_type: TwoRegisterInstructionType::SkipNotEqual,
                        Vx: second_four_bit_values,
                        Vy: third_four_bit_values,
                    }),
                    _ => panic!("Invalid Instruction")
                }
            }
            0xa => Instruction::AddressInstruction(AddressInstruction {
                instruction_type: AddressInstructionType::SetI,
                address: last_twelve_bit_value,
            }),
            0xb => Instruction::AddressInstruction(AddressInstruction {
                instruction_type: AddressInstructionType::JumpAddV0,
                address: last_twelve_bit_value,
            }),
            0xc => Instruction::RegisterByteInstruction(RegisterByteInstruction {
                instruction_type: RegisterByteInstructionType::RandAnd,
                register: second_four_bit_values,
                byte: lower_byte,
            }),
            0xd => Instruction::DrawInstruction(DrawInstruction {
                Vx: second_four_bit_values,
                Vy: third_four_bit_values,
                height: last_four_bit_values,
            }),
            0xe => {
                match lower_byte {
                    0x9e => Instruction::SingleRegisterInstruction(SingleRegisterInstruction {
                        instruction_type: SingleRegisterInstructionType::SkipPressed,
                        register: second_four_bit_values,
                    }),
                    0xa1 => Instruction::SingleRegisterInstruction(SingleRegisterInstruction {
                        instruction_type: SingleRegisterInstructionType::SkipNotPressed,
                        register: second_four_bit_values,
                    }),
                    _  => panic!("Invalid Instruction"),
                }
            },
            0xf => {
                match lower_byte {
                    0x07 => Instruction::SingleRegisterInstruction(SingleRegisterInstruction {
                        instruction_type: SingleRegisterInstructionType::ReadDelayTimer,
                        register: second_four_bit_values,
                    }),
                    0x0a => Instruction::SingleRegisterInstruction(SingleRegisterInstruction {
                        instruction_type: SingleRegisterInstructionType::WaitForKeyPress,
                        register: second_four_bit_values,
                    }),
                    0x15 => Instruction::SingleRegisterInstruction(SingleRegisterInstruction {
                        instruction_type: SingleRegisterInstructionType::SetDelayTimer,
                        register: second_four_bit_values,
                    }),
                    0x18 => Instruction::SingleRegisterInstruction(SingleRegisterInstruction {
                        instruction_type: SingleRegisterInstructionType::SetSoundTimer,
                        register: second_four_bit_values,
                    }),
                    0x1e => Instruction::SingleRegisterInstruction(SingleRegisterInstruction {
                        instruction_type: SingleRegisterInstructionType::AddI,
                        register: second_four_bit_values
                    }),
                    0x29 => Instruction::SingleRegisterInstruction(SingleRegisterInstruction {
                        instruction_type: SingleRegisterInstructionType::LoadSprite,
                        register: second_four_bit_values,
                    }),
                    0x33 => Instruction::SingleRegisterInstruction(SingleRegisterInstruction {
                        instruction_type: SingleRegisterInstructionType::StoreBCD,
                        register: second_four_bit_values,
                    }),
                    0x55 => Instruction::SingleRegisterInstruction(SingleRegisterInstruction {
                        instruction_type: SingleRegisterInstructionType::StoreRegisters,
                        register: second_four_bit_values,
                    }),
                    0x65 => Instruction::SingleRegisterInstruction(SingleRegisterInstruction {
                        instruction_type: SingleRegisterInstructionType::ReadToRegisters,
                        register: second_four_bit_values,
                    }),
                    _ => panic!("Invalid Instruction")
                }
            }
            _ => {
                // It shouldn't be possible to get here, but the type we match on is a u8, even
                // though the values are created with a bitwise and that would make it impossible to
                // be more than 2^4
                panic!("Invalid Instruction")
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{
        Instruction,
        NoArgInstructionType,
        AddressInstruction,
        AddressInstructionType,
        RegisterByteInstruction,
        RegisterByteInstructionType,
        SingleRegisterInstruction,
        SingleRegisterInstructionType,
        TwoRegisterInstruction,
        TwoRegisterInstructionType,
    };
    use crate::instruction::DrawInstruction;

    #[test]
    fn parse_handles_clear_display() {
        let raw_instruction = (0x0, 0xe0);

        let parse_instruction = Instruction::parse(raw_instruction);

        assert_eq!(parse_instruction, Instruction::NoArgInstruction(NoArgInstructionType::ClearDisplay))
    }

    #[test]
    fn parse_handles_return() {
        let raw_instruction = (0x0, 0xee);

        let parse_instruction = Instruction::parse(raw_instruction);

        assert_eq!(parse_instruction, Instruction::NoArgInstruction(NoArgInstructionType::Return))
    }

    #[test]
    fn parse_handles_sys() {
        let raw_instruction = (0x0a, 0xbc);

        let parsed_instruction = Instruction::parse(raw_instruction);

        assert_eq!(parsed_instruction, Instruction::AddressInstruction(AddressInstruction {
            instruction_type:  AddressInstructionType::SYS,
            address: 0xabc
        }))
    }

    #[test]
    fn parse_handles_jump_direct() {
        let raw_instruction = (0x12, 0x34);

        let parsed_instruction = Instruction::parse(raw_instruction);

        assert_eq!(parsed_instruction, Instruction::AddressInstruction(AddressInstruction {
            instruction_type: AddressInstructionType::JumpDirect,
            address: 0x234
        }))
    }

    #[test]
    fn parse_handles_call() {
        let raw_instruction = (0x23, 0x45);

        let parsed_instruction = Instruction::parse(raw_instruction);

        assert_eq!(parsed_instruction, Instruction::AddressInstruction(AddressInstruction {
            instruction_type: AddressInstructionType::Call,
            address: 0x345,
        }))
    }

    #[test]
    fn parse_handles_set_i() {
        let raw_instruction = (0xab, 0xcd);

        let parsed_instruction = Instruction::parse(raw_instruction);

        assert_eq!(parsed_instruction, Instruction::AddressInstruction(AddressInstruction {
            instruction_type: AddressInstructionType::SetI,
            address: 0xbcd,
        }))
    }

    #[test]
    fn parse_handles_jump_add_v0() {
        let raw_instruction = (0xbc, 0xde);

        let parsed_instruction = Instruction::parse(raw_instruction);

        assert_eq!(parsed_instruction, Instruction::AddressInstruction(AddressInstruction {
            instruction_type: AddressInstructionType::JumpAddV0,
            address: 0xcde,
        }))
    }

    #[test]
    fn parse_handles_register_byte_skip_equal() {
        let raw_instruction = (0x34,0x56);

        let parsed_instruction = Instruction::parse(raw_instruction);

        assert_eq!(parsed_instruction, Instruction::RegisterByteInstruction(RegisterByteInstruction {
            instruction_type: RegisterByteInstructionType::SkipEqual,
            register: 0x4,
            byte: 0x56,
        }))
    }

    #[test]
    fn parse_handles_register_byte_skip_not_equal() {
        let raw_instruction = (0x45, 0x67);

        let parsed_instruction = Instruction::parse(raw_instruction);

        assert_eq!(parsed_instruction, Instruction::RegisterByteInstruction(RegisterByteInstruction {
            instruction_type: RegisterByteInstructionType::SkipNotEqual,
            register: 0x5,
            byte: 0x67,
        }))
    }

    #[test]
    fn parse_handles_register_byte_set() {
        let raw_instruction = (0x67 ,0x89);

        let parsed_instruction = Instruction::parse(raw_instruction);

        assert_eq!(parsed_instruction, Instruction::RegisterByteInstruction(RegisterByteInstruction {
            instruction_type: RegisterByteInstructionType::Set,
            register: 0x7,
            byte: 0x89,
        }))
    }

    #[test]
    fn parse_handles_register_byte_add() {
        let raw_instruction = (0x78, 0x9a);

        let parsed_instruction = Instruction::parse(raw_instruction);

        assert_eq!(parsed_instruction, Instruction::RegisterByteInstruction(RegisterByteInstruction {
            instruction_type: RegisterByteInstructionType::Add,
            register: 0x8,
            byte: 0x9a,
        }));
    }

    #[test]
    fn parse_handles_register_byte_rand_and() {
        let raw_instruction = (0xcd,0xef);

        let parsed_instruction = Instruction::parse(raw_instruction);

        assert_eq!(parsed_instruction, Instruction::RegisterByteInstruction(RegisterByteInstruction {
            instruction_type: RegisterByteInstructionType::RandAnd,
            register: 0xd,
            byte: 0xef,
        }))
    }

    #[test]
    fn parse_handles_skip_pressed() {
        let raw_instruction = (0xef, 0x9e);

        let parsed_instruction = Instruction::parse(raw_instruction);

        assert_eq!(parsed_instruction, Instruction::SingleRegisterInstruction(SingleRegisterInstruction {
            instruction_type: SingleRegisterInstructionType::SkipPressed,
            register: 0xf
        }))
    }

    #[test]
    fn parse_handles_skip_not_pressed() {
        let raw_instruction = (0xef, 0xa1);

        let parsed_instruction = Instruction::parse(raw_instruction);

        assert_eq!(parsed_instruction, Instruction::SingleRegisterInstruction(SingleRegisterInstruction {
            instruction_type: SingleRegisterInstructionType::SkipNotPressed,
            register: 0xf
        }))
    }

    #[test]
    fn parse_handles_read_delay_timer() {
        let raw_instruction = (0xf0, 0x07);

        let parsed_instruction = Instruction::parse(raw_instruction);

        assert_eq!(parsed_instruction, Instruction::SingleRegisterInstruction(SingleRegisterInstruction {
            instruction_type: SingleRegisterInstructionType::ReadDelayTimer,
            register: 0x0
        }))
    }

    #[test]
    fn parse_handles_wait_for_key_press() {
        let raw_instruction = (0xf1, 0x0a);

        let parsed_instruction = Instruction::parse(raw_instruction);

        assert_eq!(parsed_instruction, Instruction::SingleRegisterInstruction(SingleRegisterInstruction {
            instruction_type: SingleRegisterInstructionType::WaitForKeyPress,
            register: 0x1,
        }))
    }

    #[test]
    fn parse_handles_set_delay_timer() {
        let raw_instruction = (0xf2, 0x15);

        let parsed_instruction = Instruction::parse(raw_instruction);

        assert_eq!(parsed_instruction, Instruction::SingleRegisterInstruction(SingleRegisterInstruction {
            instruction_type: SingleRegisterInstructionType::SetDelayTimer,
            register: 0x2
        }))
    }

    #[test]
    fn parse_handles_set_sound_timer() {
        let raw_instruction = (0xf2, 0x18);

        let parsed_instruction = Instruction::parse(raw_instruction);

        assert_eq!(parsed_instruction, Instruction::SingleRegisterInstruction(SingleRegisterInstruction {
            instruction_type: SingleRegisterInstructionType::SetSoundTimer,
            register: 0x2
        }));
    }

    #[test]
    fn parse_handles_single_register_add() {
        let raw_instruction = (0xf3, 0x1e);

        let parsed_instruction = Instruction::parse(raw_instruction);

        assert_eq!(parsed_instruction, Instruction::SingleRegisterInstruction(SingleRegisterInstruction {
            instruction_type: SingleRegisterInstructionType::AddI,
            register: 0x3
        }));
    }

    #[test]
    fn parse_handles_load_sprite()  {
        let raw_instruction = (0xf4, 0x29);

        let parsed_instruction = Instruction::parse(raw_instruction);

        assert_eq!(parsed_instruction, Instruction::SingleRegisterInstruction(SingleRegisterInstruction {
            instruction_type: SingleRegisterInstructionType::LoadSprite,
            register: 0x4
        }))
    }

    #[test]
    fn parse_handles_store_bsd() {
        let raw_instruction = (0xf5, 0x33);

        let parsed_instruction = Instruction::parse(raw_instruction);

        assert_eq!(parsed_instruction, Instruction::SingleRegisterInstruction(SingleRegisterInstruction {
            instruction_type: SingleRegisterInstructionType::StoreBCD,
            register: 0x5,
        }))
    }

    #[test]
    fn parse_handles_store_registers() {
        let raw_instruction = (0xf6, 0x55);

        let parsed_instruction = Instruction::parse(raw_instruction);

        assert_eq!(parsed_instruction, Instruction::SingleRegisterInstruction(SingleRegisterInstruction {
            instruction_type: SingleRegisterInstructionType::StoreRegisters,
            register: 0x6
        }))
    }

    #[test]
    fn parse_handles_read_to_registers() {
        let raw_instruction = (0xf7, 0x65);

        let parsed_instruction = Instruction::parse(raw_instruction);

        assert_eq!(parsed_instruction, Instruction::SingleRegisterInstruction(SingleRegisterInstruction {
            instruction_type: SingleRegisterInstructionType::ReadToRegisters,
            register: 0x7
        }))
    }

    #[test]
    fn parse_handles_two_register_skip_equal() {
        let raw_instruction = (0x56, 0x70);

        let parsed_instruction = Instruction::parse(raw_instruction);

        assert_eq!(parsed_instruction, Instruction::TwoRegisterInstruction(TwoRegisterInstruction {
            instruction_type: TwoRegisterInstructionType::SkipEqual,
            Vx: 0x6,
            Vy: 0x7,
        }))
    }

    #[test]
    fn parse_handles_two_register_set() {
        let raw_instruction = (0x89, 0xa0);

        let parsed_instruction = Instruction::parse(raw_instruction);

        assert_eq!(parsed_instruction, Instruction::TwoRegisterInstruction(TwoRegisterInstruction{
            instruction_type: TwoRegisterInstructionType::Set,
            Vx: 0x9,
            Vy: 0xa
        }))
    }

    #[test]
    fn parse_handles_two_register_or() {
        let raw_instruction = (0x8a, 0xb1);

        let parsed_instruction = Instruction::parse(raw_instruction);

        assert_eq!(parsed_instruction, Instruction::TwoRegisterInstruction(TwoRegisterInstruction {
            instruction_type: TwoRegisterInstructionType::Or,
            Vx: 0xa,
            Vy: 0xb
        }))
    }

    #[test]
    fn parse_handles_two_register_and() {
        let raw_instruction = (0x8b, 0xc2);

        let parsed_instruction = Instruction::parse(raw_instruction);

        assert_eq!(parsed_instruction, Instruction::TwoRegisterInstruction(TwoRegisterInstruction {
            instruction_type: TwoRegisterInstructionType::And,
            Vx: 0xb,
            Vy: 0xc,
        }))
    }

    #[test]
    fn parse_handles_two_register_xor() {
        let raw_instruction = (0x8c, 0xd3);

        let parsed_instruction = Instruction::parse(raw_instruction);

        assert_eq!(parsed_instruction, Instruction::TwoRegisterInstruction(TwoRegisterInstruction {
            instruction_type: TwoRegisterInstructionType::ExclusiveOr,
            Vx: 0xc,
            Vy: 0xd,
        }))
    }

    #[test]
    fn parse_handles_two_register_add() {
        let raw_instruction = (0x8d, 0xe4);

        let parsed_instruction = Instruction::parse(raw_instruction);

        assert_eq!(parsed_instruction, Instruction::TwoRegisterInstruction(TwoRegisterInstruction {
            instruction_type: TwoRegisterInstructionType::Add,
            Vx: 0xd,
            Vy: 0xe
        }))
    }

    #[test]
    fn parse_handles_two_register_subtract_borrow() {
        let raw_instruction = (0x8e, 0xf5);

        let parsed_instruction = Instruction::parse(raw_instruction);

        assert_eq!(parsed_instruction, Instruction::TwoRegisterInstruction(TwoRegisterInstruction {
            instruction_type: TwoRegisterInstructionType::SubtractBorrow,
            Vx: 0xe,
            Vy: 0xf,
        }))
    }

    #[test]
    fn parse_handles_two_register_shift_right() {
        let raw_instruction = (0x8f, 0x06);

        let parsed_instruction = Instruction::parse(raw_instruction);

        assert_eq!(parsed_instruction, Instruction::TwoRegisterInstruction(TwoRegisterInstruction {
            instruction_type: TwoRegisterInstructionType::ShiftRight,
            Vx: 0xf,
            Vy: 0x0
        }))
    }

    #[test]
    fn parse_handles_two_register_subtract_not_borrow() {
        let raw_instruction = (0x80, 0x17);

        let parsed_instruction = Instruction::parse(raw_instruction);

        assert_eq!(parsed_instruction, Instruction::TwoRegisterInstruction(TwoRegisterInstruction {
            instruction_type: TwoRegisterInstructionType::SubtractNotBorrow,
            Vx: 0x0,
            Vy: 0x1,
        }))
    }

    #[test]
    fn parse_handles_two_register_shift_left() {
        let raw_instruction = (0x81, 0x2e);

        let parsed_instruction = Instruction::parse(raw_instruction);

        assert_eq!(parsed_instruction, Instruction::TwoRegisterInstruction(TwoRegisterInstruction {
            instruction_type: TwoRegisterInstructionType::ShiftLeft,
            Vx: 0x1,
            Vy: 0x2
        }))
    }

    #[test]
    fn parse_handles_two_register_skip_not_equal() {
        let raw_instruction = (0x92, 0x30);

        let parsed_instruction = Instruction::parse(raw_instruction);

        assert_eq!(parsed_instruction, Instruction::TwoRegisterInstruction(TwoRegisterInstruction {
            instruction_type: TwoRegisterInstructionType::SkipNotEqual,
            Vx: 0x2,
            Vy: 0x3
        }))
    }

    #[test]
    fn parse_handles_draw() {
        let raw_instruction = (0xd0, 0x12);

        let parsed_instruction = Instruction::parse(raw_instruction);

        assert_eq!(parsed_instruction, Instruction::DrawInstruction(DrawInstruction {
            Vx: 0x0,
            Vy: 0x1,
            height: 0x2
        }))
    }
}