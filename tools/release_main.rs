// Produce standalone compiler artifacts for the implemented Linux x86-64 target.
use std::{
    env, io,
    path::{Path, PathBuf},
    process::{self, Command},
};
#[path = "runtime/build.rs"]
mod build;
#[path = "runtime/elf.rs"]
mod elf;
#[path = "runtime/pack.rs"]
mod pack;

// Report a contextual tooling error without shell command interpolation.
fn problem(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}

// Run one build command and retain the actual tool diagnostic on failure.
fn run(tool: &str, args: &[&std::ffi::OsStr], directory: &Path) -> io::Result<Vec<u8>> {
    let output = Command::new(tool)
        .args(args)
        .current_dir(directory)
        .env_remove("NASMENV")
        .output()?;
    if !output.status.success() {
        return Err(problem(format!(
            "{tool} failed ({}): {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        )));
    }
    Ok(output.stdout)
}

#[path = "release/package.rs"]
mod package;
#[path = "release/source.rs"]
mod source;
use package::package;

// CLI only packages the current supported target; it never publishes or edits versions.
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if !(2..=4).contains(&args.len()) {
        eprintln!("usage: release_main <source_root> <new_output_directory> [nasm] [ld]");
        process::exit(2);
    }
    let root = PathBuf::from(&args[0]);
    let out = PathBuf::from(&args[1]);
    if let Err(error) = package(
        &root,
        &out,
        args.get(2).map_or("nasm", String::as_str),
        args.get(3).map_or("ld", String::as_str),
    ) {
        eprintln!("package: {error}");
        process::exit(1);
    }
    println!("Linux x86-64 compiler artifacts: {}", out.display());
}

#[cfg(test)]
#[path = "release/tests.rs"]
mod tests;
