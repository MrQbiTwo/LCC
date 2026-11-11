; Render packed bytes and symbolic relocation slots as standalone NASM data.
section .rodata
runtime_text: db 10,'section .text',10,0
runtime_align: db 'align ',0
runtime_align_tail: db ', db 0',10,0
runtime_equ: db ' equ ',0
runtime_comma: db ',',0
runtime_relative: db ' - ($ + 4)',0
section .text
; Render section RDI, source bytes RSI and length RDX; native bytes already exist.
runtime_render:
    cmp dword [assembly_mode], 0
    je .absent
    push rbx
    push r12
    push r13
    push r14
    push r15
    sub rsp, 16
    mov [rsp], rdi
    mov [rsp+8], rdx
    mov r12, rsi
    xor r13d, r13d
    lea rbx, [runtime_relocations]
    mov r14d, runtime_relocation_count
.next:
    test r14, r14
    jz .tail
    mov rax, [rsp]
    cmp [rbx], rax
    jne .skip
    lea rdi, [r12+r13]
    mov rsi, [rbx+8]
    sub rsi, r13
    call runtime_render_bytes
    lea rsi, [asm_dd]
    cmp qword [rbx+32], 4
    je .directive
    lea rsi, [asm_dq]
.directive:
    call asm_string
    mov rax, [rbx+16]
    lea rdi, [runtime_bases-8]
    mov rdi, [rdi+rax*8]
    call asm_label_name
    mov r15, [rbx+24]
    add r15, [rbx+40]
    jo generation_error
    test r15, r15
    jz .relative
    lea rsi, [asm_address_plus]
    jns .addend
    lea rsi, [asm_address_minus]
    neg r15
.addend:
    call asm_string
    mov rdi, r15
    call asm_unsigned
.relative:
    cmp qword [rbx+32], 4
    jne .newline
    lea rsi, [runtime_relative]
    call asm_string
.newline:
    call asm_newline
    mov r13, [rbx+8]
    add r13, [rbx+32]
.skip:
    add rbx, 48
    dec r14
    jmp .next
.tail:
    lea rdi, [r12+r13]
    mov rsi, [rsp+8]
    sub rsi, r13
    call runtime_render_bytes
    add rsp, 16
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
.absent:
    ret

; Print span RDI/RSI as bounded db rows without changing native buffer positions.
runtime_render_bytes:
    push rbx
    push r12
    push r13
    mov rbx, rdi
    mov r12, rsi
    xor r13d, r13d
.byte:
    test r12, r12
    jz .done
    lea rsi, [asm_db]
    test r13, r13
    jz .prefix
    lea rsi, [runtime_comma]
.prefix:
    call asm_string
    movzx edi, byte [rbx]
    call asm_unsigned
    inc rbx
    dec r12
    inc r13
    cmp r13, 24
    jb .byte
    call asm_newline
    xor r13d, r13d
    jmp .byte
.done:
    test r13, r13
    jz .return
    call asm_newline
.return:
    pop r13
    pop r12
    pop rbx
    ret
