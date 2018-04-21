export enum Mode {
    User = 0,
    Firq = 1,
    Irq = 2,
    Svc = 3
}

export class RegisterFile {
    /* Register layout:
         0  R0
         1  R1
         2  R2
         3  R3
         4  R4
         5  R5
         6  R6
         7  R7
         8  R8
         9  R9
         10 R10
         11 R11
         12 R12
         13 R13
         14 R14
         15 R15
         16 R8_FIRQ
         17 R9_FIRQ
         18 R10_FIRQ
         19 R11_FIRQ
         20 R12_FIRQ
         21 R13_FIRQ
         22 R14_FIRQ
         23 R13_IRQ
         24 R14_IRQ
         25 R13_SVC
         26 R14_SVC
    */
    buffer = new ArrayBuffer(108);
    reg32 = new Uint32Array(this.buffer);

    private offset(reg: number): number {
        if (reg == 15) return 15;

        let mode = this.mode;

        if (mode == 0) return reg;
        if (mode == 1) return reg >= 8 ? reg + 8 : reg;
        if (mode == 2) return reg >= 13 ? reg + 10 : reg;

        return reg >= 13 ? reg + 12 : reg;
    }

    set(reg: number, value: number): void {
        this.reg32[this.offset(reg)] = value;
    }

    get(reg: number): number {
        return this.reg32[this.offset(reg)];
    }

    setWithoutPsr(reg: number, value: number): void {
        reg == 15 ? this.pc = value : this.set(reg, value);
    }

    getWithoutPsr(reg: number): number {
        return reg == 15 ? this.pc : this.get(reg);
    }

    get pc(): number {
        return this.reg32[15] & 0x03fffffc;
    }

    set pc(address: number) {
        this.reg32[15] = (this.reg32[15] & ~0x03fffffc) | (address & 0x03fffffc);
    }

    get mode(): Mode {
        return this.reg32[15] & 3;
    }
}
