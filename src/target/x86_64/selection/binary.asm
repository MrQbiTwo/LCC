; Emit an operation with left value in RAX and right value in RCX.
generate_binary_operation:
    push rbx
    push r12
    push r13
    mov rbx, rdi
    mov r12d, [rbx+N_OP]
    cmp r12d, T_ADD_EQ
    jb .mapped
    cmp r12d, T_SHR_EQ
    ja .mapped
    sub r12d, T_ADD_EQ
    lea rax, [compound_ops]
    mov r12d, [rax+r12*4]
.mapped:
    mov rax, [rbx+N_AUX]
    test rax, rax
    jnz .common
    mov rax, [rbx+N_TYPE]
.common:
    cmp dword [rax+TY_KIND], K_FLOAT
    jb .integer
    cmp dword [rax+TY_KIND], K_LDOUBLE
    ja .integer
    mov rdi, rbx
    mov esi, r12d
    call generate_fp_binary
    jmp .done
.integer:
    mov r13d, [rax+TY_FLAGS]
    cmp r12d, '+'
    je .pointer_scale
    cmp r12d, '-'
    jne .operation
.pointer_scale:
    mov rax, [rbx+N_LEFT]
    mov rax, [rax+N_TYPE]
    cmp dword [rax+TY_KIND], K_PTR
    je .scale
    cmp dword [rax+TY_KIND], K_ARRAY
    je .scale
    cmp r12d, '+'
    jne .operation
    mov rax, [rbx+N_RIGHT]
    mov rax, [rax+N_TYPE]
    cmp dword [rax+TY_KIND], K_PTR
    je .reverse_scale
    cmp dword [rax+TY_KIND], K_ARRAY
    jne .operation
.reverse_scale:
    push rax
    sub rsp, 8
    lea rdi, [i_exchange]
    call emit
    add rsp, 8
    pop rax
.scale:
    cmp r12d, '-'
    jne .scale_value
    mov rdx, [rbx+N_RIGHT]
    mov rdx, [rdx+N_TYPE]
    cmp dword [rdx+TY_KIND], K_PTR
    je .pointer_difference
    cmp dword [rdx+TY_KIND], K_ARRAY
    je .pointer_difference
.scale_value:
    mov rax, [rax+TY_BASE]
    mov esi, [rax+TY_BYTES]
    lea rdi, [p_scale_rcx]
    mov edx, 4
    call emit_immediate
    jmp .operation
.pointer_difference:
    mov rax, [rax+TY_BASE]
    mov r13d, [rax+TY_BYTES]
    test r13d, r13d
    jz generation_error
    lea rdi, [i_sub]
    call emit
    lea rdi, [p_mov_rcx]
    mov rsi, r13
    mov edx, 8
    call emit_immediate
    lea rdi, [i_cqo]
    call emit
    lea rdi, [i_div]
    call emit
    jmp .done
.operation:
    cmp r12d, '+'
    je .add
    cmp r12d, '-'
    je .sub
    cmp r12d, '*'
    je .mul
    cmp r12d, '/'
    je .div
    cmp r12d, '%'
    je .div
    cmp r12d, '&'
    je .and
    cmp r12d, '|'
    je .or
    cmp r12d, '^'
    je .xor
    cmp r12d, T_SHL
    je .shl
    cmp r12d, T_SHR
    je .shr
    lea rdi, [i_cmp]
    call emit
    lea rdi, [i_eq]
    cmp r12d, T_EQ
    je .compare
    lea rdi, [i_ne]
    cmp r12d, T_NE
    je .compare
    test r13d, 1
    jnz .unsigned_compare
    lea rdi, [i_lt]
    cmp r12d, '<'
    je .compare
    lea rdi, [i_le]
    cmp r12d, T_LE
    je .compare
    lea rdi, [i_gt]
    cmp r12d, '>'
    je .compare
    lea rdi, [i_ge]
    cmp r12d, T_GE
    je .compare
    jmp generation_error
.unsigned_compare:
    lea rdi, [i_b]
    cmp r12d, '<'
    je .compare
    lea rdi, [i_be]
    cmp r12d, T_LE
    je .compare
    lea rdi, [i_a]
    cmp r12d, '>'
    je .compare
    lea rdi, [i_ae]
    cmp r12d, T_GE
    jne generation_error
.compare:
    call emit
    lea rdi, [i_bool]
    jmp .emit
.add:
    lea rdi, [i_add]
    jmp .emit
.sub:
    lea rdi, [i_sub]
    jmp .emit
.mul:
    lea rdi, [i_mul]
    jmp .emit
.and:
    lea rdi, [i_and]
    jmp .emit
.or:
    lea rdi, [i_or]
    jmp .emit
.xor:
    lea rdi, [i_xor]
    jmp .emit
.shl:
    lea rdi, [i_shl]
    jmp .emit
.shr:
    lea rdi, [i_sar]
    test r13d, 1
    jz .emit
    lea rdi, [i_shr]
    jmp .emit
.div:
    lea rdi, [i_cqo]
    test r13d, 1
    jz .division
    lea rdi, [i_zero_rdx]
.division:
    call emit
    lea rdi, [i_div]
    test r13d, 1
    jz .divide
    lea rdi, [i_udiv]
.divide:
    call emit
    cmp r12d, '%'
    jne .done
    lea rdi, [i_remainder]
.emit:
    call emit
.done:
    pop r13
    pop r12
    pop rbx
    ret
; Reject an unsupported typed operation instead of silently emitting wrong code.
generation_error:
    lea rdi, [msg_type]
    jmp fatal
section .rodata
compound_ops: dd '+','-','*','/','%','&','|','^',T_SHL,T_SHR
instruction i_r11_rax, '    mov rax,r11', 0x4c,0x89,0xd8
instruction i_ld_one_high, '    mov r8d,0x3fff', 0x41,0xb8,0xff,0x3f,0,0
