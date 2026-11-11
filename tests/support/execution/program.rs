// Compare the same source's exit result in direct ELF and independent NASM modes.
use std::{fs, path::Path};

use super::build;
use crate::oracle;

pub fn execute(
    compiler: &Path,
    source: &Path,
    directory: &Path,
    expected: i32,
) -> Result<(), String> {
    let binary = directory.join("direct.elf");
    let output = build::compile(compiler, source, &binary, false)?;
    oracle::exit(&output, 0)?;
    if !output.stdout.is_empty() || !output.stderr.is_empty() {
        return Err(format!("unexpected compiler output: {output:?}"));
    }
    oracle::elf(&fs::read(&binary).map_err(|error| error.to_string())?)?;
    oracle::exit(&build::invoke(&binary, &[])?, expected)?;

    let assembly = directory.join("output.asm");
    let object = directory.join("output.o");
    let assembled = directory.join("assembled.elf");
    oracle::exit(&build::compile(compiler, source, &assembly, true)?, 0)?;
    build::assemble(&assembly, &object, &[])?;
    build::link(&assembled, &[&object])?;
    oracle::exit(&build::invoke(&assembled, &[])?, expected)
}
