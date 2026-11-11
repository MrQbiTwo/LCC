// Shared runtime preparation and content fingerprints for development and packages.
use super::pack;
use std::{
    fs, io,
    path::{Path, PathBuf},
    process::Command,
};

// A non-cryptographic content digest detects accidental stale or damaged products.
fn digest(bytes: &[u8]) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for &byte in bytes {
        hash = (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

// Cache records cover outputs as well as inputs, so timestamps cannot hide damage.
pub fn artifact_stamp(input: &str, artifacts: &[&Path]) -> io::Result<String> {
    let mut stamp = format!("lcc-artifacts-v1\n{input}");
    for path in artifacts {
        stamp.push_str(&digest(&fs::read(path)?));
        stamp.push('\n');
    }
    Ok(stamp)
}

// Missing, changed or unreadable artifacts are cache misses and must be regenerated.
pub fn fresh(stamp: &Path, input: &str, artifacts: &[&Path]) -> bool {
    artifact_stamp(input, artifacts)
        .is_ok_and(|expected| fs::read_to_string(stamp).is_ok_and(|actual| actual == expected))
}

// Build tools receive structured arguments and never inherit implicit NASM options.
pub fn run(tool: &str, args: &[&std::ffi::OsStr], directory: &Path) -> io::Result<()> {
    let output = Command::new(tool)
        .args(args)
        .current_dir(directory)
        .env_remove("NASMENV")
        .output()?;
    if output.status.success() {
        return Ok(());
    }
    Err(io::Error::other(format!(
        "{tool} failed ({}): {}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    )))
}

// Collect a stable recursive source inventory without traversing directory symlinks.
fn inventory(root: &Path, path: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    if !path.exists() {
        return Ok(());
    }
    if path.is_file() {
        files.push(path.strip_prefix(root).unwrap().to_path_buf());
        return Ok(());
    }
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            inventory(root, &entry.path(), files)?;
        } else if entry.file_type()?.is_file() {
            files.push(entry.path().strip_prefix(root).unwrap().to_path_buf());
        }
    }
    Ok(())
}

// Hash source contents rather than timestamps so unchanged regeneration is cheap.
pub fn fingerprint(root: &Path, paths: &[&str], options: &[&str]) -> io::Result<String> {
    let mut files = Vec::new();
    for path in paths {
        inventory(root, &root.join(path), &mut files)?;
    }
    files.sort();
    files.dedup();
    let mut hash = 0xcbf29ce484222325u64;
    let mut add = |bytes: &[u8]| {
        for &byte in bytes {
            hash = (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3);
        }
        hash = (hash ^ 0xff).wrapping_mul(0x100000001b3);
    };
    for option in options {
        add(option.as_bytes());
    }
    for file in files {
        add(file.to_string_lossy().as_bytes());
        add(&fs::read(root.join(file))?);
    }
    Ok(format!("{hash:016x}\n"))
}

// Refresh one file only when its bytes change; callers prepare all fallible work first.
pub fn publish(path: &Path, bytes: &[u8]) -> io::Result<()> {
    if fs::read(path).is_ok_and(|old| old == bytes) {
        return Ok(());
    }
    let temporary = path.with_extension(format!("tmp_{}", std::process::id()));
    let result = fs::write(&temporary, bytes).and_then(|()| fs::rename(&temporary, path));
    if result.is_err() {
        let _ = fs::remove_file(temporary);
    }
    result
}

// Assemble runtime sources only after their inputs change, then validate before publishing.
pub fn prepare(root: &Path, out: &Path, nasm: &str) -> io::Result<()> {
    // Small release-tool fixtures intentionally have no runtime subsystem.
    if !root.join("runtime/linux_x86_64/runtime.asm").is_file() {
        return Ok(());
    }
    fs::create_dir_all(out)?;
    let stamp = fingerprint(
        root,
        &[
            "runtime",
            "tools/runtime/elf.rs",
            "tools/runtime/pack.rs",
            "tools/runtime/build.rs",
        ],
        &[nasm, "elf64-runtime-v1"],
    )?;
    let include = out.join("runtime_pack.inc");
    let previous = out.join("runtime.stamp");
    if fresh(&previous, &stamp, &[&include]) {
        return Ok(());
    }
    let object = out.join(format!("runtime_{}.o", std::process::id()));
    let work = (|| {
        run(
            nasm,
            &[
                "-f".as_ref(),
                "elf64".as_ref(),
                "-I".as_ref(),
                "./".as_ref(),
                "runtime/linux_x86_64/runtime.asm".as_ref(),
                "-o".as_ref(),
                object.as_os_str(),
            ],
            root,
        )?;
        let encoded = pack::encode(&fs::read(&object)?)?;
        publish(&include, &encoded)?;
        publish(&previous, artifact_stamp(&stamp, &[&include])?.as_bytes())
    })();
    let _ = fs::remove_file(object);
    work
}

#[cfg(test)]
#[path = "build_tests.rs"]
mod tests;
