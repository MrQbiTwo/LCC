; Parse primary values, argument lists, subscripts, and postfix increments.
parse_postfix:
    push rbx
    push r12
    push r13
    push r14
    push r15
    mov r12, [parse_token]
    cmp dword [r12+TK_KIND], T_NUMBER
    jne .consume_number
    mov rdi, r12
    call parse_number
.consume_number:
    mov edi, T_NUMBER
    call accept
    test rax, rax
    jz .paren
    mov edi, A_NUM
    mov rsi, r12
    call node_new
    mov rbx, rax
    mov rax, [r12+TK_VALUE]
    mov [rbx+N_VALUE], rax
    mov eax, [r12+TK_HIGH]
    mov [rbx+N_AUX], rax
    lea rax, [type_float]
    test dword [r12+TK_FLAGS], 1024
    jnz .literal_type
    lea rax, [type_double]
    test dword [r12+TK_FLAGS], 2048
    jnz .literal_type
    lea rax, [type_ldouble]
    test dword [r12+TK_FLAGS], 4096
    jnz .literal_type
    lea rax, [type_int]
    lea rdx, [type_uint]
    test dword [r12+TK_FLAGS], 512
    jz .literal_sign
    lea rax, [type_long]
    lea rdx, [type_ulong]
.literal_sign:
    test dword [r12+TK_FLAGS], 256
    cmovnz rax, rdx
.literal_type:
    mov [rbx+N_TYPE], rax
    jmp .post
.paren:
    mov edi, '('
    call accept
    test rax, rax
    jz .identifier
    mov edi, 1
    call parse_expression
    mov rbx, rax
    mov edi, ')'
    call expect
    jmp .post
.identifier:
    mov rax, [parse_token]
    cmp dword [rax+TK_KIND], T_STRING
    jne .identifier_name
    call parse_string
    mov rbx, rax
    jmp .post
.identifier_name:
    call parse_offsetof
    test rax, rax
    jnz .builtin
    call parse_vararg
    test rax, rax
    jz .ordinary_identifier
.builtin:
    mov rbx, rax
    jmp .post
.ordinary_identifier:
    mov edi, T_IDENT
    call expect
    mov rdi, rax
    call symbol_find
    test rax, rax
    jz .unknown
    test dword [rax+S_FLAGS], SF_TYPEDEF
    jnz .bad_type
    mov r13, rax
    test dword [r13+S_FLAGS], SF_ENUM
    jnz .enum_value
    mov rdi, r13
    call symbol_entity
    or dword [rax+S_FLAGS], SF_REFERENCED
    mov edi, A_VAR
    mov rsi, r12
    call node_new
    mov rbx, rax
    mov [rbx+N_SYM], r13
    mov rax, [r13+S_TYPE]
    mov [rbx+N_TYPE], rax
    jmp .post
.enum_value:
    mov edi, A_NUM
    mov rsi, r12
    call node_new
    mov rbx, rax
    mov rax, [r13+S_OFFSET]
    mov [rbx+N_VALUE], rax
.post:
    mov r12, [parse_token]
    mov edi, '('
    call accept
    test rax, rax
    jnz .call
    mov edi, '['
    call accept
    test rax, rax
    jnz .index
    mov edi, '.'
    call accept
    test rax, rax
    jnz .member
    mov edi, T_ARROW
    call accept
    test rax, rax
    jnz .member
    mov edi, T_INC
    call accept
    test rax, rax
    jnz .post_inc
    mov edi, T_DEC
    call accept
    test rax, rax
    jnz .post_inc
    mov rax, rbx
    jmp .done
.post_inc:
    mov rdi, rbx
    call require_modifiable
    mov rdi, rax
    call require_step_type
    mov edi, A_POST
    mov rsi, r12
    call node_new
    mov [rax+N_LEFT], rbx
    mov edx, [r12+TK_KIND]
    mov [rax+N_OP], edx
    mov rdx, [rbx+N_TYPE]
    mov [rax+N_TYPE], rdx
    mov rbx, rax
    jmp .post
.member:
    mov r14, [rbx+N_TYPE]
    cmp dword [r12+TK_KIND], T_ARROW
    jne .member_type
    cmp dword [r14+TY_KIND], K_PTR
    je .member_pointer
    cmp dword [r14+TY_KIND], K_ARRAY
    jne .bad_type
.member_pointer:
    mov r14, [r14+TY_BASE]
.member_type:
    cmp dword [r14+TY_KIND], K_STRUCT
    jb .bad_type
    mov edi, T_IDENT
    call expect
    mov rsi, rax
    mov rdi, r14
    call member_find
    test rax, rax
    jz .bad_type
    mov r13, rax
    mov edi, A_MEMBER
    mov rsi, r12
    call node_new
    mov [rax+N_LEFT], rbx
    mov rbx, rax
    mov eax, [r12+TK_KIND]
    mov [rbx+N_OP], eax
    mov rax, [r13+MEM_OFFSET]
    mov [rbx+N_VALUE], rax
    mov [rbx+N_SYM], r13
    mov rax, [r13+MEM_TYPE]
    mov esi, [r14+TY_FLAGS]
    and esi, TF_CONST|TF_VOLATILE
    jz .member_ready
    mov rdi, rax
    call type_qualify
.member_ready:
    mov [rbx+N_TYPE], rax
    jmp .post
.call:
    mov r13, [rbx+N_TYPE]
    cmp dword [r13+TY_KIND], K_PTR
    jne .call_type
    mov r13, [r13+TY_BASE]
.call_type:
    cmp dword [r13+TY_KIND], K_FUNC
    jne .bad_type
    mov rax, [r13+TY_BASE]
    mov rdi, rax
    call type_require_complete
    mov edi, A_CALL
    mov rsi, r12
    call node_new
    mov r14, rax
    mov [rax+N_LEFT], rbx
    mov rax, [r13+TY_BASE]
    mov [r14+N_TYPE], rax
    mov [r14+N_AUX], r13
    cmp dword [rax+TY_KIND], K_STRUCT
    jb .call_arguments
    mov rdi, rax
    call local_temporary
    mov [r14+N_TEMP], rax
.call_arguments:
    xor r15d, r15d
    mov edi, ')'
    call accept
    test rax, rax
    jnz .called
.argument:
    mov edi, 2
    call parse_value_expression
    test r15, r15
    jz .first_arg
    mov [r15+N_NEXT], rax
    jmp .arg_linked
.first_arg:
    mov [r14+N_ARGS], rax
.arg_linked:
    mov r15, rax
    inc qword [r14+N_VALUE]
    cmp qword [r14+N_VALUE], 4096
    ja .bad_type
    mov edi, ')'
    call accept
    test rax, rax
    jnz .called
    mov edi, ','
    call expect
    jmp .argument
.called:
    test dword [r13+TY_FLAGS], 2
    jnz .call_count_ok
    mov rax, [r14+N_VALUE]
    cmp rax, [r13+TY_COUNT]
    jb .bad_type
    je .call_count_ok
    test dword [r13+TY_FLAGS], 1
    jz .bad_type
.call_count_ok:
    mov rdi, r14
    call require_call_conversions
    mov rbx, r14
    jmp .post
.index:
    mov edi, 1
    call parse_value_expression
    mov r13, rax
    mov edi, ']'
    call expect
    ; Normalize i[p] to p[i] before constructing the scaled addition.
    mov rax, [rbx+N_TYPE]
    cmp dword [rax+TY_KIND], K_PTR
    je .index_integer
    cmp dword [rax+TY_KIND], K_ARRAY
    je .index_integer
    mov rax, [r13+N_TYPE]
    cmp dword [rax+TY_KIND], K_PTR
    je .index_swap
    cmp dword [rax+TY_KIND], K_ARRAY
    jne .bad_type
.index_swap:
    xchg rbx, r13
.index_integer:
    mov rdi, r13
    call require_integer_value
    mov edi, A_BINARY
    mov rsi, r12
    call node_new
    mov [rax+N_LEFT], rbx
    mov [rax+N_RIGHT], r13
    mov dword [rax+N_OP], '+'
    mov rdx, [rbx+N_TYPE]
    mov [rax+N_TYPE], rdx
    lea rdx, [type_ulong]
    mov [rax+N_AUX], rdx
    mov r13, rax
    mov edi, A_DEREF
    mov rsi, r12
    call node_new
    mov [rax+N_LEFT], r13
    mov rdx, [rbx+N_TYPE]
    cmp dword [rdx+TY_KIND], K_PTR
    je .indexed
    cmp dword [rdx+TY_KIND], K_ARRAY
    jne .bad_type
.indexed:
    push rax
    sub rsp, 8
    mov rdi, [rdx+TY_BASE]
    cmp dword [rdi+TY_KIND], K_VOID
    je .bad_type
    cmp dword [rdi+TY_KIND], K_FUNC
    je .bad_type
    call type_require_complete
    mov rdx, [rbx+N_TYPE]
    add rsp, 8
    pop rax
    mov rdx, [rdx+TY_BASE]
    mov [rax+N_TYPE], rdx
    mov rbx, rax
    jmp .post
.done:
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
.unknown:
    lea rdi, [msg_name]
    jmp fatal
.bad_type:
    lea rdi, [msg_type]
    jmp fatal
