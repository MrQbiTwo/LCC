// Independent NASM endpoints specify expected System V classes and raw payloads.

// A chunk names its ABI class explicitly; no compiler classifier is reused here.
#[derive(Clone, Copy)]
pub(super) enum Class {
    Integer,
    Sse,
}
#[derive(Clone, Copy)]
pub(super) struct Chunk {
    pub(super) class: Class,
    pub(super) bits: u64,
    pub(super) bytes: u32,
}
pub(super) enum Layout {
    Registers(Vec<Chunk>),
    Memory(Vec<u64>, u32),
    Extended(u64, u16),
}
pub(super) struct Shape {
    pub(super) name: &'static str,
    pub(super) declaration: &'static str,
    pub(super) init: &'static str,
    pub(super) check: &'static str,
    pub(super) layout: Layout,
}

// Keep nondefault control modes and verify that every probe consumes ST0 results.
pub(super) fn driver(body: &str, foreign: &str) -> String {
    format!(
        r#"bits 64
default rel
section .text
global _start
extern target
_start:
cld
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
jmp finish
bad: mov edi,1
finish: mov eax,60
syscall
probe:
sub rsp,72
{body}
xor eax,eax
jmp .done
.bad: mov eax,1
.done: add rsp,72
ret
{foreign}
section .note.GNU-stack noalloc noexec nowrite progbits
"#
    )
}

// Assert only significant object bytes; padding and unused register bits vary.
pub(super) fn compare(bits: u64, bytes: u32) -> String {
    let mask = if bytes == 8 {
        String::new()
    } else {
        format!("mov r10,0x{:x}\nand rax,r10\n", (1u64 << (bytes * 8)) - 1)
    };
    format!("{mask}mov r10,0x{bits:x}\ncmp rax,r10\njne .bad\n")
}

// Explicit register expectations are independent of LCC's incoming/outgoing code.
pub(super) fn registers(chunks: &[Chunk], result: bool, write: bool) -> String {
    let mut text = String::new();
    let mut gp = 0;
    let mut fp = 0;
    if result && !write {
        text += "mov r8,rax\nmov r9,rdx\n";
    }
    for chunk in chunks {
        let reg = match chunk.class {
            Class::Integer => {
                let r = if result {
                    if write {
                        ["rax", "rdx"][gp]
                    } else {
                        ["r8", "r9"][gp]
                    }
                } else {
                    ["rdi", "rsi"][gp]
                };
                gp += 1;
                r.to_owned()
            }
            Class::Sse => {
                let r = format!("xmm{fp}");
                fp += 1;
                r
            }
        };
        if write {
            // R11 avoids destroying an already prepared RAX result.
            text += &format!("mov r11,0x{:x}\n", chunk.bits);
            text += &format!(
                "{} {reg},r11\n",
                if matches!(chunk.class, Class::Sse) {
                    "movq"
                } else {
                    "mov"
                }
            );
        } else {
            text += &format!(
                "{} rax,{reg}\n",
                if matches!(chunk.class, Class::Sse) {
                    "movq"
                } else {
                    "mov"
                }
            );
            text += &compare(chunk.bits, chunk.bytes);
        }
    }
    text
}

// Supply or inspect a stack object with hand-specified eightbyte contents.
pub(super) fn memory(
    words: &[u64],
    base: &str,
    start: usize,
    write: bool,
    tail_bytes: u32,
) -> String {
    let mut text = String::new();
    for (i, bits) in words.iter().enumerate() {
        let offset = start + i * 8;
        if write {
            text += &format!("mov r11,0x{bits:x}\nmov [{base}+{offset}],r11\n");
        } else {
            text += &format!("mov rax,[{base}+{offset}]\n");
            text += &compare(*bits, if i + 1 == words.len() { tail_bytes } else { 8 });
        }
    }
    text
}

// Check all four compiler boundaries separately so symmetric defects cannot cancel.
