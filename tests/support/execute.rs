// Dispatch case expectations while retaining every failed case's input and outputs.
#[path = "execution/assembly.rs"]
mod assembly;
#[path = "execution/build.rs"]
pub(crate) mod build;
#[path = "execution/cli.rs"]
mod cli;
#[path = "execution/decimal.rs"]
mod decimal;
#[path = "execution/diagnostic.rs"]
mod diagnostic;
#[path = "execution/program.rs"]
mod program;

use std::{fs, path::Path};

use crate::{interop_runner, stdio_runner, Case, Expectation};

pub use build::invoke;

// Successful cases remove their directory; an error leaves all artifacts intact.
pub fn execute(case: Case, compiler: &Path, root: &Path) -> Result<(), String> {
    let directory = root.join(&case.id);
    fs::create_dir(&directory).map_err(|error| error.to_string())?;
    for (name, content) in &case.files {
        fs::write(directory.join(name), content).map_err(|error| error.to_string())?;
    }
    let source = directory.join("input.c");
    fs::write(&source, &case.source).map_err(|error| error.to_string())?;

    match &case.expectation {
        Expectation::Exit(status) => program::execute(compiler, &source, &directory, *status)?,
        Expectation::Reject(code) => diagnostic::execute(compiler, &source, &directory, code)?,
        Expectation::Io(input, expected, status) => stdio_runner::execute(
            compiler,
            &source,
            &directory,
            input,
            expected,
            Some(*status),
        )?,
        Expectation::IoTimeout(input, expected) => {
            stdio_runner::execute(compiler, &source, &directory, input, expected, None)?
        }
        Expectation::CInterop(foreign) => {
            interop_runner::execute(compiler, &source, foreign, &directory)?
        }
        Expectation::AssemblyHarness(harness) => {
            assembly::execute(compiler, &source, &directory, harness)?
        }
        Expectation::Decimal(value, format, low, high, status) => {
            decimal::execute(&directory, value, *format, *low, *high, *status)?
        }
        Expectation::Cli(args, status, stdout) => cli::execute(compiler, args, *status, stdout)?,
        Expectation::CliVersion(args) => cli::version(compiler, args)?,
    }

    fs::remove_dir_all(&directory).map_err(|error| error.to_string())?;
    Ok(())
}
