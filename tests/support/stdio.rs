// Compile transcript cases in both modes and check exact streams with deadlines.
use crate::{execution::build, oracle, process};
use std::{fs, path::Path, time::Duration};

// Runtime behavior must match in direct ELF and independently assembled NASM.
pub fn execute(
    compiler: &Path,
    source: &Path,
    directory: &Path,
    input: &str,
    expected: &str,
    status: Option<i32>,
) -> Result<(), String> {
    for mode in ["direct", "assembled"] {
        let binary = directory.join(format!("{mode}.elf"));
        let assembly = directory.join("output.asm");
        let object = directory.join("output.o");
        let output = if mode == "direct" { &binary } else { &assembly };
        oracle::exit(
            &build::compile(compiler, source, output, mode == "assembled")?,
            0,
        )?;
        if mode == "assembled" {
            build::assemble(&assembly, &object, &[])?;
            build::link(&binary, &[&object])?;
        }
        oracle::elf(&fs::read(&binary).map_err(|error| error.to_string())?)?;
        let timeout = if status.is_some() {
            Duration::from_secs(3)
        } else {
            Duration::from_millis(400)
        };
        let output = process::invoke(&binary, &[], input.as_bytes().to_vec(), timeout)?;
        if let Some(status) = status {
            oracle::exit(&output, status)?;
        } else if !output.timed_out || output.truncated {
            return Err(format!(
                "{mode}: expected bounded application EOF loop, got {output:?}"
            ));
        }
        let stdout_matches = if status.is_some() {
            output.stdout == expected.as_bytes()
        } else {
            output.stdout.starts_with(expected.as_bytes())
        };
        if !output.stderr.is_empty() || !stdout_matches {
            return Err(format!(
                "{mode}: expected stdout {expected:?}, got stdout {:?}, stderr {:?}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            ));
        }
    }
    Ok(())
}
