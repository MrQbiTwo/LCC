; Emit an expression result into RAX, normalized to its declared target width.
generate_expression:
    push rbx
    push r12
    push r13
    push r14
    push r15
    mov rbx, rdi
    test rbx, rbx
    jz .done
    mov rax, [rbx+N_TOKEN]
    mov [diag_token], rax
    mov eax, [rbx+N_KIND]
    cmp eax, A_STRING
    je .string
    cmp eax, A_NUM
    je .number
    cmp eax, A_VAR
    je .variable
    cmp eax, A_MEMBER
    je .variable
    cmp eax, A_DEREF
    je .variable
    cmp eax, A_ADDR
    je .address
    cmp eax, A_CAST
    je .cast
    cmp eax, A_BINARY
    je .binary
    cmp eax, A_ASSIGN
    je .assign
    cmp eax, A_UNARY
    je .unary
    cmp eax, A_POST
    je .increment
    cmp eax, A_CALL
    je .call
    cmp eax, A_VARARG
    je .vararg
    cmp eax, A_COND
    je .conditional
    cmp eax, A_COMMA
    je .comma
    jmp generation_error
.string:
    mov rdi, rbx
    call generate_string
    jmp .done
.number:
    mov rsi, [rbx+N_VALUE]
    call emit_value_bits
    mov rax, [rbx+N_TYPE]
    cmp dword [rax+TY_KIND], K_LDOUBLE
    jne .normalize
    lea rdi, [p_mov_edx]
    mov rsi, [rbx+N_AUX]
    mov edx, 4
    call emit_immediate
    jmp .normalize
.variable:
    mov rdi, rbx
    call generate_address
    mov rdi, rbx
    call emit_node_load
    jmp .done
.address:
    mov rdi, [rbx+N_LEFT]
    call generate_address
    jmp .done
.cast:
    mov rdi, [rbx+N_LEFT]
    call generate_expression
    mov rax, [rbx+N_LEFT]
    mov rdi, [rax+N_TYPE]
    mov rsi, [rbx+N_TYPE]
    call generate_convert
    jmp .done
.comma:
    mov rdi, [rbx+N_LEFT]
    call generate_expression
    mov rdi, [rbx+N_RIGHT]
    call generate_expression
    jmp .done
.call:
    mov rdi, rbx
    call generate_call
    jmp .done
.vararg:
    mov rdi, rbx
    call generate_vararg
    jmp .done
.binary:
    mov eax, [rbx+N_OP]
    cmp eax, T_AND
    je .logical
    cmp eax, T_OR
    je .logical
    mov rdi, [rbx+N_LEFT]
    call generate_expression
    mov rsi, [rbx+N_AUX]
    test rsi, rsi
    jz .binary_push
    mov rax, [rbx+N_LEFT]
    mov rdi, [rax+N_TYPE]
    call generate_convert
.binary_push:
    call emit_push_value
    mov rdi, [rbx+N_RIGHT]
    call generate_expression
    mov rsi, [rbx+N_AUX]
    test rsi, rsi
    jz .binary_pop
    mov rax, [rbx+N_RIGHT]
    mov rdi, [rax+N_TYPE]
    call generate_convert
.binary_pop:
    call emit_pop_rcx
    lea rdi, [i_exchange]
    call emit
    lea rdi, [i_exchange_high]
    call emit
    mov rdi, rbx
    call generate_binary_operation
    jmp .normalize
.assign:
    mov rdi, [rbx+N_LEFT]
    call generate_address
    call emit_push_value
    cmp dword [rbx+N_OP], '='
    je .assign_rhs
    mov rdi, [rbx+N_LEFT]
    call emit_node_load
    mov rdi, [rbx+N_TYPE]
    mov rsi, [rbx+N_AUX]
    call generate_convert
    call emit_push_value
.assign_rhs:
    mov rdi, [rbx+N_RIGHT]
    call generate_expression
    cmp dword [rbx+N_OP], '='
    je .assign_simple
    mov rax, [rbx+N_RIGHT]
    mov rdi, [rax+N_TYPE]
    mov rsi, [rbx+N_AUX]
    call generate_convert
    call emit_pop_rcx
    lea rdi, [i_exchange]
    call emit
    lea rdi, [i_exchange_high]
    call emit
    mov rdi, rbx
    call generate_binary_operation
    mov rdi, [rbx+N_AUX]
    jmp .assign_convert
.assign_simple:
    mov rax, [rbx+N_RIGHT]
    mov rdi, [rax+N_TYPE]
.assign_convert:
    mov rsi, [rbx+N_TYPE]
    call generate_convert
.assign_store:
    call emit_pop_rcx
    mov rdi, [rbx+N_LEFT]
    call emit_node_store
    jmp .done
.unary:
    mov eax, [rbx+N_OP]
    cmp eax, T_INC
    je .increment
    cmp eax, T_DEC
    je .increment
    mov rdi, [rbx+N_LEFT]
    call generate_expression
    mov eax, [rbx+N_OP]
    cmp eax, '-'
    je .negate
    cmp eax, '~'
    je .complement
    cmp eax, '!'
    je .not
    jmp .normalize
.negate:
    mov rax, [rbx+N_TYPE]
    mov eax, [rax+TY_KIND]
    lea rdi, [i_neg]
    cmp eax, K_FLOAT
    jne .neg_double
    lea rdi, [i_f32_neg]
.neg_double:
    cmp eax, K_DOUBLE
    jne .neg_extended
    lea rdi, [i_f64_neg]
.neg_extended:
    cmp eax, K_LDOUBLE
    jne .neg_emit
    lea rdi, [i_ld_neg]
.neg_emit:
    call emit
    jmp .normalize
.complement:
    lea rdi, [i_not]
    call emit
    jmp .normalize
.not:
    mov rax, [rbx+N_LEFT]
    mov rdi, [rax+N_TYPE]
    call generate_truth
    lea rdi, [i_eq]
    call emit
    lea rdi, [i_bool]
    call emit
    jmp .done
.increment:
    mov rdi, [rbx+N_LEFT]
    call generate_address
    call emit_push_value
    mov rdi, [rbx+N_LEFT]
    call emit_node_load
    cmp dword [rbx+N_KIND], A_POST
    jne .modify
    call emit_push_value
.modify:
    mov r12d, 1
    mov rax, [rbx+N_TYPE]
    mov ecx, [rax+TY_KIND]
    cmp ecx, K_FLOAT
    jb .integer_increment
    cmp ecx, K_LDOUBLE
    jbe .floating_increment
.integer_increment:
    cmp dword [rax+TY_KIND], K_PTR
    jne .delta
    mov rax, [rax+TY_BASE]
    mov r12d, [rax+TY_BYTES]
.delta:
    lea rdi, [p_add_rax]
    cmp dword [rbx+N_OP], T_INC
    je .add_delta
    lea rdi, [p_sub_rax]
.add_delta:
    mov rsi, r12
    mov edx, 4
    call emit_immediate
    jmp .increment_normalize
.floating_increment:
    mov r12d, ecx
    lea rdi, [p_mov_rcx]
    mov rsi, 0x3f800000
    cmp r12d, K_FLOAT
    je .one
    mov rsi, 0x3ff0000000000000
    cmp r12d, K_DOUBLE
    je .one
    mov rsi, 0x8000000000000000
.one:
    mov edx, 8
    call emit_immediate
    cmp r12d, K_LDOUBLE
    jne .fp_increment_op
    lea rdi, [i_ld_one_high]
    call emit
.fp_increment_op:
    sub r12d, K_FLOAT
    imul r12d, 5
    cmp dword [rbx+N_OP], T_INC
    je .fp_increment_emit
    inc r12d
.fp_increment_emit:
    lea rax, [fp_operation_table]
    mov rdi, [rax+r12*8]
    call emit
.increment_normalize:
    mov rdi, [rbx+N_TYPE]
    call emit_normalize
    cmp dword [rbx+N_KIND], A_POST
    jne .increment_store
    lea rdi, [i_post_address]
    call emit
    mov rdi, [rbx+N_LEFT]
    call emit_node_store
    call emit_pop_value
    lea rdi, [p_stack_add]
    mov esi, 16
    mov edx, 4
    call emit_immediate
    jmp .done
.increment_store:
    call emit_pop_rcx
    mov rdi, [rbx+N_LEFT]
    call emit_node_store
    jmp .done
.logical:
    xor edi, edi
    call label_new
    mov r12, rax
    xor edi, edi
    call label_new
    mov r13, rax
    mov rdi, [rbx+N_LEFT]
    call generate_test_expression
    lea rdi, [p_jz]
    cmp dword [rbx+N_OP], T_AND
    je .logical_branch
    lea rdi, [p_jnz]
.logical_branch:
    mov rsi, r12
    call emit_branch
    mov rdi, [rbx+N_RIGHT]
    call generate_test_expression
    lea rdi, [i_ne]
    call emit
    lea rdi, [i_bool]
    call emit
    lea rdi, [p_jump]
    mov rsi, r13
    call emit_branch
    mov rdi, r12
    call label_define
    xor esi, esi
    cmp dword [rbx+N_OP], T_AND
    sete sil
    xor esi, 1
    lea rdi, [p_mov64]
    mov edx, 8
    call emit_immediate
    mov rdi, r13
    call label_define
    jmp .done
.conditional:
    xor edi, edi
    call label_new
    mov r12, rax
    xor edi, edi
    call label_new
    mov r13, rax
    mov rdi, [rbx+N_COND]
    call generate_test_expression
    lea rdi, [p_jz]
    mov rsi, r12
    call emit_branch
    mov rdi, [rbx+N_THEN]
    call generate_expression
    mov rax, [rbx+N_THEN]
    mov rdi, [rax+N_TYPE]
    mov rsi, [rbx+N_TYPE]
    call generate_convert
    lea rdi, [p_jump]
    mov rsi, r13
    call emit_branch
    mov rdi, r12
    call label_define
    mov rdi, [rbx+N_ELSE]
    call generate_expression
    mov rax, [rbx+N_ELSE]
    mov rdi, [rax+N_TYPE]
    mov rsi, [rbx+N_TYPE]
    call generate_convert
    mov rdi, r13
    call label_define
    jmp .done
.normalize:
    mov rdi, [rbx+N_TYPE]
    call emit_normalize
.done:
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
