// Probe decimal conversion's status and exact low/high result bits directly.
use std::{env, fs, path::Path};

use super::build;
use crate::oracle;

pub fn execute(
    directory: &Path,
    value: &str,
    format: u32,
    low: u64,
    high: u64,
    status: i32,
) -> Result<(), String> {
    // Keep a byte available for the zero-length input without changing its length.
    let data = if value.is_empty() {
        "0".into()
    } else {
        value
            .bytes()
            .map(|byte| byte.to_string())
            .collect::<Vec<_>>()
            .join(",")
    };
    let assembly = directory.join("probe.asm");
    let object = directory.join("probe.o");
    let probe = directory.join("probe.elf");
    let length = value.len();
    let text = format!(
        r#"bits 64
default rel
%include "runtime/linux_x86_64/decimal.asm"
section .data
input: db {data}
section .text
global _start
_start:
lea rdi,[input]
mov esi,{length}
mov edx,{format}
call decimal_parse
cmp ecx,{status}
jne bad
mov r8,0x{low:016x}
cmp rax,r8
jne bad
mov r8,0x{high:016x}
cmp rdx,r8
jne bad
xor edi,edi
jmp finish
bad: mov edi,1
finish: mov eax,60
syscall
section .note.GNU-stack noalloc noexec nowrite progbits
"#
    );
    fs::write(&assembly, text).map_err(|error| error.to_string())?;
    let include = format!(
        "{}/",
        env::current_dir()
            .map_err(|error| error.to_string())?
            .display()
    );
    build::assemble(&assembly, &object, &[include])?;
    build::link(&probe, &[&object])?;
    oracle::exit(&build::invoke(&probe, &[])?, 0)
}
