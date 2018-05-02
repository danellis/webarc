use std::ops::Shl;
use std::ops::Shr;
use webarc::registers::RegisterFile;
use webarc::memory::Memory;

pub enum Action {
    Continue,
    Flush
}

const COND_STRINGS: [&str; 16] = ["EQ", "NE", "CS", "CC", "MI", "PL", "VS", "VC", "HI", "LS", "GE", "LT", "GT", "LE", "", "NV"];

pub fn exec(registers: &mut RegisterFile, memory: &mut Memory, instruction: u32) -> Action {
    let offset = ((instruction >> 24) & 0xf) as usize;
    let handler = INSTRUCTION_HANDLERS[offset];
    handler(registers, memory, instruction)
}

pub fn format(address: u32, cond: u8, instruction: u32) -> String {
    let offset = ((instruction >> 24) & 0xf) as usize;
    let formatter = INSTRUCTION_FORMATTERS[offset];
    let cond_string = COND_STRINGS[cond as usize];
    return formatter(address, cond_string, instruction);
}

// Branch

fn exec_branch(registers: &mut RegisterFile, memory: &mut Memory, instruction: u32) -> Action {
    let pc = registers.reg_no_flags(15);

    if instruction & 0x01000000 != 0 {
        registers.set_reg(14, pc);
    }

    let offset = (instruction & 0x00ffffff).shl(2);
    // console.debug("Offset = 0x" + offset.toString(16));
    registers.set_reg_no_flags(15, (pc + offset) & 0x03ffffffu32);
    Action::Flush
}

fn format_branch(address: u32, cond: &str, instruction: u32) -> String {
    let offset = u32::shl(instruction & 0x00ffffff, 2);
    let dest = (address + offset + 8u32) & 0x03ffffffu32;

    if instruction & 0x01000000 != 0 {
        format!("BL{} {:0X}", cond, dest)
    } else {
        format!("B{} ${:0X}", cond, dest)
    }
}

// ALU operations
type AluInstructionHandler = fn(registers: &mut RegisterFile, memory: &mut Memory, rd: u32, op1: u32, op2: u32) -> Action;

fn exec_alu(registers: &mut RegisterFile, memory: &mut Memory, instruction: u32) -> Action {
    const ALU_INSTRUCTION_HANDLERS: [AluInstructionHandler; 16] = [
        exec_alu_unimplemented,
        exec_alu_unimplemented,
        exec_alu_unimplemented,
        exec_alu_unimplemented,
        exec_alu_add,
        exec_alu_unimplemented,
        exec_alu_unimplemented,
        exec_alu_unimplemented,
        exec_alu_unimplemented,
        exec_alu_unimplemented,
        exec_alu_unimplemented,
        exec_alu_unimplemented,
        exec_alu_unimplemented,
        exec_alu_unimplemented,
        exec_alu_unimplemented,
        exec_alu_unimplemented,
    ];

    let opcode = (instruction >> 21) & 0xf;
    println!("ALU opcode: {}", opcode);

    let rd = (instruction >> 12) & 0xf;

    let rn = (instruction >> 16) & 0xf;

    let immediate = (instruction >> 25) & 1 != 0;

    let (op1, op2) = if immediate {
        let value = instruction & 0xff;
        let rotate = ((instruction >> 8) & 0xf) * 2;
        (registers.reg_no_flags(rn), value.rotate_right(rotate))
    } else {
        let rm = instruction & 0xf;

        let shift_type = (instruction >> 5) & 0b11;

        let (op1, shift_amount, unshifted) = if instruction & (1 << 4) != 0 {
            let rs = (instruction >> 8) & 0xf;
            (
                registers.reg_no_flags(rn) + if rn == 15 { 4 } else { 0 },
                registers.reg_no_flags(rs) & 0xff,
                registers.reg(rm) + if rm == 15 { 4 } else { 0 }
            )
        } else {
            (
                registers.reg_no_flags(rn),
                (instruction >> 7) & 0b11111,
                registers.reg(rm)
            )
        };

        let op2 = match shift_type {
            0b00 => unshifted << shift_amount,
            0b01 => (unshifted as i32 >> shift_amount) as u32,
            0b10 => unshifted >> shift_amount,
            0b11 => (instruction >> shift_amount) | (instruction << (32 - shift_amount)),
            _ => unreachable!()
        };

        println!("op1 = {:08X}, op2 = {:08X}", op1, op2);
        (op1, op2)
    };

    ALU_INSTRUCTION_HANDLERS[opcode as usize](registers, memory, rd, op1, op2)
}

fn format_alu(address: u32, cond: &str, instruction: u32) -> String {
    const ALU_OPS: [&str; 16] = [
        "AND", "EOR", "SUB", "RSB", "ADD", "ADC", "SBC", "RSC",
        "TST", "TEQ", "CMP", "CMN", "ORR", "MOV", "BIC", "MVN"
    ];

    let opcode = ALU_OPS[((instruction >> 21) & 0xf) as usize];
    let rd = (instruction >> 12) & 0xf; // TODO: Change to PC if it's 15
    let rn = (instruction >> 16) & 0xf;

    let immediate = (instruction >> 25) & 1 != 0;

    let op2 = if immediate {
        let value = instruction & 0xff;
        let rotate = ((instruction >> 8) & 0xf) * 2;
        let rotate_text = if rotate > 0 { format!("ROR {}", rotate) } else { "".to_string() };
        format!("#{} {}", value, rotate_text)
    } else {
        let rm = instruction & 0xf;

        let shift_amount = if instruction & (1 << 4) != 0 {
            let rs = (instruction >> 8) & 0xf;
            format!("R{}", rs)
        } else {
            let imm = (instruction >> 7) & 0b11111;
            format!("#{}", imm)
        };

        let shift_type = match (instruction >> 5) & 0b11 {
            0b00 => "LSL",
            0b01 => "ASR",
            0b10 => "LSR",
            0b11 => "ROR",
            _ => unreachable!()
        };

        format!("R{} {} {}", rm, shift_type, shift_amount)
    };

    format!("{}{} R{}, R{}, {} ; ALU instruction", opcode, cond, rd, rn, op2)
}

fn exec_alu_add(registers: &mut RegisterFile, memory: &mut Memory, rd: u32, op1: u32, op2: u32) -> Action {
    registers.set_reg(rd, op1 + op2);
    if rd == 15 { Action::Flush } else { Action::Continue }
}

fn exec_alu_unimplemented(registers: &mut RegisterFile, memory: &mut Memory, rd: u32, op1: u32, op2: u32) -> Action {
    exec_unimplemented(registers, memory, 0)
}

// LDR/STR

fn exec_ldr_str(registers: &mut RegisterFile, memory: &mut Memory, instruction: u32) -> Action {
    let op2_is_reg = (instruction & 1 << 25) != 0;
    let pre_indexing = (instruction & 1 << 24) != 0;
    let positive_offset = (instruction & 1 << 23) != 0;
    let byte_transfer = (instruction & 1 << 22) != 0;
    let write_back = (instruction & 1 << 21) != 0 || !pre_indexing;
    let is_load = (instruction & 1 << 20) != 0;

    let base_reg = (instruction >> 16) & 0xf;
    let base = registers.reg_no_flags(base_reg);
    let sd_reg = (instruction >> 12) & 0xf;

    let offset= if op2_is_reg {
        let shift_amount = (instruction >> 7) & 0x1f;
        let shift_type = (instruction >> 5) & 0x3;
        let op2_reg = instruction & 0xf;
        let unshifted = registers.reg(op2_reg);

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
            registers.set_reg_no_flags(sd_reg, memory.load_byte(address) as u32);
        } else {
            memory.store_byte(address, registers.reg(sd_reg) as u8);
        }
    } else {
        if is_load {
            registers.set_reg_no_flags(sd_reg, memory.load(address));
        } else {
            memory.store(address, registers.reg(sd_reg));
        }
    }

    if write_back {
        registers.set_reg(base_reg, address + if pre_indexing { 0 } else { offset });
    }

    if is_load && sd_reg == 15 { Action::Flush } else { Action::Continue}
}

fn format_ldr_str(address: u32, cond: &str, instruction: u32) -> String {
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

//class LoadStoreInstruction implements Instruction {
//    stringify(address: number, cond: string, instruction: number): string {
//    }

fn exec_unimplemented(registers: &mut RegisterFile, memory: &mut Memory, instruction: u32) -> Action {
    unimplemented!();
}

fn format_unimplemented(address: u32, cond: &str, instruction: u32) -> String {
    format!("...{}", cond)
}

type InstructionHandler = fn(registers: &mut RegisterFile, memory: &mut Memory, instruction: u32) -> Action;
type InstructionFormatter = fn(address: u32, cond: &str, instruction: u32) -> String;

const INSTRUCTION_HANDLERS: [InstructionHandler; 16] = [
    exec_alu,
    exec_alu,
    exec_alu,
    exec_alu,
    exec_ldr_str,
    exec_ldr_str,
    exec_ldr_str,
    exec_ldr_str,
    exec_unimplemented,
    exec_unimplemented,
    exec_branch,
    exec_branch,
    exec_unimplemented,
    exec_unimplemented,
    exec_unimplemented,
    exec_unimplemented,
];

const INSTRUCTION_FORMATTERS: [InstructionFormatter; 16] = [
    format_alu,
    format_alu,
    format_alu,
    format_alu,
    format_ldr_str,
    format_ldr_str,
    format_ldr_str,
    format_ldr_str,
    format_unimplemented,
    format_unimplemented,
    format_branch,
    format_branch,
    format_unimplemented,
    format_unimplemented,
    format_unimplemented,
    format_unimplemented,
];
