// Abi cases for bitfield.
use super::harness;
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    let storage = "struct S{char c;unsigned a:4,b:12;char tail;};";
    let packed = "struct S{unsigned a:3;signed int b:5;unsigned c:24;};";
    let check = "s.a==5&&s.b==-3&&s.c==0xabcdef";
    let mixed = "struct S{double d;unsigned a:5,b:27;};";
    for (name, purpose, source, body, foreign, data) in [
        (
            "storage_in",
            "independent storage fixes bit and byte offsets",
            format!("{storage}int target(struct S *p){{\
                return p->c==18&&p->a==5&&p->b==0xabc&&p->tail==52&&sizeof(*p)==4;\
            }}"),
            "lea rdi,[value]\ncall target\ncmp eax,1\njne bad\n",
            "",
            "value: dd 0x34abc512\n",
        ),
        (
            "storage_out",
            "independent storage checks neighboring bytes after field stores",
            format!("{storage}int target(struct S *p){{p->a=5;p->b=0xabc;return 1;}}"),
            "lea rdi,[value]\ncall target\ncmp dword [value],0x34abc512\njne bad\n",
            "",
            "value: dd 0x34000012\n",
        ),
        (
            "parameter_in",
            "foreign caller supplies a packed INTEGER aggregate",
            format!("{packed}int target(struct S s){{return {check};}}"),
            "mov edi,0xabcdefed\ncall target\ncmp eax,1\njne bad\n",
            "",
            "",
        ),
        (
            "parameter_out",
            "foreign callee checks a packed INTEGER aggregate",
            format!("{packed}int foreign(struct S s);\
            int target(void){{\
                struct S s={{\
                    5,-3,0xabcdef}};\
                return foreign(s);\
            }}"),
            "call target\ncmp eax,1\njne bad\n",
            "global foreign\nforeign:\ncmp edi,0xabcdefed\njne bad\nmov eax,1\nret\n",
            "",
        ),
        (
            "result_out",
            "foreign caller checks the packed aggregate result",
            format!("{packed}struct S target(void){{struct S s={{5,-3,0xabcdef}};return s;}}"),
            "call target\ncmp eax,0xabcdefed\njne bad\n",
            "",
            "",
        ),
        (
            "result_in",
            "foreign callee returns packed signed and unsigned fields",
            format!("{packed}struct S foreign(void);int target(void){{struct S s=foreign();return {check};}}"),
            "call target\ncmp eax,1\njne bad\n",
            "global foreign\nforeign:\nmov eax,0xabcdefed\nret\n",
            "",
        ),
        (
            "mixed_in",
            "foreign caller supplies SSE and INTEGER aggregate chunks",
            format!("{mixed}int target(struct S s){{return s.d==2.5&&s.a==17&&s.b==0x123456;}}"),
            "mov rax,0x4004000000000000\nmovq xmm0,rax\nmov edi,0x2468ad1\ncall target\ncmp eax,1\njne bad\n",
            "",
            "",
        ),
        (
            "mixed_out",
            "foreign callee checks SSE and INTEGER aggregate chunks",
            format!("{mixed}int foreign(struct S s);\
            int target(void){{\
                struct S s={{\
                    2.5,17,0x123456}};\
                return foreign(s);\
            }}"),
            "call target\ncmp eax,1\njne bad\n",
            r#"global foreign
foreign:
cmp edi,0x2468ad1
jne bad
movq rax,xmm0
mov r10,0x4004000000000000
cmp rax,r10
jne bad
mov eax,1
ret
"#,
            "",
        ),
    ] {
        cases.push(Case::new(
            format!("bitfield_abi_{name}"),
            purpose,
            source,
            Expectation::AssemblyHarness(harness(body,foreign,data)),
        ));
    }
    // A protected following page detects over-wide reads or writes of a 24-bit tail.
    cases.push(Case::new(
        "bitfield_abi_page_boundary",
        "exact occupied-byte accesses at a protected page boundary",
        "struct S{\
            char c;\
            unsigned a:24;\
        };\
        int target(struct S *p){\
            unsigned x=p->a;\
            p->a+=3;\
            return x==0xabcdef&&p->a==0xabcdf2&&p->c==18;\
        }",
        Expectation::AssemblyHarness(harness(
            r#"mov eax,9
xor edi,edi
mov esi,8192
mov edx,3
mov r10d,0x22
mov r8,-1
xor r9d,r9d
syscall
test rax,rax
js bad
mov r12,rax
lea rdi,[rax+4096]
mov esi,4096
xor edx,edx
mov eax,10
syscall
test eax,eax
jnz bad
mov dword [r12+4092],0xabcdef12
lea rdi,[r12+4092]
call target
cmp eax,1
jne bad
cmp dword [r12+4092],0xabcdf212
jne bad
"#,
            "",
            "",
        )),
    ));
}
