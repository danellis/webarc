use std::ops::Shl;
use webarc::registers::RegisterFile;
use webarc::memory::Memory;

pub trait Instruction {
    fn exec(&self, registers: &mut RegisterFile, memory: &mut Memory, instruction: u32);
    fn stringify(&self, address: u32, cond: &str, instruction: u32) -> String;
}

pub struct InstructionDecoder {
    branch: BranchInstruction,
//    alu: AluInstruction,
//    load_store: LoadStoreInstruction
}

impl InstructionDecoder {
    pub fn new() -> InstructionDecoder {
        InstructionDecoder {
            branch: BranchInstruction {},
//            AluInstruction::new(),
//            LoadStoreInstruction::new()
        }
    }

    pub fn decode(&self, word: u32) -> Option<&Instruction> {
        if (word & 0x0e000000) == 0x0a000000 {
            Some(&self.branch)
        } /* else if (word & 0x0c000000) == 0x00000000 {
            Some(self.alu)
        } else if (word & 0x0c000000) == 0x04000000 {
            Some(self.load_store)
        } */ else {
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

//class AluInstruction implements Instruction {
//    exec(cpu: Cpu, instruction: number): void {
//
//    }
//
//    stringify(address: number, cond: string, instruction: number): string {
//        return `...${cond} ; ALU instruction`
//    }
//}
//
//class LoadStoreInstruction implements Instruction {
//    exec(cpu: Cpu, instruction: number): void {
//        let op2IsReg = !!(instruction & 1 << 25);
//        let preIndexing = !!(instruction & 1 << 24);
//        let positiveOffset = !!(instruction & 1 << 23);
//        let byteTransfer = !!(instruction & 1 << 22);
//        let writeBack = !!(instruction & 1 << 21) || !preIndexing;
//        let isLoad = !!(instruction & 1 << 20);
//
//        let baseReg = (instruction >> 16) & 0xf;
//        let base = cpu.registers.getWithoutPsr(baseReg);
//        let sdReg = (instruction >> 12) & 0xf;
//
//        let offset;
//        if (op2IsReg) {
//            let shiftAmount = (instruction >> 7) & 0x1f;
//            let shiftType = (instruction >> 5) & 0x3;
//            let op2Reg = instruction & 0xf;
//            let unshifted = cpu.registers.get(op2Reg);
//
//            switch (shiftType) {
//                case 0b00: offset = unshifted << shiftAmount; break;
//                case 0b01: offset = unshifted >>> shiftAmount; break;
//                case 0b10: offset = unshifted >> shiftAmount; break;
//                case 0b11:
//                    offset = (instruction >> shiftAmount) | (instruction << (32 - shiftAmount));
//                    break;
//            }
//        } else {
//            offset = instruction & 0xfff;
//        }
//
//        if (!positiveOffset) offset = -offset;
//
//        let address = base + (preIndexing ? offset : 0);
//        if (byteTransfer) {
//            if (isLoad) {
//                cpu.registers.setWithoutPsr(sdReg, cpu.memory.loadByte(address));
//            } else {
//                cpu.memory.storeByte(address, cpu.registers.get(sdReg));
//            }
//        } else {
//            if (isLoad) {
//                cpu.registers.setWithoutPsr(sdReg, cpu.memory.load(address));
//            } else {
//                cpu.memory.store(address, cpu.registers.get(sdReg));
//            }
//        }
//
//        if (writeBack) cpu.registers.set(baseReg, address + (preIndexing ? 0 : offset));
//    }
//
//    stringify(address: number, cond: string, instruction: number): string {
//        let op2IsReg = !!(instruction & 1 << 25);
//        let preIndexing = !!(instruction & 1 << 24);
//        let baseReg = (instruction >> 16) & 0xf;
//        let sdReg = (instruction >> 12) & 0xf;
//
//        let mnemonic = !!(instruction & 1 << 20) ? 'LDR' : 'STR';
//        let b = !!(instruction & 1 << 22) ? 'B' : '';
//        let minus = !!(instruction & 1 << 23) ? '' : '-';
//        let pling = !!(instruction & 1 << 21) ? '!' : '';
//
//        let op2;
//        if (op2IsReg) {
//            let shiftAmount = (instruction >> 7) & 0x1f;
//            let shiftType = (instruction >> 5) & 0x3;
//            let op2Reg = instruction & 0xf;
//
//            let shift = '';
//            if (shiftAmount > 0) {
//                switch (shiftType) {
//                    case 0b00: shift = ` LSL ${shiftAmount}`; break;
//                    case 0b01: shift = ` LSR ${shiftAmount}`; break;
//                    case 0b10: shift = ` ASR ${shiftAmount}`; break;
//                    case 0b11: shift = ` ROR ${shiftAmount}`; break;
//                }
//            }
//
//            if (preIndexing) {
//                op2 = `[R${baseReg}, ${minus}R${op2Reg}${shift}]${pling}`
//            } else {
//                op2 = `[R${baseReg}], ${minus}R${op2Reg}${shift}`
//            }
//        } else {
//            let offset = instruction & 0xfff;
//
//            if (preIndexing) {
//                if (offset == 0) {
//                    op2 = `[R${baseReg}]${pling}`;
//                } else {
//                    op2 = `[R${baseReg}, ${minus}${offset}]${pling}`;
//                }
//            } else {
//                if (offset == 0) {
//                    op2 = `[R${baseReg}]`;
//                } else {
//                    op2 = `[R${baseReg}], ${minus}${offset}`;
//                }
//            }
//        }
//
//        return `${mnemonic}${cond}${b} R${sdReg}, ${op2}`;
//    }
