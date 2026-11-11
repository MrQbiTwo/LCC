// Link compiler output to an independent NASM driver for machine-level checks.
use std::{fs, path::Path};

use super::build;
use crate::oracle;

pub fn execute(
    compiler: &Path,
    source: &Path,
    directory: &Path,
    harness: &str,
) -> Result<(), String> {
    let assembly = directory.join("output.asm");
    let object = directory.join("output.o");
    let driver = directory.join("driver.asm");
    let driver_object = directory.join("driver.o");
    let binary = directory.join("direct.elf");

    fs::write(&driver, harness).map_err(|error| error.to_string())?;
    oracle::exit(&build::compile(compiler, source, &assembly, true)?, 0)?;
    for (input, output) in [(&assembly, &object), (&driver, &driver_object)] {
        build::assemble(input, output, &[])?;
    }
    build::link(&binary, &[&object, &driver_object])?;
    oracle::exit(&build::invoke(&binary, &[])?, 0)
}
