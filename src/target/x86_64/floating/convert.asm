section .rodata
prefix p_jns, '    jns near ', 0x0f,0x89
prefix p_jb, '    jb near ', 0x0f,0x82
prefix p_mov_edx, '    mov edx,strict dword ', 0xba
instruction i_post_address, '    mov rcx,qword [rsp+16]', 0x48,0x8b,0x4c,0x24,16
fp_conversion_table:
    dq 0,i_f32_to_f64,i_f32_to_ld
    dq i_f64_to_f32,0,i_f64_to_ld
    dq i_ld_to_f32,i_ld_to_f64,0
fp_operation_table:
    dq i_f32_add,i_f32_sub,i_f32_mul,i_f32_div,i_f32_compare
    dq i_f64_add,i_f64_sub,i_f64_mul,i_f64_div,i_f64_compare
    dq i_ld_add,i_ld_sub,i_ld_mul,i_ld_div,i_ld_compare
section .text
; Convert the generated value from type RDI to type RSI at an explicit boundary.
generate_convert:
    push rbx
    push r12
    push r13
    push r14
    push r15
    mov rbx, rdi
    mov r12, rsi
    cmp rbx, r12
    je .done
    mov r13d, [rbx+TY_KIND]
    mov r14d, [r12+TY_KIND]
    cmp r14d, K_VOID
    je .done
    cmp r13d, K_STRUCT
    jae .aggregate
    cmp r14d, K_STRUCT
    jae .aggregate
    cmp r14d, K_FLOAT
    jb .to_integer
    cmp r14d, K_LDOUBLE
    ja .to_integer
    cmp r13d, K_FLOAT
    jb .integer_to_fp
    cmp r13d, K_LDOUBLE
    ja generation_error
    sub r13d, K_FLOAT
    sub r14d, K_FLOAT
    lea eax, [r13+r13*2]
    add eax, r14d
    lea rdx, [fp_conversion_table]
    mov rdi, [rdx+rax*8]
    test rdi, rdi
    jz .done
    call emit
    jmp .done
.aggregate:
    mov rdi, rbx
    mov rsi, r12
    call type_aggregate_common
    jmp .done
.integer_to_fp:
    cmp r14d, K_LDOUBLE
    je .integer_to_ld
    cmp dword [rbx+TY_BYTES], 8
    jne .signed_to_fp
    test dword [rbx+TY_FLAGS], 1
    jz .signed_to_fp
    xor edi, edi
    call label_new
    mov r13, rax
    xor edi, edi
    call label_new
    mov r15, rax
    lea rdi, [i_test]
    call emit
    lea rdi, [p_jns]
    mov rsi, r13
    call emit_branch
    lea rdi, [i_u64_halve]
    call emit
    lea rdi, [i_i64_to_f32]
    cmp r14d, K_FLOAT
    je .half_convert
    lea rdi, [i_i64_to_f64]
.half_convert:
    call emit
    lea rdi, [i_f32_double]
    cmp r14d, K_FLOAT
    je .double
    lea rdi, [i_f64_double]
.double:
    call emit
    lea rdi, [p_jump]
    mov rsi, r15
    call emit_branch
    mov rdi, r13
    call label_define
    lea rdi, [i_i64_to_f32]
    cmp r14d, K_FLOAT
    je .positive_convert
    lea rdi, [i_i64_to_f64]
.positive_convert:
    call emit
    mov rdi, r15
    call label_define
    jmp .done
.signed_to_fp:
    lea rdi, [i_i64_to_f32]
    cmp r14d, K_FLOAT
    je .emit
    lea rdi, [i_i64_to_f64]
    jmp .emit
.integer_to_ld:
    lea rdi, [i_i64_to_ld]
    test dword [rbx+TY_FLAGS], 1
    jz .emit
    lea rdi, [i_u64_to_ld]
    jmp .emit
.to_integer:
    cmp r13d, K_FLOAT
    jb .integer_normalize
    cmp r13d, K_LDOUBLE
    ja .integer_normalize
    je .ld_integer
    cmp dword [r12+TY_BYTES], 8
    jne .signed_integer
    test dword [r12+TY_FLAGS], 1
    jz .signed_integer
    xor edi, edi
    call label_new
    mov r14, rax
    xor edi, edi
    call label_new
    mov r15, rax
    lea rdi, [p_mov_rcx]
    mov rsi, 0x5f000000
    cmp r13d, K_FLOAT
    je .threshold
    mov rsi, 0x43e0000000000000
.threshold:
    mov edx, 8
    call emit_immediate
    lea rdi, [i_f32_compare]
    cmp r13d, K_FLOAT
    je .compare_limit
    lea rdi, [i_f64_compare]
.compare_limit:
    call emit
    lea rdi, [p_jb]
    mov rsi, r14
    call emit_branch
    lea rdi, [i_f32_subtract_limit]
    cmp r13d, K_FLOAT
    je .subtract_limit
    lea rdi, [i_f64_subtract_limit]
.subtract_limit:
    call emit
    lea rdi, [i_set_high_bit]
    call emit
    lea rdi, [p_jump]
    mov rsi, r15
    call emit_branch
    mov rdi, r14
    call label_define
    lea rdi, [i_f32_xmm_to_i64]
    cmp r13d, K_FLOAT
    je .small_integer
    lea rdi, [i_f64_xmm_to_i64]
.small_integer:
    call emit
    mov rdi, r15
    call label_define
    jmp .integer_normalize
.signed_integer:
    lea rdi, [i_f32_to_i64]
    cmp r13d, K_FLOAT
    je .truncate
    lea rdi, [i_f64_to_i64]
.truncate:
    call emit
    jmp .integer_normalize
.ld_integer:
    lea rdi, [i_ld_to_i64]
    call emit
.integer_normalize:
    mov rdi, r12
    call emit_normalize
    jmp .done
.emit:
    call emit
.done:
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
