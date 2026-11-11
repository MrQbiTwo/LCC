// Shared three-second compiler, assembler, linker, and program invocations.
use std::{path::Path, time::Duration};

use crate::{oracle, process};

pub fn invoke(program: &Path, args: &[String]) -> Result<process::Output, String> {
    process::invoke(program, args, Vec::new(), Duration::from_secs(3))
}

pub fn argument(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

pub fn compile(
    compiler: &Path,
    source: &Path,
    output: &Path,
    assembly: bool,
) -> Result<process::Output, String> {
    let mut args = vec![argument(source), "-o".into(), argument(output)];
    if assembly {
        args.push("-S".into());
    }
    invoke(compiler, &args)
}

pub fn assemble(source: &Path, object: &Path, includes: &[String]) -> Result<(), String> {
    let mut args = vec!["-f".into(), "elf64".into()];
    for include in includes {
        args.extend(["-I".into(), include.clone()]);
    }
    args.extend([argument(source), "-o".into(), argument(object)]);
    oracle::exit(&invoke(Path::new("nasm"), &args)?, 0)
}

pub fn link(binary: &Path, objects: &[&Path]) -> Result<(), String> {
    let mut args = vec![
        "-z".into(),
        "noexecstack".into(),
        "-o".into(),
        argument(binary),
    ];
    args.extend(objects.iter().map(|object| argument(object)));
    oracle::exit(&invoke(Path::new("ld"), &args)?, 0)
}
