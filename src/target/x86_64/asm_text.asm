; NASM rendering uses the same instruction records and labels as native output.
section .rodata
asm_header: db 'bits 64',10,'default rel',10,'section .text',10,0
asm_data: db 10,'section .data align=4096',10,0
asm_stack: db 10,'section .note.GNU-stack noalloc noexec nowrite progbits',10,0
asm_global: db 'global ',0
asm_extern: db 'extern ',0
asm_dollar: db '$',0
asm_local: db '..@lcc',0
asm_colon: db ':',10,0
asm_bracket: db ']',10,0
asm_zero: db '    times ',0
asm_zero_tail: db ' db 0',10,0
asm_dq: db '    dq ',0
asm_dd: db '    dd ',0
asm_dw: db '    dw ',0
asm_db: db '    db ',0
hex_digits: db '0123456789abcdef'
section .text
; Append the zero-terminated string RSI to the assembly output.
asm_string:
    cmp dword [assembly_mode], 0
    je .done
    lea rdi, [asm_buffer]
    jmp buffer_string
.done:
    ret
; End a generated assembly line.
asm_newline:
    lea rsi, [msg_newline]
    jmp asm_string
; Render unsigned integer RDI in decimal.
asm_unsigned:
    cmp dword [assembly_mode], 0
    je .done
    sub rsp, 40
    mov rsi, rsp
    call format_unsigned
    mov rdx, rax
    mov rsi, rsp
    lea rdi, [asm_buffer]
    call buffer_append
    add rsp, 40
.done:
    ret
; Render all 64 bits of RDI without signed-decimal ambiguity.
asm_hex:
    cmp dword [assembly_mode], 0
    je .done
    sub rsp, 40
    mov byte [rsp], '0'
    mov byte [rsp+1], 'x'
    lea r8, [hex_digits]
    mov rax, rdi
    mov ecx, 16
.loop:
    mov rdx, rax
    and edx, 15
    mov dl, [r8+rdx]
    mov [rsp+rcx+1], dl
    shr rax, 4
    loop .loop
    lea rdi, [asm_buffer]
    mov rsi, rsp
    mov edx, 18
    call buffer_append
    add rsp, 40
.done:
    ret
; Render a label RDI, escaping public names that NASM treats as reserved words.
asm_label_name:
    cmp dword [assembly_mode], 0
    je .absent
    push rbx
    mov rbx, rdi
    cmp qword [rbx+L_NAME], 0
    je .internal
    lea rsi, [asm_dollar]
    call asm_string
    mov rax, [rbx+L_NAME]
    mov edx, [rax+TK_LEN]
    mov rsi, [rax+TK_TEXT]
    lea rdi, [asm_buffer]
    call buffer_append
    jmp .done
.internal:
    lea rsi, [asm_local]
    call asm_string
    mov rdi, [rbx+L_ID]
    call asm_unsigned
.done:
    pop rbx
.absent:
    ret
