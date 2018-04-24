pub enum Mode {
    User = 0,
    Firq = 1,
    Irq = 2,
    Svc = 3
}

pub struct RegisterFile {
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

    registers: [u32; 27]
}

impl RegisterFile {
    pub fn new() -> RegisterFile {
        RegisterFile {
            registers: [0; 27]
        }
    }

    fn offset(&self, reg: usize) -> usize {
        if reg == 15 {
            15
        } else {
            match self.mode() {
                Mode::User => reg,
                Mode::Firq => if reg >= 8 { reg + 8 } else { reg },
                Mode::Irq => if reg >= 13 { reg + 10 } else { reg },
                Mode::Svc => if reg >= 13 { reg + 12 } else { reg }
            }
        }
    }

    pub fn set_reg(&mut self, reg: usize, value: u32) {
        self.registers[self.offset(reg)] = value;
    }

    pub fn reg(&self, reg: usize) -> u32 {
        self.registers[self.offset(reg)]
    }

    pub fn set_reg_no_flags(&mut self, reg: usize, value: u32) {
        if reg == 15 {
            self.registers[15] = (self.registers[15] & !0x03fffffc) | (value & 0x03fffffc);
        } else {
            self.set_reg(reg, value);
        }
    }

    pub fn reg_no_flags(&self, reg: usize) -> u32 {
        if reg == 15 {
            self.registers[15] & 0x03fffffc
        } else {
            self.reg(reg)
        }
    }

    pub fn mode(&self) -> Mode {
        match self.registers[15] & 3 {
            0 => Mode::User,
            1 => Mode::Firq,
            2 => Mode::Irq,
            3 => Mode::Svc,
            _ => unreachable!()
        }
    }
}
