// Independent NASM callers and callees detect symmetric compiler ABI mistakes.
use crate::{Case, Expectation};

// The driver supplies nondefault rounding controls and checks them after each probe.
fn driver(body: &str) -> String {
    format!(
        r#"bits 64
default rel
section .text
global _start
extern target
_start:
fninit
sub rsp,16
mov word [rsp],0x077f
fldcw [rsp]
mov dword [rsp+4],0x3f80
ldmxcsr [rsp+4]
call probe
test eax,eax
jnz bad
fnstcw [rsp]
cmp word [rsp],0x077f
jne bad
stmxcsr [rsp+4]
mov eax,[rsp+4]
and eax,0xffc0
cmp eax,0x3f80
jne bad
fnstsw ax
and eax,0x3800
jnz bad
xor edi,edi
jmp done
bad: mov edi,1
done: mov eax,60
syscall
{body}
section .note.GNU-stack noalloc noexec nowrite progbits
"#
    )
}

// Every expected register or stack value below is supplied independently of LCC.
pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    let mut add = |purpose: &str, source: &str, assembly: &str| {
        cases.push(Case::new(
            format!("fp_abi_{:03}", cases.len() + 1),
            purpose,
            source,
            Expectation::AssemblyHarness(driver(assembly)),
        ));
    };
    add(
        "external SSE caller and result",
        "double target(float a,double b,int c){return a+b+c;}",
        r#"probe: sub rsp,8
mov eax,0x3fc00000
movd xmm0,eax
mov rax,0x4004000000000000
movq xmm1,rax
mov edi,4
call target
movq rax,xmm0
mov rcx,0x4020000000000000
cmp rax,rcx
setne al
movzx eax,al
add rsp,8
ret"#,
    );
    add(
        "external binary80 caller and ST0 result",
        "long double target(long double a,long double b){return a-b;}",
        r#"probe: sub rsp,40
mov rax,0xf000000000000000
mov [rsp],rax
mov qword [rsp+8],0x4001
mov rax,0xa000000000000000
mov [rsp+16],rax
mov qword [rsp+24],0x4000
call target
fstp tword [rsp]
mov rax,0xa000000000000000
cmp [rsp],rax
jne .bad
cmp word [rsp+8],0x4001
jne .bad
xor eax,eax
jmp .done
.bad: mov eax,1
.done: add rsp,40
ret"#,
    );
    add(
        "external SSE callee",
        "double foreign(float a,double b,int c); int target(void){return foreign(1.5f,2.5,4)==8.0;}",
        r#"global foreign
foreign: movd eax,xmm0
cmp eax,0x3fc00000
jne .bad
movq rax,xmm1
mov rcx,0x4004000000000000
cmp rax,rcx
jne .bad
cmp edi,4
jne .bad
mov rax,0x4020000000000000
movq xmm0,rax
ret
.bad: pxor xmm0,xmm0
ret
probe: sub rsp,8
call target
xor eax,1
add rsp,8
ret"#,
    );
    add(
        "external binary80 callee",
        "long double foreign(long double a,int b); int target(void){return foreign(7.5L,3)==2.5L;}",
        r#"global foreign
foreign: mov rax,rsp
and eax,15
cmp eax,8
jne .bad
mov rax,0xf000000000000000
cmp [rsp+8],rax
jne .bad
cmp word [rsp+16],0x4001
jne .bad
cmp edi,3
jne .bad
fld tword [value]
ret
.bad: fldz
ret
probe: sub rsp,8
call target
xor eax,1
add rsp,8
ret
section .rodata
value: dq 0xa000000000000000,0x4000
section .text"#,
    );
    add(
        "ellipsis promotions and AL vector count",
        "int foreign(int a,...); int target(void){float x=1.5f; return foreign(7,x,2.5,3.5L);}",
        r#"global foreign
foreign: cmp al,2
jne .bad
cmp edi,7
jne .bad
movq rax,xmm0
mov rcx,0x3ff8000000000000
cmp rax,rcx
jne .bad
movq rax,xmm1
mov rcx,0x4004000000000000
cmp rax,rcx
jne .bad
mov rax,0xe000000000000000
cmp [rsp+8],rax
jne .bad
cmp word [rsp+16],0x4000
jne .bad
xor eax,eax
ret
.bad: mov eax,1
ret
probe: sub rsp,8
call target
add rsp,8
ret"#,
    );
    add(
        "repeated binary80 returns leave stack balanced",
        "long double target(long double x){return (x+1.0L)*2.0L;}",
        r#"probe: push r12
sub rsp,16
mov r12d,100
.loop: mov rax,0xc000000000000000
mov [rsp],rax
mov qword [rsp+8],0x3fff
call target
fstp tword [rsp]
mov rax,0xa000000000000000
cmp [rsp],rax
jne .bad
cmp word [rsp+8],0x4001
jne .bad
dec r12d
jnz .loop
xor eax,eax
jmp .done
.bad: mov eax,1
.done: add rsp,16
pop r12
ret"#,
    );
    add(
        "cast truncation ignores nondefault rounding",
        "int target(double a,long double b){return (int)a+(int)b;}",
        r#"probe: sub rsp,24
mov rax,0xc01f000000000000
movq xmm0,rax
mov rax,0xf800000000000000
mov [rsp],rax
mov qword [rsp+8],0x4001
call target
add rsp,24
ret"#,
    );
    add(
        "external caller with exhausted SSE bank",
        "double target(double a,double b,double c,double d,double e,double f,double g,double h,\
        double i,int j){\
            return a+b+c+d+e+f+g+h+i+j;\
        }",
        r#"probe: sub rsp,24
mov rax,0x3ff0000000000000
movq xmm0,rax
movq xmm1,rax
movq xmm2,rax
movq xmm3,rax
movq xmm4,rax
movq xmm5,rax
movq xmm6,rax
movq xmm7,rax
mov [rsp],rax
mov edi,3
call target
movq rax,xmm0
mov rcx,0x4028000000000000
cmp rax,rcx
setne al
movzx eax,al
add rsp,24
ret"#,
    );
    cases
}
