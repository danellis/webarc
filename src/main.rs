mod webarc;
mod boot;

use std::fs::File;
use std::io::prelude::*;
use boot::boot;

#[cfg(not(target_os = "emscripten"))]
fn main() {
    println!("WebArc (native)");
    let mut rom: Vec<u32> = vec![0; 1 * 1024 * 1024];

    println!("Opening ROM file");
    let mut f = File::open("dist/riscos311.rom").expect("Couldn't open ROM file");

    println!("Reading ROM file");
    f.read(as_u8_slice(&mut rom));

    boot::boot(rom.into_boxed_slice());
}

#[cfg(target_os = "emscripten")]
fn main() {
    println!("WebArc (WebAssembly)");
}

fn as_u8_slice(v: &mut[u32]) -> &mut[u8] {
    unsafe {
        std::slice::from_raw_parts_mut(
            v.as_mut_ptr() as *mut u8,
            v.len() * std::mem::size_of::<i32>(),
        )
    }
}
