// A rejected compilation must report its code and preserve an existing output.
use std::{fs, path::Path};

use super::build;
use crate::oracle;

pub fn execute(compiler: &Path, source: &Path, directory: &Path, code: &str) -> Result<(), String> {
    let binary = directory.join("direct.elf");
    fs::write(&binary, b"previous output").map_err(|error| error.to_string())?;
    let output = build::compile(compiler, source, &binary, false)?;
    oracle::diagnostic(&output, code)?;
    if fs::read(&binary).map_err(|error| error.to_string())? != b"previous output" {
        return Err("failed compilation damaged previous output".into());
    }
    Ok(())
}
