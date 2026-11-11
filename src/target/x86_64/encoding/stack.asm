; Additional scalar instructions used for expression stacks and calls.
section .rodata
instruction i_copy_setup, `    mov rsi,rax\n    mov rdi,rcx\n    mov r11,rcx`, 0x48,0x89,0xc6,0x48,0x89,0xcf,0x49,0x89,0xcb
instruction i_copy_bytes, `    rep movsb\n    mov rax,r11`, 0xf3,0xa4,0x4c,0x89,0xd8
instruction i_pop_rax, '    pop rax', 0x58
prefix p_store_rsp, '    mov qword [rsp+', 0x48,0x89,0x84,0x24
prefix p_add_rax, '    add rax,strict dword ', 0x48,0x05
prefix p_sub_rax, '    sub rax,strict dword ', 0x48,0x2d
prefix p_scale_rcx, '    imul rcx,rcx,strict dword ', 0x48,0x69,0xc9
prefix p_mov_rcx, '    mov rcx,strict qword ', 0x48,0xb9
asm_store_tail: db '],rax',10,0
section .text
; Push an expression value while retaining 16-byte call-site alignment.
emit_push_value:
    sub rsp, 8
    lea rdi, [i_push]
    call emit
    lea rdi, [i_push_rax]
    call emit
    lea rdi, [i_high_to_rsp]
    call emit
    add rsp, 8
    ret
; Pop an expression value into RCX.
emit_pop_rcx:
    sub rsp, 8
    lea rdi, [i_pop_rcx]
    call emit
    lea rdi, [i_left_high_from_rsp]
    call emit
    lea rdi, [i_unpad]
    call emit
    add rsp, 8
    ret
; Pop an expression value into RAX.
emit_pop_value:
    sub rsp, 8
    lea rdi, [i_pop_rax]
    call emit
    lea rdi, [i_high_from_rsp]
    call emit
    lea rdi, [i_unpad]
    call emit
    add rsp, 8
    ret
; Store RAX at generated RSP plus displacement RDI.
emit_stack_store:
    push rbx
    sub rsp, 16
    mov rbx, rdi
    lea rdi, [p_store_rsp]
    call emit
    mov [rsp], ebx
    lea rdi, [code_buffer]
    mov rsi, rsp
    mov edx, 4
    call buffer_append
    mov rdi, rbx
    call asm_unsigned
    lea rsi, [asm_store_tail]
    call asm_string
    add rsp, 16
    pop rbx
    ret
