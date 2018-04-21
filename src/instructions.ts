import { Cpu } from './cpu';

export interface Instruction {
    exec(cpu: Cpu, instruction: number): void
    stringify(address: number, cond: string, instruction: number): string
}

export class InstructionDecoder {
    private branch = new BranchInstruction;
    private alu = new AluInstruction;
    private loadStore = new LoadStoreInstruction;

    decode(word: number): Instruction {
        if ((word & 0x0e000000) === 0x0a000000) return this.branch;
        if ((word & 0x0c000000) === 0x00000000) return this.alu;
        if ((word & 0x0c000000) === 0x04000000) return this.loadStore;
        return null;
    }
}

class BranchInstruction implements Instruction {
    exec(cpu: Cpu, instruction: number): void {
        let pc = cpu.registers.pc;

        if (instruction & 1 << 24) {
            cpu.registers.set(14, pc);
        }

        let offset = (instruction & 0x00ffffff) << 2;
        console.debug("Offset = 0x" + offset.toString(16));
        cpu.registers.pc = (pc + offset) & 0x03ffffff;
    }

    stringify(address: number, cond: string, instruction: number): string {
        let offset = (instruction & 0x00ffffff) << 2;
        let dest = (address + offset + 8) & 0x03ffffff;

        if (instruction & 1 << 24) {
            return `BL${cond} 0x${dest.toString(16)}`;
        }

        return `B${cond} 0x${dest.toString(16)}`;
    }
}

class AluInstruction implements Instruction {
    exec(cpu: Cpu, instruction: number): void {

    }

    stringify(address: number, cond: string, instruction: number): string {
        return `...${cond} ; ALU instruction`
    }
}

class LoadStoreInstruction implements Instruction {
    exec(cpu: Cpu, instruction: number): void {
        let op2IsReg = !!(instruction & 1 << 25);
        let preIndexing = !!(instruction & 1 << 24);
        let positiveOffset = !!(instruction & 1 << 23);
        let byteTransfer = !!(instruction & 1 << 22);
        let writeBack = !!(instruction & 1 << 21) || !preIndexing;
        let isLoad = !!(instruction & 1 << 20);

        let baseReg = (instruction >> 16) & 0xf;
        let base = cpu.registers.getWithoutPsr(baseReg);
        let sdReg = (instruction >> 12) & 0xf;

        let offset;
        if (op2IsReg) {
            let shiftAmount = (instruction >> 7) & 0x1f;
            let shiftType = (instruction >> 5) & 0x3;
            let op2Reg = instruction & 0xf;
            let unshifted = cpu.registers.get(op2Reg);

            switch (shiftType) {
                case 0b00: offset = unshifted << shiftAmount; break;
                case 0b01: offset = unshifted >>> shiftAmount; break;
                case 0b10: offset = unshifted >> shiftAmount; break;
                case 0b11:
                    offset = (instruction >> shiftAmount) | (instruction << (32 - shiftAmount));
                    break;
            }
        } else {
            offset = instruction & 0xfff;
        }

        if (!positiveOffset) offset = -offset;

        let address = base + (preIndexing ? offset : 0);
        if (byteTransfer) {
            if (isLoad) {
                cpu.registers.setWithoutPsr(sdReg, cpu.memory.loadByte(address));
            } else {
                cpu.memory.storeByte(address, cpu.registers.get(sdReg));
            }
        } else {
            if (isLoad) {
                cpu.registers.setWithoutPsr(sdReg, cpu.memory.load(address));
            } else {
                cpu.memory.store(address, cpu.registers.get(sdReg));
            }
        }

        if (writeBack) cpu.registers.set(baseReg, address + (preIndexing ? 0 : offset));
    }

    stringify(address: number, cond: string, instruction: number): string {
        let op2IsReg = !!(instruction & 1 << 25);
        let preIndexing = !!(instruction & 1 << 24);
        let baseReg = (instruction >> 16) & 0xf;
        let sdReg = (instruction >> 12) & 0xf;

        let mnemonic = !!(instruction & 1 << 20) ? 'LDR' : 'STR';
        let b = !!(instruction & 1 << 22) ? 'B' : '';
        let minus = !!(instruction & 1 << 23) ? '' : '-';
        let pling = !!(instruction & 1 << 21) ? '!' : '';

        let op2;
        if (op2IsReg) {
            let shiftAmount = (instruction >> 7) & 0x1f;
            let shiftType = (instruction >> 5) & 0x3;
            let op2Reg = instruction & 0xf;

            let shift = '';
            if (shiftAmount > 0) {
                switch (shiftType) {
                    case 0b00: shift = ` LSL ${shiftAmount}`; break;
                    case 0b01: shift = ` LSR ${shiftAmount}`; break;
                    case 0b10: shift = ` ASR ${shiftAmount}`; break;
                    case 0b11: shift = ` ROR ${shiftAmount}`; break;
                }
            }

            if (preIndexing) {
                op2 = `[R${baseReg}, ${minus}R${op2Reg}${shift}]${pling}`
            } else {
                op2 = `[R${baseReg}], ${minus}R${op2Reg}${shift}`
            }
        } else {
            let offset = instruction & 0xfff;

            if (preIndexing) {
                if (offset == 0) {
                    op2 = `[R${baseReg}]${pling}`;
                } else {
                    op2 = `[R${baseReg}, ${minus}${offset}]${pling}`;
                }
            } else {
                if (offset == 0) {
                    op2 = `[R${baseReg}]`;
                } else {
                    op2 = `[R${baseReg}], ${minus}${offset}`;
                }
            }
        }

        return `${mnemonic}${cond}${b} R${sdReg}, ${op2}`;
    }
}
