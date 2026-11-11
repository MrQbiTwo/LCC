; Native Linux process entry; all compiler modules are assembled by NASM.
bits 64
default rel
%include "src/internal/layout.inc"
section .text
global _start
; Establish the process stack and dispatch the command-line driver.
_start:
    cld
    fninit
    ldmxcsr [compiler_mxcsr]
    xor ebp, ebp
    mov rdi, [rsp]
    lea rsi, [rsp+8]
    and rsp, -16
    call cli_main
    mov edi, eax
    mov eax, 60
    syscall
    ud2
%include "src/core/arena.asm"
%include "src/core/diag.asm"
%include "src/core/symbol.asm"
%include "src/host/linux_x86_64/sys.asm"
%include "src/driver/cli.asm"
%include "src/frontend/source.asm"
%include "src/frontend/lex.asm"
%include "src/frontend/preprocess.asm"
%include "src/frontend/type.asm"
%include "src/frontend/aggregate.asm"
%include "src/frontend/initializer.asm"
%include "src/frontend/vararg.asm"
%include "src/frontend/parse.asm"
%include "src/target/x86_64/encode.asm"
%include "src/target/x86_64/asm_text.asm"
%include "src/target/x86_64/float.asm"
%include "src/target/x86_64/abi_type.asm"
%include "src/target/x86_64/abi.asm"
%include "src/target/x86_64/abi_value.asm"
%include "src/target/x86_64/vararg.asm"
%include "src/target/x86_64/bitfield.asm"
%include "src/target/x86_64/select.asm"
%include "src/ir/lower.asm"
%include "src/ir/runtime.asm"
%include "src/ir/initializer.asm"
%include "src/format/elf64.asm"
%include "src/core/number.asm"
%include "src/core/constant.asm"
%include "runtime/linux_x86_64/decimal.asm"
section .rodata
compiler_mxcsr: dd 0x1f80
section .note.GNU-stack noalloc noexec nowrite progbits
