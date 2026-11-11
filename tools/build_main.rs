// Build the NASM compiler with checked runtime assets and incremental source hashes.
use std::{env, fs, io, path::Path, process};
#[path = "runtime/elf.rs"] mod elf;
#[path = "runtime/pack.rs"] mod pack;
#[path = "runtime/build.rs"] mod build;

// Delete only a marked generated directory strictly inside the current source tree.
fn clean(path: &Path) -> io::Result<()> {
    if !path.exists() { return Ok(()); }
    let root = env::current_dir()?.canonicalize()?;
    let target = path.canonicalize()?;
    if target == root || !target.starts_with(&root) || !target.join(".lcc_build").is_file() {
        return Err(io::Error::other("refusing to remove an unmarked or external directory"));
    }
    fs::remove_dir_all(target)
}

// Build into temporary products and preserve the last compiler if either tool fails.
fn compile(root: &Path, out: &Path, nasm: &str, ld: &str, runtime_only: bool) -> io::Result<()> {
    let root = root.canonicalize()?;
    fs::create_dir_all(out)?;
    let out = out.canonicalize()?;
    build::publish(&out.join(".lcc_build"), b"LCC generated output\n")?;
    build::prepare(&root, &out, nasm)?;
    if runtime_only { return Ok(()); }
    let stamp = build::fingerprint(&root, &["src", "include", "runtime", "tools/build_main.rs",
        "tools/runtime/elf.rs", "tools/runtime/pack.rs", "tools/runtime/build.rs"],
        &[nasm, ld, "elf64-dwarf-static-v1"])?;
    let previous = out.join("compiler.stamp");
    let products = [out.join("lcc"), out.join("lcc.o")];
    if build::fresh(&previous, &stamp, &[&products[0], &products[1]]) { return Ok(()); }
    let object = out.join(format!("lcc_{}.o", process::id()));
    let binary = out.join(format!("lcc_{}.new", process::id()));
    let include = format!("{}/", out.display());
    let work = (|| {
        build::run(nasm, &["-f".as_ref(), "elf64".as_ref(), "-g".as_ref(), "-F".as_ref(), "dwarf".as_ref(),
            "-I".as_ref(), "./".as_ref(), "-I".as_ref(), include.as_ref(),
            "src/entry.asm".as_ref(), "-o".as_ref(), object.as_os_str()], &root)?;
        build::run(ld, &["-static".as_ref(), "-z".as_ref(), "noexecstack".as_ref(),
            "-o".as_ref(), binary.as_os_str(), object.as_os_str()], &root)?;
        fs::rename(&object, out.join("lcc.o"))?;
        fs::rename(&binary, out.join("lcc"))?;
        build::publish(&previous, build::artifact_stamp(&stamp, &[&products[0], &products[1]])?.as_bytes())
    })();
    let _ = fs::remove_file(object);
    let _ = fs::remove_file(binary);
    work
}

// Ordinary PATH tool selection and output paths remain command-line arguments.
fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let result = if args.first().is_some_and(|arg| arg == "--clean") {
        args.get(1).ok_or_else(|| io::Error::other("missing build directory")).and_then(|path| clean(Path::new(path)))
    } else {
        let runtime_only = args.first().is_some_and(|arg| arg == "--runtime");
        if runtime_only { args.remove(0); }
        env::current_dir().and_then(|root| compile(&root, Path::new(args.first().map_or("build", String::as_str)),
            args.get(1).map_or("nasm", String::as_str), args.get(2).map_or("ld", String::as_str), runtime_only))
    };
    if let Err(error) = result { eprintln!("build: {error}"); process::exit(1); }
}

#[cfg(test)]
#[path = "runtime/compiler_tests.rs"] mod tests;
