// Validate a runtime ELF object and emit self-contained NASM embedding tables.
use std::{env, fs, process};
#[path = "runtime/elf.rs"] mod elf;
#[path = "runtime/pack.rs"] mod pack;

// Keep the standalone packer independent of compiler and host configuration.
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 2 {
        eprintln!("usage: pack_runtime <runtime.o> <runtime_pack.inc>");
        process::exit(2);
    }
    let output = std::path::Path::new(&args[1]);
    let temporary = output.with_extension(format!("tmp_{}", process::id()));
    let result = fs::read(&args[0]).and_then(|bytes| pack::encode(&bytes))
        .and_then(|bytes| fs::write(&temporary, bytes))
        .and_then(|()| fs::rename(&temporary, output));
    if result.is_err() { let _ = fs::remove_file(temporary); }
    if let Err(error) = result {
        eprintln!("runtime pack: {error}");
        process::exit(1);
    }
}
