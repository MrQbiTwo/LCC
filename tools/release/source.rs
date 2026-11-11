// Flatten checked NASM assets into self-contained compiler source.
use super::problem;
use std::{
    fs,
    io::{self, Write},
    path::Path,
};

// Extract a literal NASM incbin path, accepting its default offset and comment.
pub(super) fn asset_name(operand: &str) -> io::Result<&str> {
    let operand = operand.trim();
    let quote = operand
        .chars()
        .next()
        .ok_or_else(|| problem("missing incbin operand"))?;
    if quote != '\'' && quote != '"' {
        return Err(problem("incbin requires a literal path"));
    }
    let end = operand[1..]
        .find(quote)
        .ok_or_else(|| problem("unterminated incbin path"))?
        + 1;
    let tail = operand[end + 1..].split(';').next().unwrap().trim();
    let default_offset = tail
        .strip_prefix(',')
        .is_some_and(|offset| offset.trim() == "0");
    if !tail.is_empty() && !default_offset {
        return Err(problem(
            "incbin slices are not supported by the artifact packer",
        ));
    }
    Ok(&operand[1..end])
}

// Resolve embedded data strictly within the selected source tree.
pub(super) fn asset(root: &Path, name: &str) -> io::Result<Vec<u8>> {
    let relative = Path::new(name);
    if relative.is_absolute() {
        return Err(problem("absolute incbin path"));
    }
    let path = root.join(relative).canonicalize()?;
    if !path.starts_with(root) || !path.is_file() {
        return Err(problem("incbin escapes source root"));
    }
    if fs::metadata(&path)?.len() > 64 * 1024 * 1024 {
        return Err(problem("incbin resource limit"));
    }
    fs::read(path)
}

// Replace preprocessed incbin directives with exact decimal bytes and remove
// source-location directives, so the resulting NASM file has no source-tree paths.
pub(super) fn flatten(root: &Path, preprocessed: &str) -> io::Result<Vec<u8>> {
    let mut result=b"; LCC compiler for Linux x86-64; standalone NASM source.\n; Assemble: nasm -f elf64 lcc.asm -o lcc.o\n; Link: ld -static -z noexecstack -o lcc.elf lcc.o\n".to_vec();
    for line in preprocessed.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("%line ") {
            continue;
        }
        if trimmed.starts_with('%') {
            return Err(problem("unresolved NASM preprocessing directive"));
        }
        let (label, instruction) = match trimmed.split_once(':') {
            Some((label, rest))
                if !label.contains(|c: char| c.is_whitespace() || c == '\'' || c == '"') =>
            {
                (Some(label), rest.trim_start())
            }
            _ => (None, trimmed),
        };
        let end = instruction
            .find(char::is_whitespace)
            .unwrap_or(instruction.len());
        if instruction[..end].eq_ignore_ascii_case("incbin") {
            let bytes = asset(root, asset_name(&instruction[end..])?)?;
            if let Some(label) = label {
                writeln!(result, "{label}:")?;
            }
            for row in bytes.chunks(24) {
                write!(result, "    db ")?;
                for (n, byte) in row.iter().enumerate() {
                    if n != 0 {
                        write!(result, ",")?;
                    }
                    write!(result, "{byte}")?;
                }
                writeln!(result)?;
            }
        } else {
            writeln!(result, "{line}")?;
        }
        if result.len() > 256 * 1024 * 1024 {
            return Err(problem("standalone source resource limit"));
        }
    }
    Ok(result)
}
