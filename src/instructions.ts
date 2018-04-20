import { Cpu } from './cpu';

export interface Instruction {
    exec(instruction: number, cpu: Cpu): void
    stringify(address: number, instruction: number): string
}

export class InstructionDecoder {
    private branch = new BranchInstruction;
    private alu = new AluInstruction;

    decode(word: number): Instruction {
        if ((word & 0x0e000000) === 0x0a000000) return this.branch;
        if ((word & 0x0c000000) === 0x00000000) return this.alu;
        return null;
    }
}

class BranchInstruction implements Instruction {
    exec(instruction: number, cpu: Cpu): void {
        let pc = cpu.registers.pc;

        if (instruction & 1 << 24) {
            cpu.registers.set(14, pc);
        }

        let offset = (instruction & 0x00ffffff) << 2;
        console.debug("Offset = 0x" + offset.toString(16));
        cpu.registers.pc = (pc + offset + 8) & 0x03ffffff;
    }

    stringify(address: number, instruction: number): string {
        let offset = (instruction & 0x00ffffff) << 2;
        let dest = (address + offset + 8) & 0x03ffffff;

        if (instruction & 1 << 24) {
            return 'BL 0x' + dest.toString(16);
        }

        return 'B 0x' + dest.toString(16);
    }
}

class AluInstruction implements Instruction {
    exec(instruction: number, cpu: Cpu): void {

    }

    stringify(address: number, instruction: number): string {
        return "ALU instruction"
    }
}
