use webarc::cpu::Cpu;

#[no_mangle]
pub fn boot(rom: Box<[u32]>) {
    let mut cpu = Cpu::new(rom);
    cpu.run();
}
