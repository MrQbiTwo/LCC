// Isolated runtime preparation checks cover cache recovery and failure preservation.
use super::*;
use std::{
    env,
    sync::atomic::{AtomicU64, Ordering},
};
static NEXT: AtomicU64 = AtomicU64::new(0);

// Each test owns its source and output tree; no repository artifacts are touched.
struct Tree(PathBuf);
impl Tree {
    fn new() -> Self {
        let path = env::temp_dir().join(format!(
            "lcc_runtime_build_{}_{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        fs::create_dir_all(path.join("runtime/linux_x86_64")).unwrap();
        fs::write(path.join("runtime/linux_x86_64/runtime.asm"),
            "bits 64\nsection .text\nglobal lcc_rt_printf,lcc_rt_scanf,lcc_rt_getchar,lcc_rt_exit\nlcc_rt_printf: ret\nlcc_rt_scanf: ret\nlcc_rt_getchar: ret\nlcc_rt_exit: ret\n").unwrap();
        Self(path)
    }
    fn output(&self) -> PathBuf {
        self.0.join("output")
    }
}
impl Drop for Tree {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

// Repeated preparation leaves content and modification times unchanged.
#[test]
fn unchanged_runtime_is_not_republished() {
    let tree = Tree::new();
    let out = tree.output();
    prepare(&tree.0, &out, "nasm").unwrap();
    let include = out.join("runtime_pack.inc");
    let before = fs::metadata(&include).unwrap().modified().unwrap();
    prepare(&tree.0, &out, "nasm").unwrap();
    assert_eq!(before, fs::metadata(include).unwrap().modified().unwrap());
    assert_eq!(fs::read_dir(out).unwrap().count(), 2);
}

// A stamp cannot authenticate a changed generated include, even with a new mtime.
#[test]
fn damaged_runtime_include_is_rebuilt() {
    let tree = Tree::new();
    let out = tree.output();
    prepare(&tree.0, &out, "nasm").unwrap();
    let include = out.join("runtime_pack.inc");
    let expected = fs::read(&include).unwrap();
    fs::write(&include, b"; corrupt generated output\n").unwrap();
    prepare(&tree.0, &out, "nasm").unwrap();
    assert_eq!(fs::read(&include).unwrap(), expected);
    fs::remove_file(&include).unwrap();
    prepare(&tree.0, &out, "nasm").unwrap();
    assert_eq!(fs::read(include).unwrap(), expected);
}

// Broken runtime source preserves the previous accepted include and stamp.
#[test]
fn assembly_failure_preserves_runtime_products() {
    let tree = Tree::new();
    let out = tree.output();
    prepare(&tree.0, &out, "nasm").unwrap();
    let include = fs::read(out.join("runtime_pack.inc")).unwrap();
    let stamp = fs::read(out.join("runtime.stamp")).unwrap();
    fs::write(
        tree.0.join("runtime/linux_x86_64/runtime.asm"),
        "invalid opcode here\n",
    )
    .unwrap();
    assert!(prepare(&tree.0, &out, "nasm").is_err());
    assert_eq!(fs::read(out.join("runtime_pack.inc")).unwrap(), include);
    assert_eq!(fs::read(out.join("runtime.stamp")).unwrap(), stamp);
    assert_eq!(fs::read_dir(out).unwrap().count(), 2);
}

// Successful assembly is insufficient if its exports do not satisfy the contract.
#[test]
fn invalid_object_preserves_runtime_products() {
    let tree = Tree::new();
    let out = tree.output();
    prepare(&tree.0, &out, "nasm").unwrap();
    let include = fs::read(out.join("runtime_pack.inc")).unwrap();
    let stamp = fs::read(out.join("runtime.stamp")).unwrap();
    fs::write(
        tree.0.join("runtime/linux_x86_64/runtime.asm"),
        "bits 64\nsection .text\nret\n",
    )
    .unwrap();
    assert!(prepare(&tree.0, &out, "nasm").is_err());
    assert_eq!(fs::read(out.join("runtime_pack.inc")).unwrap(), include);
    assert_eq!(fs::read(out.join("runtime.stamp")).unwrap(), stamp);
    assert_eq!(fs::read_dir(out).unwrap().count(), 2);
}

// Content fingerprints include renamed and deleted inputs rather than just mtimes.
#[test]
fn fingerprint_tracks_source_inventory() {
    let tree = Tree::new();
    let inputs = ["runtime"];
    let before = fingerprint(&tree.0, &inputs, &["one"]).unwrap();
    let source = tree.0.join("runtime/linux_x86_64/runtime.asm");
    let renamed = source.with_file_name("renamed.asm");
    fs::rename(&source, &renamed).unwrap();
    let after = fingerprint(&tree.0, &inputs, &["one"]).unwrap();
    assert_ne!(before, after);
    fs::remove_file(renamed).unwrap();
    assert_ne!(after, fingerprint(&tree.0, &inputs, &["one"]).unwrap());
    assert_ne!(after, fingerprint(&tree.0, &inputs, &["two"]).unwrap());
}

// Failed publication removes its temporary file and preserves the destination.
#[test]
fn failed_publish_cleans_temporary_file() {
    let tree = Tree::new();
    let target = tree.0.join("destination");
    fs::create_dir(&target).unwrap();
    fs::write(target.join("keep"), b"unchanged").unwrap();
    assert!(publish(&target, b"replacement").is_err());
    assert_eq!(fs::read(target.join("keep")).unwrap(), b"unchanged");
    assert!(!target
        .with_extension(format!("tmp_{}", std::process::id()))
        .exists());
}
