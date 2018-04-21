import { Instruction, InstructionDecoder } from './instructions';
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

const COND_STRINGS = ['EQ', 'NE', 'CS', 'CC', 'MI', 'PL', 'VS', 'VC', 'HI', 'LS', 'GE', 'LT', 'GT', 'LE', '', 'NV'];

class UndefinedInstruction {}

export class Cpu {
    registers = new RegisterFile();
    memory: Memory;
    debugElement: HTMLElement;

    private decoder = new InstructionDecoder();

    constructor(romData: ArrayBuffer) {
        this.debugElement = document.getElementById('debugoutput');
        this.memory = new Memory(romData);

        // Start executing from the reset vector (accounting for pipeline offset)
        this.registers.set(15, RESET_VECTOR + 8);
    }

    run() {
        for (;;) this.step();
    }

    step() {
        let fetchAddress = this.registers.pc - 8;
        let fetchedWord = this.memory.load(fetchAddress);
        let instruction = this.decoder.decode(fetchedWord);
        let cond = fetchedWord >>> 28;
        this.logInstruction(fetchAddress, fetchedWord, cond, instruction);

        let pcBefore = this.registers.pc;

        if (Cpu.conditionMet(cond, this.registers.get(15))) {
            if (instruction === null) {
                Cpu.undefinedInstruction();
            } else {
                instruction.exec(this, fetchedWord);
            }
        }

        if (this.registers.pc == pcBefore) {
            // Increment PC to next instruction
            this.registers.pc += 4;
        } else {
            // Pipeline would usually be flushed here, but we don't have one
            // PC has the address of the next instruction, so add 8 to simulate pipeline's effect
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

    private static hex(n: number): string {
        return ("0000000" + n.toString(16)).substr(-8)
    }

    private log(s: string): void {
        this.debugElement.innerText += s + '\n';
    }

    private logInstruction(fetchAddress: number, fetchedWord: number, cond: number, instruction: Instruction): void {
        let condString = COND_STRINGS[cond];
        let stringified = instruction ? instruction.stringify(fetchAddress, condString, fetchedWord) : `...${condString}`;
        let logLine = `${Cpu.hex(fetchAddress)}  ${Cpu.hex(fetchedWord)}  ${stringified}`;
        console.debug(logLine);
        this.log(logLine);
    }
}
