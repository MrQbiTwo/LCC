// NASM endpoints supply concrete psABI registers and records without LCC help.
use crate::{Case, Expectation};

pub(super) fn harness(body: &str, foreign: &str) -> String {
    format!(
        r#"bits 64
default rel
section .text
global _start
extern target
_start:
cld
sub rsp,512
mov word [rsp+480],0x077f
fldcw [rsp+480]
mov dword [rsp+484],0x3f80
ldmxcsr [rsp+484]
{body}
fnstcw [rsp+488]
cmp word [rsp+488],0x077f
jne bad
stmxcsr [rsp+492]
mov eax,[rsp+492]
and eax,0xffc0
cmp eax,0x3f80
jne bad
fnstsw ax
test ax,0x3800
jnz bad
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

pub(super) fn add(cases: &mut Vec<Case>, name: &str, source: String, body: &str, foreign: &str) {
    cases.push(Case::new(
        format!("vararg_abi_{name}"),
        name,
        format!("#include <stdarg.h>\n{source}"),
        Expectation::AssemblyHarness(harness(body, foreign)),
    ));
}
