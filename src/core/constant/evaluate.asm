; Route constant expressions through the same target-type boundaries as code.
evaluate_constant:
    push rbx
    push r12
    push r13
    sub rsp, 32
    mov rbx, rdi
    mov rax, [rbx+N_TOKEN]
    mov [diag_token], rax
    mov eax, [rbx+N_KIND]
    cmp eax, A_CAST
    je .cast
    cmp eax, A_UNARY
    je .unary
    cmp eax, A_COND
    je .conditional
    cmp eax, A_NUM
    je .literal
    cmp eax, A_BINARY
    jne .integer
    mov eax, [rbx+N_OP]
    cmp eax, T_AND
    je .logical
    cmp eax, T_OR
    je .logical
    mov rax, [rbx+N_AUX]
    mov eax, [rax+TY_KIND]
    cmp eax, K_FLOAT
    jb .integer
    cmp eax, K_LDOUBLE
    jbe .floating
.integer:
    mov rdi, rbx
    call evaluate_integer_constant
    jmp .done
.literal:
    mov rax, [rbx+N_VALUE]
    mov rdx, [rbx+N_AUX]
    jmp .done
.cast:
    mov r12, [rbx+N_LEFT]
    mov rdi, r12
    call evaluate_constant
    mov rdi, [r12+N_TYPE]
    mov rsi, [rbx+N_TYPE]
    call constant_convert
    jmp .done
.unary:
    mov r12, [rbx+N_LEFT]
    cmp dword [rbx+N_OP], '!'
    je .unary_value
    mov rax, [rbx+N_TYPE]
    cmp dword [rax+TY_KIND], K_FLOAT
    jb .integer
    cmp dword [rax+TY_KIND], K_LDOUBLE
    ja .integer
.unary_value:
    mov rdi, r12
    call evaluate_constant
    mov ecx, [rbx+N_OP]
    cmp ecx, '!'
    je .not
    mov rsi, [rbx+N_TYPE]
    mov rdi, [r12+N_TYPE]
    call constant_convert
    mov rdi, [rbx+N_TYPE]
    mov ecx, [rdi+TY_KIND]
    cmp ecx, K_FLOAT
    jb .integer_unary
    cmp ecx, K_LDOUBLE
    ja .integer_unary
    cmp dword [rbx+N_OP], '+'
    je .done
    cmp dword [rbx+N_OP], '-'
    jne generation_error
    cmp ecx, K_FLOAT
    je .neg_float
    cmp ecx, K_DOUBLE
    je .neg_double
    xor edx, 0x8000
    jmp .done
.neg_float:
    xor eax, 0x80000000
    jmp .done
.neg_double:
    btc rax, 63
    jmp .done
.integer_unary:
    mov rdi, rbx
    call evaluate_integer_constant
    jmp .done
.not:
    mov rdi, [r12+N_TYPE]
    call constant_truth
    xor eax, 1
    jmp .done
.conditional:
    mov r12, [rbx+N_COND]
    mov rdi, r12
    call evaluate_constant
    mov rdi, [r12+N_TYPE]
    call constant_truth
    test eax, eax
    mov r12, [rbx+N_THEN]
    cmovz r12, [rbx+N_ELSE]
    mov rdi, r12
    call evaluate_constant
    mov rdi, [r12+N_TYPE]
    mov rsi, [rbx+N_TYPE]
    call constant_convert
    jmp .done
.logical:
    mov r12, [rbx+N_LEFT]
    mov rdi, r12
    call evaluate_constant
    mov rdi, [r12+N_TYPE]
    call constant_truth
    cmp dword [rbx+N_OP], T_AND
    jne .or
    test eax, eax
    jz .done
    jmp .logical_right
.or:
    test eax, eax
    jnz .done
.logical_right:
    mov r12, [rbx+N_RIGHT]
    mov rdi, r12
    call evaluate_constant
    mov rdi, [r12+N_TYPE]
    call constant_truth
    jmp .done
.floating:
    mov r12, [rbx+N_AUX]
    mov r13, [rbx+N_LEFT]
    mov rdi, r13
    call evaluate_constant
    mov rdi, [r13+N_TYPE]
    mov rsi, r12
    call constant_convert
    mov [rsp], rax
    mov [rsp+8], rdx
    mov r13, [rbx+N_RIGHT]
    mov rdi, r13
    call evaluate_constant
    mov rdi, [r13+N_TYPE]
    mov rsi, r12
    call constant_convert
    mov [rsp+16], rax
    mov [rsp+24], rdx
    cmp dword [rbx+N_OP], '/'
    jne .nonzero
    mov rdi, r12
    call constant_truth
    test eax, eax
    jz .range
.nonzero:
    mov ecx, [r12+TY_KIND]
    mov r13d, [rbx+N_OP]
    cmp ecx, K_FLOAT
    je .single
    cmp ecx, K_DOUBLE
    je .double
    fld tword [rsp]
    fld tword [rsp+16]
    cmp r13d, '+'
    je .ld_add
    cmp r13d, '-'
    je .ld_sub
    cmp r13d, '*'
    je .ld_mul
    cmp r13d, '/'
    je .ld_div
    fxch st1
    fucomip st0, st1
    fstp st0
    jmp .compare
.ld_add:
    faddp st1, st0
    jmp .ld_result
.ld_sub:
    fsubp st1, st0
    jmp .ld_result
.ld_mul:
    fmulp st1, st0
    jmp .ld_result
.ld_div:
    fdivp st1, st0
.ld_result:
    fstp tword [rsp]
    mov rax, [rsp]
    movzx edx, word [rsp+8]
    jmp .finite
.single:
    movss xmm0, [rsp]
    movss xmm1, [rsp+16]
    cmp r13d, '+'
    je .f_add
    cmp r13d, '-'
    je .f_sub
    cmp r13d, '*'
    je .f_mul
    cmp r13d, '/'
    je .f_div
    ucomiss xmm0, xmm1
    jmp .compare
.f_add:
    addss xmm0, xmm1
    jmp .f_result
.f_sub:
    subss xmm0, xmm1
    jmp .f_result
.f_mul:
    mulss xmm0, xmm1
    jmp .f_result
.f_div:
    divss xmm0, xmm1
.f_result:
    movd eax, xmm0
    jmp .finite
.double:
    movsd xmm0, [rsp]
    movsd xmm1, [rsp+16]
    cmp r13d, '+'
    je .d_add
    cmp r13d, '-'
    je .d_sub
    cmp r13d, '*'
    je .d_mul
    cmp r13d, '/'
    je .d_div
    ucomisd xmm0, xmm1
    jmp .compare
.d_add:
    addsd xmm0, xmm1
    jmp .d_result
.d_sub:
    subsd xmm0, xmm1
    jmp .d_result
.d_mul:
    mulsd xmm0, xmm1
    jmp .d_result
.d_div:
    divsd xmm0, xmm1
.d_result:
    movq rax, xmm0
    jmp .finite
.finite:
    mov rdi, r12
    call constant_finite
    jmp .done
.compare:
    ; Capture all comparison flags before dispatching on the C operator.
    sete al
    setp dl
    setb cl
    seta r8b
    setae r9b
    cmp r13d, T_NE
    je .ne
    test dl, dl
    jnz .false
    cmp r13d, T_EQ
    je .bool
    cmp r13d, '<'
    je .less
    cmp r13d, T_LE
    je .less_equal
    cmp r13d, '>'
    je .greater
    cmp r13d, T_GE
    jne generation_error
    mov al, r9b
    jmp .bool
.ne:
    xor al, 1
    or al, dl
    jmp .bool
.less:
    mov al, cl
    jmp .bool
.less_equal:
    or al, cl
    jmp .bool
.greater:
    mov al, r8b
.bool:
    movzx eax, al
    jmp .done
.false:
    xor eax, eax
    jmp .done
.range:
    ; All ordinary results branch to .done; only invalid arithmetic falls here.
    lea rdi, [msg_range]
    jmp fatal
.done:
    add rsp, 32
    pop r13
    pop r12
    pop rbx
    ret
; Reject nonfinite results where a static arithmetic constant is required.
constant_finite:
    mov ecx, [rdi+TY_KIND]
    cmp ecx, K_FLOAT
    je .single
    cmp ecx, K_DOUBLE
    je .double
    cmp ecx, K_LDOUBLE
    jne .done
    mov ecx, edx
    and ecx, 0x7fff
    cmp ecx, 0x7fff
    je .range
    ret
.single:
    mov ecx, eax
    and ecx, 0x7f800000
    cmp ecx, 0x7f800000
    je .range
    ret
.double:
    mov rcx, rax
    shr rcx, 52
    and ecx, 2047
    cmp ecx, 2047
    je .range
.done:
    ret
.range:
    lea rdi, [msg_range]
    jmp fatal
