use std::ops::Shr;
use webarc::memory::Memory;
use webarc::registers::RegisterFile;
use webarc::instructions::*;

const RESET_VECTOR: u32 = 0x00;
const UNDEFINED_INSTRUCTION_VECTOR: u32 = 0x04;
const SWI_VECTOR: u32 = 0x08;
const PREFETCH_ABORT_VECTOR: u32 = 0x0c;
const DATA_ABORT_VECTOR: u32 = 0x10;
const ADDRESS_EXCEPTION_VECTOR: u32 = 0x14;
const IRQ_VECTOR: u32 = 0x18;
const FIRQ_VECTOR: u32 = 0x1c;

const N_BIT: u32 = 0x80000000;
const Z_BIT: u32 = 0x40000000;
const C_BIT: u32 = 0x20000000;
const V_BIT: u32 = 0x10000000;

pub struct Cpu {
    pub registers: RegisterFile,
    pub memory: Memory,
}

impl Cpu {
    pub fn new(rom: Box<[u32]>) -> Cpu {
        let mut cpu = Cpu {
            registers: RegisterFile::new(),
            memory: Memory::new(rom),
        };

        // Start executing from the reset vector (accounting for pipeline offset)
        cpu.registers.set_reg(15, RESET_VECTOR + 8);
        cpu
    }

    pub fn run(&mut self) {
        loop {
            self.step();
        }
    }

    pub fn step(&mut self) {
        let fetch_address = self.registers.reg_no_flags(15) - 8;
        let fetched_word = self.memory.load(fetch_address);
        let cond = fetched_word.shr(28) as u8;
        self.log_instruction(fetch_address, fetched_word, cond, fetched_word);

        let action = if self.condition_met(cond, self.registers.reg(15)) {
            exec(&mut self.registers, &mut self.memory, fetched_word)
        } else {
            Action::Continue
        };

        let pc_increment = match action {
            // Increment PC to next instruction
            Action::Continue => 4,

            // Pipeline would usually be flushed here, but we don't have one
            // PC has the address of the next instruction, so add 8 to simulate pipeline's effect
            Action::Flush => 8,
        };

        let new_pc = self.registers.reg_no_flags(15) + pc_increment;
        self.registers.set_reg_no_flags(15, new_pc);
    }

    fn condition_met(&self, cond: u8, r15: u32) -> bool {
        match cond {
            0x0 /* EQ */ => (r15 & Z_BIT) != 0,
            0x1 /* NE */ => (r15 & Z_BIT) == 0,
            0x2 /* CS */ => (r15 & C_BIT) != 0,
            0x3 /* CC */ => (r15 & C_BIT) == 0,
            0x4 /* MI */ => (r15 & N_BIT) != 0,
            0x5 /* PL */ => (r15 & N_BIT) == 0,
            0x6 /* VS */ => (r15 & Z_BIT) != 0,
            0x7 /* VC */ => (r15 & Z_BIT) == 0,
            0x8 /* HI */ => (r15 & (C_BIT | Z_BIT)) == C_BIT,
            0x9 /* LS */ => (r15 & (C_BIT | Z_BIT)) != 0,
            0xA /* GE */ => {
                let masked = r15 & (N_BIT | V_BIT);
                masked == (N_BIT | V_BIT) || masked == 0
            },
            0xB /* LT */ => {
                let masked = r15 & (N_BIT | V_BIT);
                masked == N_BIT || masked == V_BIT
            },
            0xC /* GT */ => {
                let masked = r15 & (Z_BIT | N_BIT | V_BIT);
                masked == N_BIT || masked == V_BIT || masked == Z_BIT || masked == 0
            },
            0xD /* LE */ => {
                let masked = r15 & (N_BIT | V_BIT);
                ((r15 & Z_BIT) == Z_BIT) || masked == N_BIT || masked == V_BIT
            },
            0xE /* AL */ => true,
            0xF /* NV */ => false,
            _ => unreachable!()
        }
    }

    fn log(&self, s: String) {
        println!("{}", s);
    }

    fn log_instruction(&self, fetch_address: u32, fetched_word: u32, cond: u8, instruction: u32) {
        let stringified = format(fetch_address, cond, instruction);

        let log_line = format!("{:08X}  {:08X}  {}", fetch_address, fetched_word, stringified);
        self.log(log_line);
    }
}
