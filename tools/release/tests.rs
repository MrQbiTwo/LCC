// Artifact tests use isolated trees and independent expected source bytes.
use super::*;
use std::fs;
use std::sync::atomic::{AtomicU64, Ordering};
static NEXT: AtomicU64 = AtomicU64::new(0);

// Remove only the temporary tree created by this test instance.
struct Tree(PathBuf);
impl Tree {
    // Allocate a unique, owned fixture directory with the required entry path.
    fn new() -> Self {
        let path = env::temp_dir().join(format!(
            "lcc_package_test_{}_{}",
            process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        fs::create_dir(path.join("src")).unwrap();
        fs::write(
            path.join("src/entry.asm"),
            "bits 64\nsection .text\nglobal _start\n_start: mov eax,60\nxor edi,edi\nsyscall\n",
        )
        .unwrap();
        Self(path.canonicalize().unwrap())
    }
}
impl Drop for Tree {
    // The fixture constructor owns this directory, including failed staging work.
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

#[path = "tests/assets.rs"]
mod assets;
#[path = "tests/package.rs"]
mod package;
