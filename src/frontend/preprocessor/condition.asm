; Evaluate a preprocessing expression after defined substitution and expansion.
pp_evaluate:
    push rbx
    push r12
    push r13
    push r14
    push r15
    sub rsp, 16
    mov rbx, rdi
    xor r12d, r12d
    xor r13d, r13d
    test rbx, rbx
    jz pp_failure
.raw:
    test rbx, rbx
    jz .expand
    mov rdi, rbx
    lea rsi, [pp_defined]
    call token_is
    test eax, eax
    jz .copy
    mov r15, rbx
    mov rbx, [rbx+TK_NEXT]
    test rbx, rbx
    jz pp_failure
    xor r14d, r14d
    cmp dword [rbx+TK_KIND], '('
    jne .defined_name
    mov r14d, 1
    mov rbx, [rbx+TK_NEXT]
.defined_name:
    test rbx, rbx
    jz pp_failure
    cmp dword [rbx+TK_KIND], T_IDENT
    jne pp_failure
    mov rdi, rbx
    call macro_find
    test rax, rax
    setnz al
    movzx eax, al
    mov [rsp], rax
    mov rbx, [rbx+TK_NEXT]
    test r14d, r14d
    jz .defined_value
    test rbx, rbx
    jz pp_failure
    cmp dword [rbx+TK_KIND], ')'
    jne pp_failure
    mov rbx, [rbx+TK_NEXT]
.defined_value:
    mov rdi, r15
    call token_clone
    mov dword [rax+TK_KIND], T_NUMBER
    or dword [rax+TK_FLAGS], TF_NUMBER_PARSED
    mov rdx, [rsp]
    mov [rax+TK_VALUE], rdx
    jmp .link
.copy:
    mov rdi, rbx
    call token_clone
    mov rbx, [rbx+TK_NEXT]
.link:
    test r13, r13
    jz .first
    mov [r13+TK_NEXT], rax
    jmp .linked
.first:
    mov r12, rax
.linked:
    mov r13, rax
    jmp .raw
.expand:
    mov rdi, r12
    call pp_expand
    mov r12, rax
    mov r13, rdx
    test r12, r12
    jz pp_failure
    mov rbx, r12
.identifiers:
    test rbx, rbx
    jz .sentinel
    cmp dword [rbx+TK_KIND], T_IDENT
    jne .width
    mov dword [rbx+TK_KIND], T_NUMBER
    mov qword [rbx+TK_VALUE], 0
    or dword [rbx+TK_FLAGS], TF_NUMBER_PARSED
.width:
    cmp dword [rbx+TK_KIND], T_NUMBER
    jne .next_width
    mov rdi, rbx
    call parse_number
    test dword [rbx+TK_FLAGS], 7168
    jnz pp_failure
    or dword [rbx+TK_FLAGS], 512
.next_width:
    mov rbx, [rbx+TK_NEXT]
    jmp .identifiers
.sentinel:
    mov rdi, r13
    call token_clone
    mov dword [rax+TK_KIND], 0
    mov dword [rax+TK_LEN], 0
    mov [r13+TK_NEXT], rax
    mov r14, [parse_token]
    mov [parse_token], r12
    mov edi, 1
    call parse_expression
    mov rbx, rax
    mov rax, [parse_token]
    cmp dword [rax+TK_KIND], 0
    jne pp_failure
    mov [parse_token], r14
    mov rdi, rbx
    call integer_constant_form
    test eax, eax
    jz pp_failure
    mov rdi, rbx
    call evaluate_constant
    test rax, rax
    setnz al
    movzx eax, al
    add rsp, 16
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
section .rodata
pp_pragma_warning: db 'lcc: W001: ignoring unsupported pragma',10,0
section .rodata
