; Resolve record designators to size_t constants without constructing a pointer.
section .rodata
offsetof_name: db '__lcc_offsetof',0
offsetof_name_length equ $-offsetof_name-1
section .text
; Return an unsigned-long literal, or zero without consuming an ordinary name.
parse_offsetof:
    xor eax, eax
    mov rdi, [parse_token]
    cmp dword [rdi+TK_LEN], offsetof_name_length
    jne .absent
    push rbx
    push r12
    push r13
    push r14
    push r15
    mov r14, [parse_token]
    mov rdi, r14
    lea rsi, [offsetof_name]
    call token_is
    test eax, eax
    jz .done
    mov edi, T_IDENT
    call expect
    mov edi, '('
    call expect
    call parse_type_name
    mov r12, rax
    xor r13d, r13d
    mov edi, ','
    call expect
.member:
    mov rax, [parse_token]
    mov [diag_token], rax
    cmp dword [r12+TY_KIND], K_STRUCT
    jb .bad_type
    cmp dword [r12+TY_KIND], K_UNION
    ja .bad_type
    mov edi, T_IDENT
    call expect
    mov rsi, rax
    mov rdi, r12
    call member_find
    test rax, rax
    jz .bad_type
    cmp dword [rax+MEM_BITS], 0
    jne .bad_type
    add r13, [rax+MEM_OFFSET]
    jc .range
    mov r12, [rax+MEM_TYPE]
.suffix:
    mov edi, '.'
    call accept
    test rax, rax
    jnz .member
    mov edi, '['
    call accept
    test rax, rax
    jz .close
    mov rbx, rax
    cmp dword [r12+TY_KIND], K_ARRAY
    jne .bad_type
    mov rdi, r12
    call type_require_complete
    mov edi, 2
    call parse_expression
    mov r15, rax
    mov rax, [rax+N_TYPE]
    cmp dword [rax+TY_KIND], K_CHAR
    jb .bad_type
    cmp dword [rax+TY_KIND], K_LONG
    ja .bad_type
    mov rdi, r15
    call evaluate_required_integer_constant
    mov [diag_token], rbx
    ; Unsigned comparison also rejects negative canonical index values.
    cmp rax, [r12+TY_COUNT]
    jae .bad_type
    mov r12, [r12+TY_BASE]
    mov ecx, [r12+TY_BYTES]
    mul rcx
    test rdx, rdx
    jnz .range
    add r13, rax
    jc .range
    mov edi, ']'
    call expect
    jmp .suffix
.close:
    mov edi, ')'
    call expect
    mov edi, A_NUM
    mov rsi, r14
    call node_new
    mov [rax+N_VALUE], r13
    lea rdx, [type_ulong]
    mov [rax+N_TYPE], rdx
.done:
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
.absent:
    ret
.bad_type:
    lea rdi, [msg_type]
    jmp fatal
.range:
    lea rdi, [msg_range]
    jmp fatal
