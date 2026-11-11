// Bit-field tests keep storage, expression semantics, and ABI boundaries distinct.
use crate::Case;

// Build a libc-free independent endpoint with an aligned call site.
fn harness(body: &str, foreign: &str, data: &str) -> String {
    format!(
        r#"bits 64
default rel
section .data
align 16
{data}
section .text
global _start
extern target
_start:
cld
sub rsp,128
{body}
xor edi,edi
jmp finish
bad: mov edi,1
finish: mov eax,60
syscall
{foreign}
section .note.GNU-stack noalloc noexec nowrite progbits
"#
    )
}

// Register field semantics, diagnostics, and manually specified storage probes.
mod abi;
mod calls;
mod constraints;
mod initialization;
mod layout;
mod operations;

// Compose focused case families without changing their IDs or order.
pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    layout::append(&mut cases);
    operations::append(&mut cases);
    initialization::append(&mut cases);
    calls::append(&mut cases);
    constraints::append(&mut cases);
    abi::append(&mut cases);
    cases
}
