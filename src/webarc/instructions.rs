use std::ops::Shl;
use webarc::registers::RegisterFile;
use webarc::memory::Memory;

pub trait Instruction {
    fn exec(&self, registers: &mut RegisterFile, memory: &mut Memory, instruction: u32);
    fn stringify(&self, address: u32, cond: &str, instruction: u32) -> String;
}

pub struct InstructionDecoder {
    branch: BranchInstruction,
    alu: AluInstruction,
    load_store: LoadStoreInstruction
}

impl InstructionDecoder {
    pub fn new() -> InstructionDecoder {
        InstructionDecoder {
            branch: BranchInstruction {},
            alu: AluInstruction {},
            load_store: LoadStoreInstruction {}
        }
    }

    pub fn decode(&self, word: u32) -> Option<&Instruction> {
        if (word & 0x0e000000) == 0x0a000000 {
            Some(&self.branch)
        } else if (word & 0x0c000000) == 0x00000000 {
            Some(&self.alu)
        } else if (word & 0x0c000000) == 0x04000000 {
            Some(&self.load_store)
        } else {
            None
        }
    }
}

struct BranchInstruction;
impl Instruction for BranchInstruction {
    fn exec(&self,registers: &mut RegisterFile, memory: &mut Memory, instruction: u32) {
        let pc = registers.reg_no_flags(15);

        if instruction & 0x01000000 != 0 {
            registers.set_reg(14, pc);
        }

        let offset = (instruction & 0x00ffffff).shl(2);
        // console.debug("Offset = 0x" + offset.toString(16));
        registers.set_reg_no_flags(15, (pc + offset) & 0x03ffffffu32);
    }

    fn stringify(&self, address: u32, cond: &str, instruction: u32) -> String {
        let offset = u32::shl(instruction & 0x00ffffff, 2);
        let dest = (address + offset + 8u32) & 0x03ffffffu32;

        if instruction & 0x01000000 != 0 {
            format!("BL{} {:0X}", cond, dest)
        } else {
            format!("B{} ${:0X}", cond, dest)
        }
    }
}

struct AluInstruction;
impl Instruction for AluInstruction {
    fn exec(&self, registers: &mut RegisterFile, memory: &mut Memory, instruction: u32) {

    }

    fn stringify(&self, address: u32, cond: &str, instruction: u32) -> String {
        format!("...{} ; ALU instruction", cond)
    }
}

struct LoadStoreInstruction;
impl Instruction for LoadStoreInstruction {
    fn exec(&self, registers: &mut RegisterFile, memory: &mut Memory, instruction: u32) {
        let op2_is_reg = (instruction & 1 << 25) != 0;
        let pre_indexing = (instruction & 1 << 24) != 0;
        let positive_offset = (instruction & 1 << 23) != 0;
        let byte_transfer = (instruction & 1 << 22) != 0;
        let write_back = (instruction & 1 << 21) != 0 || !pre_indexing;
        let is_load = (instruction & 1 << 20) != 0;

        let base_reg = (instruction >> 16) & 0xf;
        let base = registers.reg_no_flags(base_reg as usize);
        let sd_reg = (instruction >> 12) & 0xf;

        let offset= if op2_is_reg {
            let shift_amount = (instruction >> 7) & 0x1f;
            let shift_type = (instruction >> 5) & 0x3;
            let op2_reg = instruction & 0xf;
            let unshifted = registers.reg(op2_reg as usize);

            match shift_type {
                0b00 => unshifted << shift_amount,
                0b01 => (unshifted as i32 >> shift_amount) as u32,
                0b10 => unshifted >> shift_amount,
                0b11 => (instruction >> shift_amount) | (instruction << (32 - shift_amount)),
                _ => unreachable!()
            }
        } else {
            instruction & 0xfff
        };

        let signed_offset = if positive_offset { offset as i32 } else { -(offset as i32) };
        let address = (base as i32 + if pre_indexing { signed_offset } else { 0 }) as u32;

        if byte_transfer {
            if is_load {
                registers.set_reg_no_flags(sd_reg as usize, memory.load_byte(address) as u32);
            } else {
                memory.store_byte(address, registers.reg(sd_reg as usize) as u8);
            }
        } else {
            if is_load {
                registers.set_reg_no_flags(sd_reg as usize, memory.load(address));
            } else {
                memory.store(address, registers.reg(sd_reg as usize));
            }
        }

        if write_back {
            registers.set_reg(base_reg as usize, address + if pre_indexing { 0 } else { offset });
        }
    }

    fn stringify(&self, address: u32, cond: &str, instruction: u32) -> String {
        const SHIFT_MNEMONICS: [&str; 4] = ["LSL", "LSR", "ASR", "ROR"];

        let op2_is_reg = (instruction & 1 << 25) != 0;
        let pre_indexing = (instruction & 1 << 24) != 0;
        let base_reg = (instruction >> 16) & 0xf;
        let sd_reg = (instruction >> 12) & 0xf;

        let mnemonic = if instruction & 1 << 20 != 0 { "LDR" } else { "STR" };
        let b = if instruction & 1 << 22 != 0 { "B" } else { "" };
        let minus = if instruction & 1 << 23 != 0 { "" } else { "-" };
        let pling = if instruction & 1 << 21 != 0 { "!" } else { "" };

        let op2 = if op2_is_reg {
            let shift_amount = (instruction >> 7) & 0x1f;
            let shift_type = (instruction >> 5) & 0x3;
            let op2_reg = instruction & 0xf;

            let shift = if shift_amount > 0 {
                format!(" {} {}", SHIFT_MNEMONICS[shift_type as usize], shift_amount)
            } else {
                "".to_owned()
            };

            if pre_indexing {
                format!("[R{}, {}R{}{}]{}", base_reg, minus, op2_reg, shift, pling)
            } else {
                format!("[R{}], {}R{}{}", base_reg, minus, op2_reg, shift)
            }
        } else {
            let offset = instruction & 0xfff;

            if pre_indexing {
                if offset == 0 {
                    format!("[R{}]{}", base_reg, pling)
                } else {
                    format!("[R{}, {}{}]{}", base_reg, minus, offset, pling)
                }
            } else {
                if offset == 0 {
                    format!("[R{}]", base_reg)
                } else {
                    format!("[R{}], {}{}", base_reg, minus, offset)
                }
            }
        };

        format!("{}{}{} R{}, {}", mnemonic, cond, b, sd_reg, op2)
    }
}

//class LoadStoreInstruction implements Instruction {
//    stringify(address: number, cond: string, instruction: number): string {
//    }
