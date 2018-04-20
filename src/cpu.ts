import { InstructionDecoder } from './instructions';
import { RegisterFile } from './registers';
import { Memory } from './memory';

const RESET_VECTOR = 0x00;
const UNDEFINED_INSTRUCTION_VECTOR = 0x04;
const SWI_VECTOR = 0x08;
const PREFETCH_ABORT_VECTOR = 0x0c;
const DATA_ABORT_VECTOR = 0x10;
const ADDRESS_EXCEPTION_VECTOR = 0x14;
const IRQ_VECTOR = 0x18;
const FIRQ_VECTOR = 0x1c;

const N_BIT = 1 << 31;
const Z_BIT = 1 << 30;
const C_BIT = 1 << 29;
const V_BIT = 1 << 28;

class UndefinedInstruction {}

export class Cpu {
    registers = new RegisterFile();
    memory: Memory;

    private decoder = new InstructionDecoder();

    constructor(romData: ArrayBuffer) {
        this.memory = new Memory(romData);

        // Start executing from the reset vector (accounting for pipeline offset)
        this.registers.set(15, RESET_VECTOR + 8);
    }

    step() {
        let fetchAddress = this.registers.pc - 8;

        let fetchedWord = this.memory.load(fetchAddress);
        console.debug(`Fetched: ${fetchedWord.toString(16)}`);

        let instruction = this.decoder.decode(fetchedWord);

        let cond = fetchedWord >>> 28;

        let pcBefore = this.registers.pc;
        if (Cpu.conditionMet(cond, this.registers.get(15))) {
            if (instruction === null) {
                Cpu.undefinedInstruction();
            } else {
                console.debug(`0x${fetchAddress.toString(16)} ${instruction.stringify(fetchAddress, fetchedWord)}`);
                instruction.exec(fetchedWord, this);
            }
        } else {
            console.debug("Skipping because of unmet condition")
        }

        if (this.registers.pc == pcBefore) {
            this.registers.pc += 4;
        } else {
            this.registers.pc += 8;
        }
    }

    private static conditionMet(cond: number, r15: number): boolean {
        let masked;

        console.debug("cond", cond);

        switch (cond) {
            case 0x0: /* EQ */ return (r15 & Z_BIT) != 0;
            case 0x1: /* NE */ return (r15 & Z_BIT) == 0;
            case 0x2: /* CS */ return (r15 & C_BIT) != 0;
            case 0x3: /* CC */ return (r15 & C_BIT) == 0;
            case 0x4: /* MI */ return (r15 & N_BIT) != 0;
            case 0x5: /* PL */ return (r15 & N_BIT) == 0;
            case 0x6: /* VS */ return (r15 & Z_BIT) != 0;
            case 0x7: /* VC */ return (r15 & Z_BIT) == 0;
            case 0x8: /* HI */ return (r15 & (C_BIT | Z_BIT)) == C_BIT;
            case 0x9: /* LS */ return (r15 & (C_BIT | Z_BIT)) != 0;
            case 0xA: /* GE */
                masked = r15 & (N_BIT | V_BIT);
                return masked == (N_BIT | V_BIT) || masked == 0;
            case 0xB: /* LT */
                masked = r15 & (N_BIT | V_BIT);
                return masked == N_BIT || masked == V_BIT;
            case 0xC: /* GT */
                masked = r15 & (Z_BIT | N_BIT | V_BIT);
                return masked == N_BIT || masked == V_BIT || masked == Z_BIT || masked == 0;
            case 0xD: /* LE */
                masked = r15 & (N_BIT | V_BIT);
                return ((r15 & Z_BIT) == Z_BIT) || masked == N_BIT || masked == V_BIT;
            case 0xE: /* AL */ return true;
            case 0xF: /* NV */ return false;
        }
    }

    private static undefinedInstruction(): void {
        throw new UndefinedInstruction;
    }
}
