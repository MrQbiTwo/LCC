// Independent layout cases for aggregate.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    // Independent NASM data fixes offsets instead of sharing the compiler's layout.
    cases.push(Case::new(
        "aggregate_abi_read",
        "independent caller provides aggregate storage",
        "struct S{\
            char c;\
            int i;\
            double d;\
            long double l;\
            char a[3];\
        };\
        int inspect(struct S *p){\
            return p->c==3&&p->i==-7&&p->d==2.5&&p->l==4.25L&&p->a[2]==9&&sizeof(*p)==48;\
        }",
        Expectation::AssemblyHarness(
            r#"bits 64
default rel
section .data
align 16
value: db 3,0,0,0
dd -7
dq 0x4004000000000000
dq 0x8800000000000000
dw 0x4001
times 6 db 0
db 0,0,9
times 13 db 0
section .text
global _start
extern inspect
_start:
    and rsp,-16
    lea rdi,[value]
    call inspect
    cmp eax,1
    setne dil
    movzx edi,dil
    mov eax,60
    syscall
section .note.GNU-stack noalloc noexec nowrite progbits
"#
            .into(),
        ),
    ));
    cases.push(Case::new(
        "aggregate_abi_write",
        "independent caller verifies aggregate writes and padding boundaries",
        "struct S{\
            char c;\
            int i;\
            double d;\
            long double l;\
            char a[3];\
        };\
        void fill(struct S *p){\
            p->c=3;\
            p->i=-7;\
            p->d=2.5;\
            p->l=4.25L;\
            p->a[2]=9;\
        }",
        Expectation::AssemblyHarness(
            r#"bits 64
default rel
section .data
align 16
before: times 16 db 0x55
value: times 48 db 0x55
after: times 16 db 0x55
section .text
global _start
extern fill
_start:
    and rsp,-16
    lea rdi,[value]
    call fill
    cmp byte [value],3
    jne bad
    cmp byte [value+1],0x55
    jne bad
    cmp dword [value+4],-7
    jne bad
    mov rax,0x4004000000000000
    cmp [value+8],rax
    jne bad
    mov rax,0x8800000000000000
    cmp [value+16],rax
    jne bad
    cmp word [value+24],0x4001
    jne bad
    cmp byte [value+34],9
    jne bad
    cmp byte [value+35],0x55
    jne bad
    cmp byte [before+15],0x55
    jne bad
    cmp byte [after],0x55
    jne bad
    xor edi,edi
    jmp finish
bad: mov edi,1
finish: mov eax,60
    syscall
section .note.GNU-stack noalloc noexec nowrite progbits
"#
            .into(),
        ),
    ));
}
