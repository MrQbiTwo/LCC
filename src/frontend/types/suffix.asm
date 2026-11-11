; Parse declarator suffixes around base type RDI.
parse_type_suffix:
    push rbx
    push r12
    push r13
    push r14
    push r15
    sub rsp, 16
    mov rbx, rdi
    mov edi, '('
    call accept
    test rax, rax
    jz .array
    mov edi, TY_SIZE
    call arena_alloc
    mov r12, rax
    mov dword [r12+TY_KIND], K_FUNC
    mov [r12+TY_BASE], rbx
    xor r13d, r13d
    mov edi, ')'
    call accept
    test rax, rax
    jz .void_check
    or dword [r12+TY_FLAGS], 2
    jmp .function_done
.void_check:
    mov r14, [parse_token]
    lea rdi, [kw_void]
    call accept_keyword
    test rax, rax
    jz .param
    mov edi, ')'
    call accept
    test rax, rax
    jnz .function_done
    mov [parse_token], r14
.param:
    call parse_declspec
    test qword [decl_storage], 2048|4096|8192|16384
    jnz .type_error
    mov rcx, [decl_storage]
    mov [rsp], rcx
    mov rdi, rax
    mov esi, 1
    call parse_declarator
    mov r14, rax
    mov r15, rdx
    cmp dword [rax+TY_KIND], K_ARRAY
    je .decay_array
    cmp dword [rax+TY_KIND], K_FUNC
    jne .parameter
    mov rdi, rax
    call type_pointer
    mov r14, rax
    jmp .parameter
.decay_array:
    mov rdi, [rax+TY_BASE]
    call type_pointer
    mov r14, rax
.parameter:
    cmp dword [r14+TY_KIND], K_VOID
    je .type_error
    mov edi, S_SIZE
    call arena_alloc
    mov [rax+S_NAME], r15
    mov [rax+S_TYPE], r14
    ; Nested parameter declarators may have changed decl_storage.
    test qword [rsp], 32768
    jz .parameter_flags
    or dword [rax+S_FLAGS], SF_REGISTER
.parameter_flags:
    test r13, r13
    jz .param_first
    mov [r13+S_PARAM_NEXT], rax
    jmp .param_linked
.param_first:
    mov [r12+TY_PARAMS], rax
.param_linked:
    mov r13, rax
    inc qword [r12+TY_COUNT]
    cmp qword [r12+TY_COUNT], 4096
    ja .type_error
    mov edi, ')'
    call accept
    test rax, rax
    jnz .function_done
    mov edi, ','
    call expect
    mov edi, T_ELLIPSIS
    call accept
    test rax, rax
    jz .param
    or dword [r12+TY_FLAGS], 1
    mov edi, ')'
    call expect
.function_done:
    mov rax, r12
    jmp .done
.array:
    mov edi, '['
    call accept
    test rax, rax
    jz .base
    xor r12d, r12d
    mov edi, ']'
    call accept
    test rax, rax
    jnz .element_type
    mov edi, 2
    call parse_expression
    mov rdi, rax
    mov rax, [rax+N_TYPE]
    cmp dword [rax+TY_KIND], K_CHAR
    jb .type_error
    cmp dword [rax+TY_KIND], K_LONG
    ja .type_error
    call evaluate_required_integer_constant
    mov r12, rax
    test r12, r12
    jz .type_error
    cmp r12, MAX_MEMORY
    ja .type_error
    mov edi, ']'
    call expect
.element_type:
    mov rdi, rbx
    call parse_type_suffix
    mov rbx, rax
    mov rdi, rbx
    call type_require_complete
    cmp dword [rbx+TY_KIND], K_VOID
    je .type_error
    cmp dword [rbx+TY_KIND], K_FUNC
    je .type_error
    mov edi, TY_SIZE
    call arena_alloc
    mov dword [rax+TY_KIND], K_ARRAY
    mov [rax+TY_BASE], rbx
    mov [rax+TY_COUNT], r12
    mov ecx, [rbx+TY_ALIGN]
    mov [rax+TY_ALIGN], ecx
    test r12, r12
    jnz .array_size
    or dword [rax+TY_FLAGS], TF_INCOMPLETE
.array_size:
    mov edx, [rbx+TY_BYTES]
    imul rdx, r12
    cmp rdx, MAX_MEMORY
    ja .type_error
    mov [rax+TY_BYTES], edx
    jmp .done
.base:
    mov rax, rbx
.done:
    add rsp, 16
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
.type_error:
    lea rdi, [msg_type]
    jmp fatal

