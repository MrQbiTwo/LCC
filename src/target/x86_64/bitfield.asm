; Bit-field accesses touch only occupied bytes and retain neighboring bits.
section .rodata
va_instruction i_bf_zero, '    xor r9d,r9d', xor r9d,r9d
va_instruction i_bf_collect, '    or r9,rax', or r9,rax
va_instruction i_bf_value, '    mov rax,r9', mov rax,r9
va_instruction i_bf_bits, '    mov r9,rax', mov r9,rax
va_instruction i_bf_result, '    mov r8,rax', mov r8,rax
va_instruction i_bf_return, '    mov rax,r8', mov rax,r8
va_instruction i_bf_old, '    movzx edx,byte [r11]', movzx edx,byte [r11]
va_instruction i_bf_merge, '    or rax,rdx', or rax,rdx
va_instruction i_bf_store, '    mov byte [r11],al', mov byte [r11],al
prefix p_bf_byte, '    movzx eax,byte [r11+', 0x41,0x0f,0xb6,0x83
prefix p_bf_destination, '    lea r11,[rcx+', 0x4c,0x8d,0x99
prefix p_bf_shl, '    shl rax,strict byte ', 0x48,0xc1,0xe0
prefix p_bf_shr, '    shr rax,strict byte ', 0x48,0xc1,0xe8
prefix p_bf_sar, '    sar rax,strict byte ', 0x48,0xc1,0xf8
prefix p_bf_mask, '    and rax,strict dword ', 0x48,0x25
prefix p_bf_preserve, '    and edx,strict dword ', 0x81,0xe2
section .text
; Load an addressed node RDI using its member width when appropriate.
emit_node_load:
    cmp dword [rdi+N_KIND], A_MEMBER
    jne .ordinary
    mov rax, [rdi+N_SYM]
    cmp dword [rax+MEM_BITS], 0
    je .ordinary
    mov rdi, rax
    jmp emit_bitfield_load
.ordinary:
    mov rdi, [rdi+N_TYPE]
    jmp emit_load
; Store through generated RCX and retain the converted assignment result.
emit_node_store:
    cmp dword [rdi+N_KIND], A_MEMBER
    jne .ordinary
    mov rax, [rdi+N_SYM]
    cmp dword [rax+MEM_BITS], 0
    je .ordinary
    mov rdi, rax
    jmp emit_bitfield_store
.ordinary:
    mov rdi, [rdi+N_TYPE]
    jmp emit_store
; Truncate generated RAX to member RDI's width, then extend its declared sign.
emit_bitfield_normalize:
    push rbx
    mov rbx, rdi
    lea rdi, [p_bf_shl]
    mov esi, 64
    sub esi, [rbx+MEM_BITS]
    mov edx, 1
    call emit_immediate
    lea rdi, [p_bf_sar]
    mov rax, [rbx+MEM_TYPE]
    test dword [rax+TY_FLAGS], 1
    jz .shift
    lea rdi, [p_bf_shr]
.shift:
    mov esi, 64
    sub esi, [rbx+MEM_BITS]
    mov edx, 1
    call emit_immediate
    pop rbx
    ret
; Load member RDI from generated RAX without reading surrounding object bytes.
emit_bitfield_load:
    push rbx
    push r12
    push r13
    mov rbx, rdi
    mov r13d, [rbx+MEM_SHIFT]
    add r13d, [rbx+MEM_BITS]
    add r13d, 7
    shr r13d, 3
    lea rdi, [i_rax_r11]
    call emit
    lea rdi, [i_bf_zero]
    call emit
    xor r12d, r12d
.byte:
    lea rdi, [p_bf_byte]
    mov rsi, r12
    call emit_offset_load
    lea rdi, [p_bf_shl]
    lea rsi, [r12*8]
    mov edx, 1
    call emit_immediate
    lea rdi, [i_bf_collect]
    call emit
    inc r12
    cmp r12, r13
    jb .byte
    lea rdi, [i_bf_value]
    call emit
    lea rdi, [p_bf_shr]
    mov esi, [rbx+MEM_SHIFT]
    mov edx, 1
    call emit_immediate
    mov rdi, rbx
    call emit_bitfield_normalize
    pop r13
    pop r12
    pop rbx
    ret
; Ordinary writes preserve all other bits in each occupied byte.
emit_bitfield_store:
    mov rsi, -1
    jmp emit_bitfield_initializer
; Store member RDI from generated RAX through RCX; RSI selects preserved bits.
; Initializers preserve prior low bits only, initializing other bits with zero.
emit_bitfield_initializer:
    push rbx
    push r12
    push r13
    push r14
    push r15
    mov rbx, rdi
    mov r12, rsi
    mov ecx, [rbx+MEM_BITS]
    mov r14d, 1
    shl r14, cl
    dec r14
    mov ecx, [rbx+MEM_SHIFT]
    shl r14, cl
    mov rax, r14
    not rax
    and r12, rax
    mov r15d, [rbx+MEM_SHIFT]
    add r15d, [rbx+MEM_BITS]
    add r15d, 7
    shr r15d, 3
    call emit_bitfield_normalize
    lea rdi, [i_bf_result]
    call emit
    lea rdi, [p_bf_shl]
    mov esi, [rbx+MEM_SHIFT]
    mov edx, 1
    call emit_immediate
    lea rdi, [i_bf_bits]
    call emit
    xor r13d, r13d
.byte:
    lea rdi, [p_bf_destination]
    mov rsi, r13
    call emit_offset_load
    lea rdi, [i_bf_value]
    call emit
    lea rdi, [p_bf_shr]
    lea rsi, [r13*8]
    mov edx, 1
    call emit_immediate
    lea rdi, [p_bf_mask]
    movzx esi, r14b
    mov edx, 4
    call emit_immediate
    test r12b, r12b
    jz .write
    lea rdi, [i_bf_old]
    call emit
    lea rdi, [p_bf_preserve]
    movzx esi, r12b
    mov edx, 4
    call emit_immediate
    lea rdi, [i_bf_merge]
    call emit
.write:
    lea rdi, [i_bf_store]
    call emit
    shr r12, 8
    shr r14, 8
    inc r13
    cmp r13, r15
    jb .byte
    lea rdi, [i_bf_return]
    call emit
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
