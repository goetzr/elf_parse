use my_elf_lib::{ElfParser, ElfResult};
use std::env;

fn main() {
    if let Err(e) = run() {
        eprintln!("ERROR: {e}");
    }
}

fn run() -> ElfResult<()> {
    let file_path = env::args().skip(1).next().unwrap();
    let mut parser = ElfParser::new();
    let elf = parser.parse(file_path)?;
    println!("ELF identification: {:?}", elf.ident);
    Ok(())
}
