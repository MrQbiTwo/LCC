; Lower one statement node RDI, recursively preserving loop destinations.
generate_statement:
    push rbx
    push r12
    push r13
    push r14
    push r15
    sub rsp, 16
    mov rbx, rdi
    test rbx, rbx
    jz .done
    mov rax, [rbx+N_TOKEN]
    mov [diag_token], rax
    mov eax, [rbx+N_KIND]
    cmp eax, A_EXPR
    je .expression
    cmp eax, A_DECL
    je .declaration
    cmp eax, A_RETURN
    je .return
    cmp eax, A_BLOCK
    je .block
    cmp eax, A_IF
    je .if
    cmp eax, A_FOR
    je .loop
    cmp eax, A_DO
    je .loop
    cmp eax, A_BREAK
    je .break
    cmp eax, A_CONTINUE
    je .continue
    cmp eax, A_SWITCH
    je .switch
    cmp eax, A_CASE
    je .label
    cmp eax, A_LABEL
    je .label
    cmp eax, A_GOTO
    je .goto
    jmp generation_error
.expression:
    mov rdi, [rbx+N_LEFT]
    call generate_expression
    jmp .done
.declaration:
    cmp qword [rbx+N_LEFT], 0
    je .done
    mov rdi, [rbx+N_SYM]
    mov rsi, [rbx+N_LEFT]
    call generate_local_initializer
    jmp .done
.return:
    mov rdi, [rbx+N_LEFT]
    test rdi, rdi
    jz .return_branch
    call generate_expression
    mov rax, [current_function]
    mov rax, [rax+S_TYPE]
    mov r12, [rax+TY_BASE]
    mov rax, [rbx+N_LEFT]
    mov rdi, [rax+N_TYPE]
    mov rsi, r12
    call generate_convert
    cmp dword [r12+TY_KIND], K_STRUCT
    jb .return_scalar
    mov rdi, r12
    call generate_aggregate_return
    jmp .return_branch
.return_scalar:
    mov eax, [r12+TY_KIND]
    lea rdi, [i_f32_to_xmm]
    cmp eax, K_FLOAT
    je .return_fp
    lea rdi, [i_f64_to_xmm]
    cmp eax, K_DOUBLE
    je .return_fp
    lea rdi, [i_ld_return]
    cmp eax, K_LDOUBLE
    jne .return_branch
.return_fp:
    call emit
.return_branch:
    lea rdi, [p_jump]
    mov rsi, [return_label]
    call emit_branch
    jmp .done
.block:
    mov r12, [rbx+N_LEFT]
.block_next:
    test r12, r12
    jz .done
    mov rdi, r12
    call generate_statement
    mov r12, [r12+N_NEXT]
    jmp .block_next
.if:
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
    call generate_statement
    lea rdi, [p_jump]
    mov rsi, r13
    call emit_branch
    mov rdi, r12
    call label_define
    mov rdi, [rbx+N_ELSE]
    call generate_statement
    mov rdi, r13
    call label_define
    jmp .done
.loop:
    mov rax, [break_label]
    mov [rsp], rax
    mov rax, [continue_label]
    mov [rsp+8], rax
    xor edi, edi
    call label_new
    mov r12, rax
    xor edi, edi
    call label_new
    mov r13, rax
    mov [continue_label], rax
    xor edi, edi
    call label_new
    mov r14, rax
    mov [break_label], rax
    mov rdi, [rbx+N_LEFT]
    call generate_expression
    mov rdi, r12
    call label_define
    cmp dword [rbx+N_KIND], A_DO
    je .body
    cmp qword [rbx+N_COND], 0
    je .body
    mov rdi, [rbx+N_COND]
    call generate_test_expression
    lea rdi, [p_jz]
    mov rsi, r14
    call emit_branch
.body:
    mov rdi, [rbx+N_THEN]
    call generate_statement
    mov rdi, r13
    call label_define
    cmp dword [rbx+N_KIND], A_DO
    je .do_condition
    mov rdi, [rbx+N_RIGHT]
    call generate_expression
    lea rdi, [p_jump]
    mov rsi, r12
    call emit_branch
    jmp .loop_end
.do_condition:
    mov rdi, [rbx+N_COND]
    call generate_test_expression
    lea rdi, [p_jnz]
    mov rsi, r12
    call emit_branch
.loop_end:
    mov rdi, r14
    call label_define
    mov rax, [rsp]
    mov [break_label], rax
    mov rax, [rsp+8]
    mov [continue_label], rax
    jmp .done
.break:
    lea rdi, [p_jump]
    mov rsi, [break_label]
    call emit_branch
    jmp .done
.continue:
    lea rdi, [p_jump]
    mov rsi, [continue_label]
    call emit_branch
    jmp .done
.label:
    mov rdi, [rbx+N_SYM]
    call label_define
    mov rdi, [rbx+N_LEFT]
    call generate_statement
    jmp .done
.goto:
    lea rdi, [p_jump]
    mov rsi, [rbx+N_SYM]
    call emit_branch
    jmp .done
.switch:
    mov rax, [break_label]
    mov [rsp], rax
    xor edi, edi
    call label_new
    mov r12, rax
    mov [break_label], rax
    mov rdi, [rbx+N_COND]
    call generate_expression
    mov rdi, [rbx+N_TYPE]
    call emit_normalize
    mov r13, [rbx+N_ARGS]
.switch_case:
    test r13, r13
    jz .switch_default
    lea rdi, [p_mov_rcx]
    mov rsi, [r13+N_VALUE]
    mov edx, 8
    call emit_immediate
    lea rdi, [i_cmp]
    call emit
    lea rdi, [p_jz]
    mov rsi, [r13+N_SYM]
    call emit_branch
    mov r13, [r13+N_ARGS]
    jmp .switch_case
.switch_default:
    mov rsi, r12
    mov rax, [rbx+N_ELSE]
    test rax, rax
    jz .switch_jump
    mov rsi, [rax+N_SYM]
.switch_jump:
    lea rdi, [p_jump]
    call emit_branch
    mov rdi, [rbx+N_THEN]
    call generate_statement
    mov rdi, r12
    call label_define
    mov rax, [rsp]
    mov [break_label], rax
.done:
    add rsp, 16
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
