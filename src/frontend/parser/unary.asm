; Parse casts, prefix operators, and sizeof before postfix expressions.
parse_unary:
    push rbx
    push r12
    push r13
    inc dword [parse_depth]
    cmp dword [parse_depth], 256
    ja syntax_error
    mov r12, [parse_token]
    lea rdi, [kw_sizeof]
    call accept_keyword
    test rax, rax
    jnz .sizeof
    mov edi, '('
    call accept
    test rax, rax
    jz .operator
    call is_type_token
    test eax, eax
    jz .restore
    call parse_type_name
    mov r13, rax
    mov edi, ')'
    call expect
    call parse_unary
    mov rbx, rax
    mov rdi, rax
    call require_complete_value
    mov rdi, rbx
    mov rsi, r13
    mov rdx, r12
    call require_cast_value
    mov edi, A_CAST
    mov rsi, r12
    call node_new
    mov [rax+N_LEFT], rbx
    mov [rax+N_TYPE], r13
    jmp .done
.restore:
    mov [parse_token], r12
.operator:
    mov eax, [r12+TK_KIND]
    cmp eax, '+'
    je .prefix
    cmp eax, '-'
    je .prefix
    cmp eax, '!'
    je .prefix
    cmp eax, '~'
    je .prefix
    cmp eax, '*'
    je .prefix
    cmp eax, '&'
    je .prefix
    cmp eax, T_INC
    je .prefix
    cmp eax, T_DEC
    je .prefix
    call parse_postfix
    jmp .done
.prefix:
    mov r13d, eax
    mov edi, eax
    call accept
    call parse_unary
    mov rbx, rax
    mov rdi, rbx
    mov esi, r13d
    call require_unary_operand
    mov edi, A_UNARY
    cmp r13d, '*'
    jne .address
    mov edi, A_DEREF
.address:
    cmp r13d, '&'
    jne .prefix_node
    mov edi, A_ADDR
.prefix_node:
    mov rsi, r12
    call node_new
    mov r12, rax
    mov [rax+N_LEFT], rbx
    mov [rax+N_OP], r13d
    mov rax, [rbx+N_TYPE]
    mov [r12+N_TYPE], rax
    cmp r13d, '&'
    je .address_type
    cmp r13d, '*'
    je .deref_type
    cmp dword [rax+TY_KIND], K_STRUCT
    jae .bad_type
    cmp r13d, '!'
    je .int_type
    cmp r13d, T_INC
    je .increment_type
    cmp r13d, T_DEC
    je .increment_type
    mov rdi, rax
    call type_promote
    mov [r12+N_TYPE], rax
    jmp .prefix_done
.int_type:
    lea rax, [type_int]
    mov [r12+N_TYPE], rax
    jmp .prefix_done
.increment_type:
    mov rdi, rbx
    call require_modifiable
    mov rdi, rax
    call require_step_type
    jmp .prefix_done
.address_type:
    mov rdi, rbx
    call require_addressable
    mov rdi, [rbx+N_TYPE]
    call type_pointer
    mov [r12+N_TYPE], rax
    jmp .prefix_done
.deref_type:
    mov eax, [rax+TY_KIND]
    cmp eax, K_PTR
    je .base_type
    cmp eax, K_ARRAY
    jne .bad_type
.base_type:
    mov rax, [rbx+N_TYPE]
    mov rax, [rax+TY_BASE]
    mov [r12+N_TYPE], rax
.prefix_done:
    mov rax, r12
    jmp .done
.sizeof:
    mov r13, [parse_token]
    mov edi, '('
    call accept
    test rax, rax
    jz .sizeof_expr
    call is_type_token
    test eax, eax
    jz .sizeof_restore
    call parse_type_name
    mov rbx, rax
    mov edi, ')'
    call expect
    jmp .sizeof_value
.sizeof_restore:
    mov [parse_token], r13
.sizeof_expr:
    call parse_unary
    mov rbx, rax
    mov rdi, rax
    call require_non_bitfield
    mov rax, rbx
    mov rbx, [rax+N_TYPE]
.sizeof_value:
    mov rdi, rbx
    call type_require_complete
    cmp dword [rbx+TY_KIND], K_VOID
    je .bad_type
    cmp dword [rbx+TY_KIND], K_FUNC
    je .bad_type
    mov edi, A_NUM
    mov rsi, r12
    call node_new
    mov edx, [rbx+TY_BYTES]
    mov [rax+N_VALUE], rdx
    lea rdx, [type_ulong]
    mov [rax+N_TYPE], rdx
.done:
    dec dword [parse_depth]
    pop r13
    pop r12
    pop rbx
    ret
.bad_type:
    lea rdi, [msg_type]
    jmp fatal
