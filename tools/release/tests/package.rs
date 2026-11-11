// Atomic installation and detached package reproduction.
use super::*;

// Existing outputs remain intact and tools are never invoked to replace them.
#[test]
fn refuse_existing_destination() {
    let tree = Tree::new();
    let out = tree.0.join("output");
    fs::create_dir(&out).unwrap();
    fs::write(out.join("keep"), b"prior artifact").unwrap();
    assert!(package(&tree.0, &out, "missing-nasm", "missing-ld").is_err());
    assert_eq!(fs::read(out.join("keep")).unwrap(), b"prior artifact");
}
// Tool failures leave neither a destination nor unfinished staging directories.
#[test]
fn failed_build_is_atomic() {
    let tree = Tree::new();
    let out = tree.0.join("output");
    assert!(package(&tree.0, &out, "missing-lcc-nasm-test-tool", "missing-ld").is_err());
    assert!(!out.exists());
    assert_eq!(fs::read_dir(&tree.0).unwrap().count(), 1);
}
// Complete packaging expands includes/assets and produces identical artifacts
// in separate directories; the result still assembles after removing inputs.
#[test]
fn standalone_and_reproducible() {
    let tree = Tree::new();
    fs::write(
        tree.0.join("body.inc"),
        "mov eax,60\nxor edi,edi\nsyscall\n",
    )
    .unwrap();
    fs::write(tree.0.join("asset.bin"), [0, 1, 127, 255]).unwrap();
    fs::write(tree.0.join("src/entry.asm"),"bits 64\nsection .text\nglobal _start\n_start:\n%include 'body.inc'\nsection .data\nbytes: incbin 'asset.bin'\n").unwrap();
    let first = tree.0.join("first");
    let second = tree.0.join("second");
    package(&tree.0, &first, "nasm", "ld").unwrap();
    package(&tree.0, &second, "nasm", "ld").unwrap();
    for name in ["lcc.asm", "lcc.elf"] {
        assert_eq!(
            fs::read(first.join(name)).unwrap(),
            fs::read(second.join(name)).unwrap()
        );
    }
    for name in ["src/entry.asm", "body.inc", "asset.bin"] {
        fs::remove_file(tree.0.join(name)).unwrap();
    }
    run(
        "nasm",
        &[
            "-f".as_ref(),
            "elf64".as_ref(),
            "lcc.asm".as_ref(),
            "-o".as_ref(),
            "detached.o".as_ref(),
        ],
        &first,
    )
    .unwrap();
    assert!(Command::new(first.join("lcc.elf"))
        .status()
        .unwrap()
        .success());
}
// Runtime bytes are generated inside staging, with no development include needed.
#[test]
fn package_builds_runtime_from_source() {
    let tree = Tree::new();
    fs::create_dir_all(tree.0.join("runtime/linux_x86_64")).unwrap();
    fs::write(tree.0.join("runtime/linux_x86_64/runtime.asm"),
        "bits 64\nsection .text\nglobal lcc_rt_printf,lcc_rt_scanf,lcc_rt_getchar,lcc_rt_exit\nlcc_rt_printf: ret\nlcc_rt_scanf: ret\nlcc_rt_getchar: ret\nlcc_rt_exit: ret\n").unwrap();
    let entry = tree.0.join("src/entry.asm");
    let mut source = fs::read_to_string(&entry).unwrap();
    source.push_str("%include 'runtime_pack.inc'\n");
    fs::write(entry, source).unwrap();
    let first = tree.0.join("first");
    let second = tree.0.join("second");
    package(&tree.0, &first, "nasm", "ld").unwrap();
    package(&tree.0, &second, "nasm", "ld").unwrap();
    for name in ["lcc.asm", "lcc.elf"] {
        assert_eq!(
            fs::read(first.join(name)).unwrap(),
            fs::read(second.join(name)).unwrap()
        );
    }
    let source = fs::read_to_string(first.join("lcc.asm")).unwrap();
    assert!(source.contains("runtime_code_bytes:"));
    assert!(source.contains("db 195,195,195,195"));
    assert_eq!(fs::read_dir(&first).unwrap().count(), 2);
    fs::remove_file(tree.0.join("runtime/linux_x86_64/runtime.asm")).unwrap();
    run(
        "nasm",
        &[
            "-f".as_ref(),
            "elf64".as_ref(),
            "lcc.asm".as_ref(),
            "-o".as_ref(),
            "detached.o".as_ref(),
        ],
        &first,
    )
    .unwrap();
    assert!(Command::new(first.join("lcc.elf"))
        .status()
        .unwrap()
        .success());
}

// A malformed runtime cannot leave a published package or staging products.
#[test]
fn invalid_runtime_package_is_atomic() {
    let tree = Tree::new();
    fs::create_dir_all(tree.0.join("runtime/linux_x86_64")).unwrap();
    fs::write(
        tree.0.join("runtime/linux_x86_64/runtime.asm"),
        "bits 64\nsection .text\nret\n",
    )
    .unwrap();
    let out = tree.0.join("output");
    assert!(package(&tree.0, &out, "nasm", "ld").is_err());
    assert!(!out.exists());
    assert_eq!(fs::read_dir(&tree.0).unwrap().count(), 2);
}
