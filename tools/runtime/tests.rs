// Mutation tests exercise externally assembled records rather than parser internals.
use super::*;
use std::{
    env, fs,
    process::{self, Command},
    sync::OnceLock,
};

// Assemble one immutable fixture covering both segments and both supported fixups.
fn fixture() -> Vec<u8> {
    static OBJECT: OnceLock<Vec<u8>> = OnceLock::new();
    OBJECT.get_or_init(|| {
        let path = env::temp_dir().join(format!("lcc_runtime_test_{}", process::id()));
        fs::create_dir(&path).unwrap();
        let source = "bits 64\nsection .text align=16\nglobal lcc_rt_printf,lcc_rt_scanf,lcc_rt_getchar,lcc_rt_exit\nlcc_rt_printf: dd ro - $\nlcc_rt_scanf: dq buffer + 3\nlcc_rt_getchar: ret\nlcc_rt_exit: ret\nsection .rodata align=16\nro: db 1,2,3\nsection .data align=8\npointer: dq lcc_rt_printf + 7\nsection .bss align=32\nbuffer: resb 17\n";
        fs::write(path.join("fixture.asm"), source).unwrap();
        let status = Command::new("nasm").args(["-f", "elf64", "fixture.asm", "-o", "fixture.o"])
            .current_dir(&path).status().unwrap();
        assert!(status.success());
        let bytes = fs::read(path.join("fixture.o")).unwrap();
        fs::remove_dir_all(path).unwrap();
        bytes
    }).clone()
}

// Obtain a header offset from independently assembled section metadata.
fn section(bytes: &[u8], name: &str) -> usize {
    let object = Object::parse(bytes).unwrap();
    let index = object
        .sections
        .iter()
        .position(|section| section.name == name)
        .unwrap();
    size_at(bytes, 40).unwrap() + index * 64
}

// Replace one ELF word while retaining all unrelated fixture bytes.
fn put64(bytes: &mut [u8], offset: usize, value: u64) {
    bytes[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
}

// Required exports, alignment, zero-fill and normalized addends have exact values.
#[test]
fn folds_sections_and_normalizes_relocations() {
    let encoded = String::from_utf8(encode(&fixture()).unwrap()).unwrap();
    for expected in [
        "runtime_code_size 14",
        "runtime_code_alignment 16",
        "runtime_data_size 49",
        "runtime_data_alignment 32",
        "runtime_printf_offset 0",
        "runtime_scanf_offset 4",
        "runtime_getchar_offset 12",
        "runtime_exit_offset 13",
        "runtime_relocation_count 3",
        "dq 1,0,2,0,4,4",
        "dq 1,4,2,32,8,3",
        "dq 2,8,1,0,8,7",
    ] {
        assert!(encoded.contains(expected), "missing {expected}: {encoded}");
    }
}

// Every truncation of a referenced byte must fail; trailing padding is optional.
#[test]
fn rejects_truncated_objects() {
    let bytes = fixture();
    let object = Object::parse(&bytes).unwrap();
    let table_end = size_at(&bytes, 40).unwrap() + object.sections.len() * 64;
    let required = object
        .sections
        .iter()
        .filter(|s| s.kind != 8)
        .map(|s| s.offset + s.size)
        .fold(table_end, usize::max);
    for length in 0..required {
        assert!(
            encode(&bytes[..length]).is_err(),
            "accepted prefix {length}"
        );
    }
}

// Foreign architecture and executable files cannot masquerade as runtime objects.
#[test]
fn rejects_wrong_identity() {
    for (offset, value) in [(0, 0), (4, 1), (5, 2), (16, 2), (18, 3), (20, 0)] {
        let mut bytes = fixture();
        bytes[offset] = value;
        assert!(encode(&bytes).is_err(), "accepted invalid field {offset}");
    }
}

// Section count and physical offsets are bounded before table allocation or reads.
#[test]
fn rejects_section_bounds() {
    let mut bytes = fixture();
    put64(&mut bytes, 40, u64::MAX);
    assert!(encode(&bytes).is_err());
    let mut bytes = fixture();
    let header = section(&bytes, ".text");
    put64(&mut bytes, header + 24, u64::MAX);
    assert!(encode(&bytes).is_err());
}

// Unsupported alignment must fail rather than misaligning an embedded object.
#[test]
fn rejects_invalid_alignment() {
    for alignment in [3, 8192] {
        let mut bytes = fixture();
        let header = section(&bytes, ".bss");
        put64(&mut bytes, header + 48, alignment);
        assert!(encode(&bytes).is_err());
    }
}

// NOBITS cannot request unbounded generated files even when the input is tiny.
#[test]
fn rejects_oversized_zero_fill() {
    let mut bytes = fixture();
    let header = section(&bytes, ".bss");
    put64(&mut bytes, header + 32, (LIMIT + 1) as u64);
    assert!(encode(&bytes).is_err());
}

// Symbol and relocation entry sizes must exactly match the ELF64 ABI.
#[test]
fn rejects_table_entry_sizes() {
    for name in [".symtab", ".rela.text"] {
        let mut bytes = fixture();
        let header = section(&bytes, name);
        put64(&mut bytes, header + 56, 16);
        assert!(encode(&bytes).is_err());
    }
}

// A required export cannot silently disappear or become an external reference.
#[test]
fn rejects_missing_and_undefined_exports() {
    let mut bytes = fixture();
    let object = Object::parse(&bytes).unwrap();
    let symbols = object
        .sections
        .iter()
        .position(|s| s.name == ".symtab")
        .unwrap();
    let names = object.data(object.sections[symbols].link).unwrap();
    let index = object
        .data(symbols)
        .unwrap()
        .chunks_exact(24)
        .position(|s| name_at(names, u32_at(s, 0).unwrap() as usize).unwrap() == "lcc_rt_printf")
        .unwrap();
    let offset = object.sections[symbols].offset + index * 24;
    bytes[offset + 6..offset + 8].copy_from_slice(&0u16.to_le_bytes());
    assert!(encode(&bytes).is_err());
    let mut bytes = fixture();
    bytes[offset..offset + 4].copy_from_slice(&0u32.to_le_bytes());
    assert!(encode(&bytes).is_err());
}

// Unknown relocation types and invalid symbol indices cannot escape validation.
#[test]
fn rejects_relocation_types_and_symbols() {
    for info in [3, u64::MAX] {
        let mut bytes = fixture();
        let header = section(&bytes, ".rela.text");
        let offset = size_at(&bytes, header + 24).unwrap();
        put64(&mut bytes, offset + 8, info);
        assert!(encode(&bytes).is_err());
    }
}

// Patches cannot overwrite another fixup or bytes beyond the source section.
#[test]
fn rejects_overlapping_and_outside_patches() {
    for offset in [2, u64::MAX, 12] {
        let mut bytes = fixture();
        let header = section(&bytes, ".rela.text");
        let records = size_at(&bytes, header + 24).unwrap();
        put64(&mut bytes, records + 24, offset);
        assert!(encode(&bytes).is_err());
    }
}

// PC32 normalization must not wrap its signed addend during conversion.
#[test]
fn rejects_pc_addend_overflow() {
    let mut bytes = fixture();
    let header = section(&bytes, ".rela.text");
    let offset = size_at(&bytes, header + 24).unwrap();
    put64(&mut bytes, offset + 16, i64::MAX as u64);
    assert!(encode(&bytes).is_err());
}

// Metadata pointers must identify a valid symbol or string table.
#[test]
fn rejects_table_links() {
    for name in [".symtab", ".rela.text"] {
        let mut bytes = fixture();
        let header = section(&bytes, name);
        bytes[header + 40..header + 44].copy_from_slice(&u32::MAX.to_le_bytes());
        assert!(encode(&bytes).is_err());
    }
}

// Identical input objects must yield byte-identical standalone include files.
#[test]
fn deterministic_encoding() {
    assert_eq!(encode(&fixture()).unwrap(), encode(&fixture()).unwrap());
}

// The first ELF symbol is reserved and cannot name an allocated object.
#[test]
fn rejects_nonzero_null_symbol() {
    let mut bytes = fixture();
    let header = section(&bytes, ".symtab");
    let symbols = size_at(&bytes, header + 24).unwrap();
    let text = Object::parse(&bytes)
        .unwrap()
        .sections
        .iter()
        .position(|s| s.name == ".text")
        .unwrap();
    bytes[symbols + 6..symbols + 8].copy_from_slice(&(text as u16).to_le_bytes());
    assert!(encode(&bytes).is_err());
}

// A function export cannot point after its section or describe a data object.
#[test]
fn rejects_invalid_export_entries() {
    let original = fixture();
    let object = Object::parse(&original).unwrap();
    let table = object
        .sections
        .iter()
        .position(|s| s.name == ".symtab")
        .unwrap();
    let names = object.data(object.sections[table].link).unwrap();
    let index = object
        .data(table)
        .unwrap()
        .chunks_exact(24)
        .position(|s| name_at(names, u32_at(s, 0).unwrap() as usize).unwrap() == "lcc_rt_printf")
        .unwrap();
    let offset = object.sections[table].offset + index * 24;
    let text = object.sections.iter().find(|s| s.name == ".text").unwrap();
    let mut bytes = original.clone();
    put64(&mut bytes, offset + 8, text.size as u64);
    assert!(encode(&bytes).is_err());
    let mut bytes = original.clone();
    bytes[offset + 4] = 0x11;
    assert!(encode(&bytes).is_err());
    let mut bytes = original;
    bytes[offset + 4] = 0;
    assert!(encode(&bytes).is_err());
}
