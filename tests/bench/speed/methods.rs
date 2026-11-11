// Compiler commands, deadlines and independent untimed artifact verification.
use crate::process::{invoke, Output};
use std::{
    env, fs,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

pub struct Method {
    pub name: &'static str,
    pub program: PathBuf,
    pub flags: Vec<&'static str>,
    assembly: bool,
    nasm: bool,
}

impl Method {
    pub fn extension(&self) -> &'static str {
        if self.assembly {
            if self.nasm {
                "asm"
            } else {
                "s"
            }
        } else {
            "elf"
        }
    }
}

// GCC/Clang executable selection is explicit and recorded rather than assumed current.
pub fn all(lcc: PathBuf) -> Vec<Method> {
    let gcc = PathBuf::from(env::var_os("GCC").unwrap_or_else(|| "gcc".into()));
    let clang = PathBuf::from(env::var_os("CLANG").unwrap_or_else(|| "clang".into()));
    vec![
        Method {
            name: "lcc_elf",
            program: lcc.clone(),
            flags: vec![],
            assembly: false,
            nasm: true,
        },
        Method {
            name: "gcc_o0_elf",
            program: gcc.clone(),
            flags: vec!["-std=c89", "-O0", "-fno-pie", "-no-pie"],
            assembly: false,
            nasm: false,
        },
        Method {
            name: "clang_o0_elf",
            program: clang.clone(),
            flags: vec!["-std=c89", "-O0", "-fno-pie", "-no-pie"],
            assembly: false,
            nasm: false,
        },
        Method {
            name: "lcc_asm",
            program: lcc,
            flags: vec!["-S"],
            assembly: true,
            nasm: true,
        },
        Method {
            name: "gcc_o0_asm",
            program: gcc,
            flags: vec!["-std=c89", "-O0", "-fno-pie", "-S"],
            assembly: true,
            nasm: false,
        },
        Method {
            name: "clang_o0_asm",
            program: clang,
            flags: vec!["-std=c89", "-O0", "-fno-pie", "-S"],
            assembly: true,
            nasm: false,
        },
    ]
}

// Include the complete command and captured diagnostics in any retained failure.
pub fn checked(program: &Path, args: &[String], timeout: Duration) -> Result<Output, String> {
    let result = invoke(program, args, Vec::new(), timeout)
        .map_err(|error| format!("{} {args:?}: {error}", program.display()))?;
    if result.status != Some(0) || result.timed_out || result.truncated {
        return Err(format!(
            "{} {args:?}: exit={:?}, timeout={}, truncated={}\nstdout:\n{}\nstderr:\n{}",
            program.display(),
            result.status,
            result.timed_out,
            result.truncated,
            String::from_utf8_lossy(&result.stdout),
            String::from_utf8_lossy(&result.stderr)
        ));
    }
    Ok(result)
}

pub fn compile(method: &Method, source: &Path, output: &Path) -> Result<f64, String> {
    // A successful child must produce a fresh artifact, not leave an earlier one.
    match fs::remove_file(output) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(format!("remove previous owned output: {error}")),
    }
    let mut args: Vec<String> = method.flags.iter().map(|s| (*s).into()).collect();
    args.extend([
        source.display().to_string(),
        "-o".into(),
        output.display().to_string(),
    ]);
    let started = Instant::now();
    checked(&method.program, &args, Duration::from_secs(120))?;
    Ok(started.elapsed().as_secs_f64() * 1000.0)
}

// NASM and GAS results have different dialects and require their own assemblers.
pub fn verify(
    method: &Method,
    output: &Path,
    directory: &Path,
    expected: i32,
) -> Result<(), String> {
    let linked = directory.join(format!("{}_verified.elf", method.name));
    let binary = if method.assembly {
        if method.nasm {
            let object = directory.join(format!("{}.o", method.name));
            checked(
                Path::new("nasm"),
                &[
                    "-f".into(),
                    "elf64".into(),
                    output.display().to_string(),
                    "-o".into(),
                    object.display().to_string(),
                ],
                Duration::from_secs(30),
            )?;
            checked(
                Path::new("ld"),
                &[
                    "-z".into(),
                    "noexecstack".into(),
                    object.display().to_string(),
                    "-o".into(),
                    linked.display().to_string(),
                ],
                Duration::from_secs(30),
            )?;
        } else {
            checked(
                &method.program,
                &[
                    "-no-pie".into(),
                    "-x".into(),
                    "assembler".into(),
                    output.display().to_string(),
                    "-o".into(),
                    linked.display().to_string(),
                ],
                Duration::from_secs(30),
            )?;
        }
        &linked
    } else {
        output
    };
    let result = invoke(binary, &[], Vec::new(), Duration::from_secs(5))
        .map_err(|error| format!("execute {}: {error}", binary.display()))?;
    if result.status != Some(expected)
        || result.timed_out
        || result.truncated
        || !result.stdout.is_empty()
        || !result.stderr.is_empty()
    {
        return Err(format!("verify {}: expected exit={expected} and empty streams, got exit={:?}, timeout={}, truncated={}\nstdout:\n{}\nstderr:\n{}",
            binary.display(), result.status, result.timed_out, result.truncated,
            String::from_utf8_lossy(&result.stdout), String::from_utf8_lossy(&result.stderr)));
    }
    Ok(())
}
