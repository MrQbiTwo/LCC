// Build a new standalone package and publish only complete artifacts.
use super::{build, problem, run, source::flatten};
use std::{fs, io, path::Path, process};

// Build both compiler forms in a new staging directory and publish them together.
pub(super) fn package(root: &Path, output: &Path, nasm: &str, ld: &str) -> io::Result<()> {
    let root = root.canonicalize()?;
    if !root.join("src/entry.asm").is_file() {
        return Err(problem("missing compiler entry source"));
    }
    let parent = output
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    fs::create_dir_all(parent)?;
    let parent = parent.canonicalize()?;
    let name = output
        .file_name()
        .ok_or_else(|| problem("output requires a directory name"))?;
    let destination = parent.join(name);
    if destination.symlink_metadata().is_ok() {
        return Err(problem("artifact directory already exists"));
    }
    let stage = parent.join(format!(".lcc_package_{}", process::id()));
    fs::create_dir(&stage)?;
    let build = (|| {
        build::prepare(&root, &stage, nasm)?;
        let generated = format!("{}/", stage.display());
        let preprocessed = run(
            nasm,
            &[
                "-E".as_ref(),
                "-I".as_ref(),
                "./".as_ref(),
                "-I".as_ref(),
                generated.as_ref(),
                "src/entry.asm".as_ref(),
            ],
            &root,
        )?;
        let preprocessed =
            std::str::from_utf8(&preprocessed).map_err(|e| problem(e.to_string()))?;
        let source = flatten(&root, preprocessed)?;
        fs::write(stage.join("lcc.asm"), source)?;
        run(
            nasm,
            &[
                "-f".as_ref(),
                "elf64".as_ref(),
                "lcc.asm".as_ref(),
                "-o".as_ref(),
                "lcc.o".as_ref(),
            ],
            &stage,
        )?;
        run(
            ld,
            &[
                "-static".as_ref(),
                "-z".as_ref(),
                "noexecstack".as_ref(),
                "-o".as_ref(),
                "lcc.elf".as_ref(),
                "lcc.o".as_ref(),
            ],
            &stage,
        )?;
        fs::remove_file(stage.join("lcc.o"))?;
        for name in ["runtime_pack.inc", "runtime.stamp"] {
            let path = stage.join(name);
            if path.exists() {
                fs::remove_file(path)?;
            }
        }
        fs::rename(&stage, &destination)?;
        Ok(())
    })();
    if build.is_err() {
        let _ = fs::remove_dir_all(&stage);
    }
    build
}
