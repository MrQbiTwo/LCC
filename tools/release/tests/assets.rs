// Source flattening and filesystem-boundary contracts.
use super::super::source::{asset, asset_name, flatten};
use super::*;

// Every embedded byte is represented without escaping, encoding loss, or paths.
#[test]
fn arbitrary_asset_bytes() {
    let tree = Tree::new();
    fs::write(tree.0.join("bytes.bin"), [0, 10, 13, 39, 34, 255]).unwrap();
    let output = String::from_utf8(
        flatten(
            &tree.0,
            "%line 42+1 src/input.asm\ndata: incbin 'bytes.bin'\n",
        )
        .unwrap(),
    )
    .unwrap();
    assert!(output.ends_with("data:\n    db 0,10,13,39,34,255\n"));
    assert!(!output.contains("src/input.asm"));
}
// Empty assets preserve their labels and do not fabricate data.
#[test]
fn empty_asset() {
    let tree = Tree::new();
    fs::write(tree.0.join("empty.bin"), []).unwrap();
    assert!(flatten(&tree.0, "empty: incbin \"empty.bin\" ; comment\n")
        .unwrap()
        .ends_with(b"empty:\n"));
}
// Operand slices must fail explicitly until the packer supports their semantics.
#[test]
fn reject_sliced_asset() {
    assert!(asset_name("'data',1,2").is_err());
}
// NASM preprocessing adds an explicit zero offset to ordinary incbin input.
#[test]
fn accept_default_offset() {
    assert_eq!(asset_name("'data', 0 ; default").unwrap(), "data");
}
// Indirect names cannot silently leave an external dependency in the artifact.
#[test]
fn reject_indirect_asset() {
    assert!(asset_name("FILE_NAME").is_err());
}
// Preprocessing must be complete before the standalone artifact is accepted.
#[test]
fn reject_unexpanded_directive() {
    let tree = Tree::new();
    assert!(flatten(&tree.0, "%include \"hidden.inc\"\n").is_err());
}
// A missing asset is an error, rather than an empty replacement.
#[test]
fn reject_missing_asset() {
    let tree = Tree::new();
    assert!(asset(&tree.0, "missing.bin").is_err());
}
// Parent traversal cannot copy private data into a distribution artifact.
#[test]
fn reject_parent_escape() {
    let tree = Tree::new();
    let inner = tree.0.join("inner");
    fs::create_dir(&inner).unwrap();
    assert!(asset(&inner, "../src/entry.asm").is_err());
}
// Absolute machine-specific paths are not allowed in embedded source inputs.
#[test]
fn reject_absolute_asset() {
    let tree = Tree::new();
    assert!(asset(&tree.0, tree.0.join("src/entry.asm").to_str().unwrap()).is_err());
}
// String colons and comments are ordinary source, not incbin labels.
#[test]
fn preserve_ordinary_source() {
    let tree = Tree::new();
    let input = "db 'x:y',0\n; incbin 'missing'\n";
    assert!(flatten(&tree.0, input).unwrap().ends_with(input.as_bytes()));
}
// Symlinks cannot bypass the canonical source-root check.
#[cfg(unix)]
#[test]
fn reject_symlink_escape() {
    let tree = Tree::new();
    let inner = tree.0.join("inner");
    fs::create_dir(&inner).unwrap();
    std::os::unix::fs::symlink(tree.0.join("src/entry.asm"), inner.join("outside.bin")).unwrap();
    assert!(asset(&inner, "outside.bin").is_err());
}
