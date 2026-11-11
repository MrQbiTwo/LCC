// Independent exit, diagnostics, and artifact expectations for compiler cases.
use super::process::Output;

// Reject timeout, truncation, or signal death before comparing ordinary results.
pub fn exit(output: &Output, expected: i32) -> Result<(), String> {
    if output.timed_out || output.truncated || output.status != Some(expected) {
        return Err(format!("expected exit {expected}, got {output:?}"));
    }
    Ok(())
}

// Require a stable diagnostic code without matching incidental prose.
pub fn diagnostic(output: &Output, code: &str) -> Result<(), String> {
    exit(output, 1)?;
    if !String::from_utf8_lossy(&output.stderr).contains(code) {
        return Err(format!("missing diagnostic {code}: {output:?}"));
    }
    Ok(())
}

// Check ELF metadata independently of the compiler serializer.
pub fn elf(bytes: &[u8]) -> Result<(), String> {
    if bytes.len() < 288 || &bytes[..7] != b"\x7fELF\x02\x01\x01" {
        return Err("invalid ELF header".into());
    }
    let u16_at = |n| u16::from_le_bytes(bytes[n..n + 2].try_into().unwrap());
    let u32_at = |n| u32::from_le_bytes(bytes[n..n + 4].try_into().unwrap());
    let u64_at = |n| u64::from_le_bytes(bytes[n..n + 8].try_into().unwrap());
    if u16_at(16) != 2 || u16_at(18) != 62 || u16_at(54) != 56 {
        return Err("wrong executable class, machine, or program-header size".into());
    }
    let count = u16_at(56) as usize;
    let start = u64_at(32) as usize;
    if start
        .checked_add(count * 56)
        .map_or(true, |end| end > bytes.len())
    {
        return Err("program headers outside file".into());
    }
    let entry = u64_at(24);
    let mut executable_entry = false;
    let mut nonexec_stack = false;
    for n in 0..count {
        let p = start + n * 56;
        let kind = u32_at(p);
        let flags = u32_at(p + 4);
        if kind == 3 || kind == 2 {
            return Err("dynamic dependency".into());
        }
        if kind == 0x6474e551 {
            nonexec_stack = flags & 1 == 0;
        }
        if kind != 1 {
            continue;
        }
        let offset = u64_at(p + 8);
        let address = u64_at(p + 16);
        let file_size = u64_at(p + 32);
        let memory_size = u64_at(p + 40);
        let alignment = u64_at(p + 48);
        if flags & 3 == 3
            || file_size > memory_size
            || alignment != 4096
            || offset % alignment != address % alignment
            || offset
                .checked_add(file_size)
                .map_or(true, |end| end > bytes.len() as u64)
        {
            return Err("invalid load segment bounds or permissions".into());
        }
        executable_entry |= flags & 1 != 0 && entry >= address && entry < address + memory_size;
    }
    if !executable_entry || !nonexec_stack {
        return Err("invalid entry or stack policy".into());
    }
    Ok(())
}
