// Independent endpoint probes cover register rollback, stack alignment, and sret.
use crate::{Case, Expectation};

// All endpoint probes use a 16-byte aligned call site and bounded scratch space.
fn harness(body: &str, foreign: &str) -> String {
    format!(
        r#"bits 64
default rel
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

// Emit both directions from manually specified ABI placement and C field checks.
fn endpoints(
    cases: &mut Vec<Case>,
    name: &str,
    decl: &str,
    params: &str,
    args: &str,
    initializer: &str,
    check: &str,
    setup: &str,
    inspect: &str,
) {
    let source = format!("{decl}int target({params}){{return {check};}}");
    cases.push(Case::new(
        format!("aggregate_limits_{name}_in"),
        format!("independent incoming {name}"),
        source,
        Expectation::AssemblyHarness(harness(
            &format!("{setup}call target\ncmp eax,1\njne bad\n"),
            "",
        )),
    ));
    let source = format!(
        "{decl}int foreign({params});int target(void){{{initializer}return foreign({args});}}"
    );
    let foreign = format!("global foreign\nforeign:\n{inspect}mov eax,1\nret\n");
    cases.push(Case::new(
        format!("aggregate_limits_{name}_out"),
        format!("independent outgoing {name}"),
        source,
        Expectation::AssemblyHarness(harness("call target\ncmp eax,1\njne bad\n", &foreign)),
    ));
}

// Explicit slot indices detect cursor changes from failed aggregate allocation.
pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    let gp_setup = "mov edi,1\nmov esi,2\nmov edx,3\nmov ecx,4\nmov r8d,5\n";
    let gp_check="cmp edi,1\njne bad\ncmp esi,2\njne bad\ncmp edx,3\njne bad\ncmp ecx,4\njne bad\ncmp r8d,5\njne bad\n";
    endpoints(
        &mut cases,
        "gp_rollback",
        "struct S{long x,y;};",
        "long a,long b,long c,long d,long e,struct S s,long last",
        "1,2,3,4,5,s,9",
        "struct S s={6,7};",
        "a==1&&b==2&&c==3&&d==4&&e==5&&s.x==6&&s.y==7&&last==9",
        &format!("{gp_setup}mov r9d,9\nmov qword [rsp],6\nmov qword [rsp+8],7\n"),
        &format!("{gp_check}cmp r9d,9\njne bad\ncmp qword [rsp+8],6\njne bad\ncmp qword [rsp+16],7\njne bad\n"),
    );
    let fp_setup = (0..7)
        .map(|i| format!("movq xmm{i},rax\n"))
        .collect::<String>();
    let fp_check = (0..7)
        .map(|i| format!("movq rax,xmm{i}\nmov r10,0x3ff0000000000000\ncmp rax,r10\njne bad\n"))
        .collect::<String>();
    endpoints(
        &mut cases,
        "sse_rollback",
        "struct S{double x,y;};",
        "double a,double b,double c,double d,double e,double f,double g,struct S s,double last",
        "1.0,1.0,1.0,1.0,1.0,1.0,1.0,s,4.0",
        "struct S s={2.0,3.0};",
        "a+b+c+d+e+f+g==7.0&&s.x==2.0&&s.y==3.0&&last==4.0",
        &format!(
            r#"mov rax,0x3ff0000000000000
{fp_setup}mov rax,0x4010000000000000
movq xmm7,rax
mov rax,0x4000000000000000
mov [rsp],rax
mov rax,0x4008000000000000
mov [rsp+8],rax
"#
        ),
        &format!(
            r#"{fp_check}movq rax,xmm7
mov r10,0x4010000000000000
cmp rax,r10
jne bad
mov rax,[rsp+8]
mov r10,0x4000000000000000
cmp rax,r10
jne bad
mov rax,[rsp+16]
mov r10,0x4008000000000000
cmp rax,r10
jne bad
"#
        ),
    );
    endpoints(
        &mut cases,
        "mixed_gp_exhaustion",
        "struct S{double x;long y;};",
        "long a,long b,long c,long d,long e,long f,struct S s,double last",
        "1,2,3,4,5,6,s,4.0",
        "struct S s={2.0,7};",
        "a+b+c+d+e+f==21&&s.x==2.0&&s.y==7&&last==4.0",
        &format!(
            r#"{gp_setup}mov r9d,6
mov rax,0x4010000000000000
movq xmm0,rax
mov rax,0x4000000000000000
mov [rsp],rax
mov qword [rsp+8],7
"#
        ),
        &format!(
            r#"{gp_check}cmp r9d,6
jne bad
movq rax,xmm0
mov r10,0x4010000000000000
cmp rax,r10
jne bad
mov rax,[rsp+8]
mov r10,0x4000000000000000
cmp rax,r10
jne bad
cmp qword [rsp+16],7
jne bad
"#
        ),
    );
    endpoints(
        &mut cases,
        "mixed_sse_exhaustion",
        "struct S{long x;double y;};",
        "double a,double b,double c,double d,double e,double f,double g,double h,struct S s,long last",
        "1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,s,9",
        "struct S s={7,2.0};",
        "a+b+c+d+e+f+g+h==8.0&&s.x==7&&s.y==2.0&&last==9",
        &format!(r#"mov rax,0x3ff0000000000000
{fp_setup}movq xmm7,rax
mov edi,9
mov qword [rsp],7
mov rax,0x4000000000000000
mov [rsp+8],rax
"#),
        &format!(r#"cmp edi,9
jne bad
{fp_check}movq rax,xmm7
mov r10,0x3ff0000000000000
cmp rax,r10
jne bad
cmp qword [rsp+8],7
jne bad
mov rax,[rsp+16]
mov r10,0x4000000000000000
cmp rax,r10
jne bad
"#),
    );
    endpoints(
        &mut cases,
        "aligned_overflow",
        "struct S{long double x;};",
        "long a,long b,long c,long d,long e,long f,long spill,struct S s,long last,double fp",
        "1,2,3,4,5,6,11,s,12,4.0",
        "struct S s={3.0L};",
        "a+b+c+d+e+f==21&&spill==11&&s.x==3.0L&&last==12&&fp==4.0",
        &format!(
            r#"{gp_setup}mov r9d,6
mov qword [rsp],11
mov rax,0xc000000000000000
mov [rsp+16],rax
mov qword [rsp+24],0x4000
mov qword [rsp+32],12
mov rax,0x4010000000000000
movq xmm0,rax
"#
        ),
        &format!(
            r#"{gp_check}cmp r9d,6
jne bad
cmp qword [rsp+8],11
jne bad
mov rax,[rsp+24]
mov r10,0xc000000000000000
cmp rax,r10
jne bad
cmp word [rsp+32],0x4000
jne bad
cmp qword [rsp+40],12
jne bad
movq rax,xmm0
mov r10,0x4010000000000000
cmp rax,r10
jne bad
mov rax,rsp
and eax,15
cmp eax,8
jne bad
"#
        ),
    );
    let decl = "struct S{long x,y;};struct R{long x,y,z;};";
    let params = "long a,long b,long c,long d,long e,long spill,struct S s";
    let source = format!(
        "{decl}struct R target({params}){{struct R r={{a+b+c+d+e,spill,s.x+s.y}};return r;}}"
    );
    let body = r#"lea rdi,[rsp+48]
mov esi,1
mov edx,2
mov ecx,3
mov r8d,4
mov r9d,5
mov qword [rsp],6
mov qword [rsp+8],7
mov qword [rsp+16],8
mov qword [rsp+72],99
call target
lea r10,[rsp+48]
cmp rax,r10
jne bad
cmp qword [rsp+48],15
jne bad
cmp qword [rsp+56],6
jne bad
cmp qword [rsp+64],15
jne bad
cmp qword [rsp+72],99
jne bad
"#;
    cases.push(Case::new(
        "aggregate_limits_sret_in",
        "hidden result pointer shifts GP and stack argument allocation",
        source,
        Expectation::AssemblyHarness(harness(body, "")),
    ));
    let source = format!(
        "{decl}struct R foreign({params});\
        int target(void){{\
            struct S s={{\
                7,8}};\
            struct R r=foreign(1,2,3,4,5,6,s);\
            return r.x==15&&r.y==6&&r.z==15;\
        }}"
    );
    let foreign = r#"global foreign
foreign:
cmp esi,1
jne bad
cmp edx,2
jne bad
cmp ecx,3
jne bad
cmp r8d,4
jne bad
cmp r9d,5
jne bad
cmp qword [rsp+8],6
jne bad
cmp qword [rsp+16],7
jne bad
cmp qword [rsp+24],8
jne bad
mov qword [rdi],15
mov qword [rdi+8],6
mov qword [rdi+16],15
mov rax,rdi
ret
"#;
    cases.push(Case::new(
        "aggregate_limits_sret_out",
        "independent callee verifies hidden result pointer and spills",
        source,
        Expectation::AssemblyHarness(harness("call target\ncmp eax,1\njne bad\n", foreign)),
    ));
    cases
}
