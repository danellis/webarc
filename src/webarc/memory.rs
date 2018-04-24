use std::ops::Shl;
use std::ops::Shr;

pub struct Memory {
    ram: Box<[u32]>,
    rom: Box<[u32]>,
    rom_mapped: bool
}

impl Memory {
    pub fn new(rom: Box<[u32]>) -> Memory {
        Memory {
            ram: vec![0; 1 * 1024 * 1024].into_boxed_slice(),
            rom,
            rom_mapped: true
        }
        // console.debug('ROM size: 0x' + this.rom.byteLength.toString(16));
    }

    pub fn load(&mut self, address: u32) -> u32 {
        let masked_address = address & 0x03fffffc;

        // Logically mapped RAM unless ROM is mapped low
        if masked_address < 0x02000000 {
            if self.rom_mapped {
                // console.debug("Fetching from ROM mapped low");
                self.rom[(masked_address / 4) as usize]
            } else {
                // This should be mapped logical-to-physical
                // console.debug("Fetching from logical RAM");
                self.ram[(masked_address / 4) as usize]
            }
        } else if masked_address < 0x03000000 {
            // Physically mapped RAM
            // console.debug("Fetching from physical RAM");
            self.ram[((masked_address - 0x02000000) / 4) as usize]
        } else if masked_address < 0x03400000 {
            // console.debug("Fetching from I/O controllers");
            unimplemented!()
        } else if masked_address < 0x03800000 {
            // console.debug("Fetching from low ROM");
            self.rom_mapped = false;
            unimplemented!()
        } else {
            // High ROM
            self.rom_mapped = false;
            self.rom[((masked_address - 0x03800000) / 4) as usize]
        }
    }

    pub fn store(&self, _address: u32, _data: u32) {
//        let masked_address = address & 0x03fffffc;
        unimplemented!()
    }

    pub fn load_byte(&mut self, address: u32) -> u8 {
        let word = self.load(address);
        let field = address & 0x00000003;
        (word.shr(field * 8) & 0xff) as u8
    }

    pub fn store_byte(&mut self, address: u32, data: u8) {
        let field = (address & 0x00000003) as u8;
        let word = (data & 0xff).shl(field * 8);
        self.store(address, word as u32);
    }
}
