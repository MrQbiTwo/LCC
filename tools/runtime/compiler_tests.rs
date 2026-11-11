// Compiler builder checks use minimal sources to isolate cache and tool behavior.
use super::*;
use std::{path::PathBuf, sync::atomic::{AtomicU64, Ordering}};
static NEXT: AtomicU64 = AtomicU64::new(0);

struct Tree(PathBuf);
impl Tree {
    fn new() -> Self {
        let path = env::temp_dir().join(format!("lcc_compiler_build_{}_{}", process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
        fs::create_dir(&path).unwrap(); fs::create_dir(path.join("src")).unwrap();
        fs::write(path.join("src/entry.asm"),
            "bits 64\nsection .text\nglobal _start\n_start: mov eax,60\nxor edi,edi\nsyscall\n").unwrap();
        Self(path)
    }
    fn output(&self) -> PathBuf { self.0.join("output") }
    fn compile(&self, linker: &str) -> io::Result<()> { compile(&self.0, &self.output(), "nasm", linker, false) }
}
impl Drop for Tree {
    fn drop(&mut self) { fs::remove_dir_all(&self.0).unwrap(); }
}

// Unchanged inputs preserve products and failed linking preserves the last compiler.
#[test]
fn compiler_cache_and_link_failure() {
    let tree = Tree::new(); tree.compile("ld").unwrap();
    let binary = tree.output().join("lcc"); let object = tree.output().join("lcc.o");
    let before = fs::metadata(&binary).unwrap().modified().unwrap();
    let executable = fs::read(&binary).unwrap(); let assembled = fs::read(&object).unwrap();
    tree.compile("ld").unwrap();
    assert_eq!(before, fs::metadata(&binary).unwrap().modified().unwrap());
    assert!(tree.compile("missing-lcc-linker-test").is_err());
    assert_eq!(fs::read(binary).unwrap(), executable);
    assert_eq!(fs::read(object).unwrap(), assembled);
    assert_eq!(fs::read_dir(tree.output()).unwrap().count(), 4);
}

// An intact input stamp cannot conceal a damaged executable or missing object.
#[test]
fn compiler_recovers_damaged_products() {
    let tree = Tree::new(); tree.compile("ld").unwrap();
    let binary = tree.output().join("lcc"); let object = tree.output().join("lcc.o");
    let executable = fs::read(&binary).unwrap(); let assembled = fs::read(&object).unwrap();
    fs::write(&binary, b"corrupt").unwrap(); tree.compile("ld").unwrap();
    assert_eq!(fs::read(binary).unwrap(), executable);
    fs::remove_file(&object).unwrap(); tree.compile("ld").unwrap();
    assert_eq!(fs::read(object).unwrap(), assembled);
}

// Bad source cannot publish a partial compiler or leave temporary tool products.
#[test]
fn compiler_assembly_failure_is_atomic() {
    let tree = Tree::new(); tree.compile("ld").unwrap();
    let binary = fs::read(tree.output().join("lcc")).unwrap();
    let object = fs::read(tree.output().join("lcc.o")).unwrap();
    let stamp = fs::read(tree.output().join("compiler.stamp")).unwrap();
    fs::write(tree.0.join("src/entry.asm"), "invalid opcode here\n").unwrap();
    assert!(tree.compile("ld").is_err());
    assert_eq!(fs::read(tree.output().join("lcc")).unwrap(), binary);
    assert_eq!(fs::read(tree.output().join("lcc.o")).unwrap(), object);
    assert_eq!(fs::read(tree.output().join("compiler.stamp")).unwrap(), stamp);
    assert_eq!(fs::read_dir(tree.output()).unwrap().count(), 4);
}
