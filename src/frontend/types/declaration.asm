; Compose linked declarations without confusing lexical and storage ownership.
section .text
; Merge type RDI with declaration RSI; arrays may complete the owned outer type.
declaration_type_merge:
    cmp dword [rdi+TY_KIND], K_ARRAY
    je type_array_redeclare
    cmp dword [rsi+TY_KIND], K_ARRAY
    je aggregate_error
    push rbx
    push r12
    push r13
    mov rbx, rdi
    mov r12, rsi
    cmp dword [rdi+TY_KIND], K_FUNC
    jne .exact
    cmp dword [rsi+TY_KIND], K_FUNC
    jne aggregate_error
    mov eax, [rdi+TY_FLAGS]
    xor eax, [rsi+TY_FLAGS]
    test eax, 2
    jz .prototype
    ; An unspecified parameter list composes only with default-promoted types.
    mov rdi, [rbx+TY_BASE]
    mov rsi, [r12+TY_BASE]
    call type_array_element_match
    test dword [rbx+TY_FLAGS], 2
    cmovnz rbx, r12
    test dword [rbx+TY_FLAGS], 1
    jnz aggregate_error
    mov r12, [rbx+TY_PARAMS]
.parameter:
    test r12, r12
    jz .done
    mov rdi, [r12+S_TYPE]
    cmp dword [rdi+TY_KIND], K_FLOAT
    je aggregate_error
    call type_promote
    mov rsi, rax
    mov rdi, [r12+S_TYPE]
    call conditional_pointee_match
    mov r12, [r12+S_PARAM_NEXT]
    jmp .parameter
.prototype:
    test eax, 1
    jnz aggregate_error
    mov rdi, [rbx+TY_BASE]
    mov rsi, [r12+TY_BASE]
    call type_array_element_match
    mov r13, [rbx+TY_PARAMS]
    mov r12, [r12+TY_PARAMS]
.prototype_parameter:
    test r13, r13
    jz .prototype_end
    test r12, r12
    jz aggregate_error
    mov rdi, [r13+S_TYPE]
    mov rsi, [r12+S_TYPE]
    call conditional_pointee_match
    mov r13, [r13+S_PARAM_NEXT]
    mov r12, [r12+S_PARAM_NEXT]
    jmp .prototype_parameter
.prototype_end:
    test r12, r12
    jnz aggregate_error
    jmp .done
.exact:
    call type_array_element_match
.done:
    mov rax, rbx
    pop r13
    pop r12
    pop rbx
    ret
; Validate a declaration RSI against entity RDI's independent linkage type.
entity_type_merge:
    push rbx
    push r12
    sub rsp, 8
    mov rbx, rdi
    mov r12, rsi
    ; A defined empty-list function has zero parameters in every later scope.
    test dword [rbx+S_FLAGS], SF_DEFINED
    jz .compatibility
    mov rax, [rbx+S_TYPE]
    cmp dword [rax+TY_KIND], K_FUNC
    jne .compatibility
    test dword [rax+TY_FLAGS], 2
    jz .compatibility
    cmp dword [r12+TY_KIND], K_FUNC
    jne .compatibility
    cmp qword [r12+TY_COUNT], 0
    jne aggregate_error
.compatibility:
    mov rdi, [rbx+S_COMPAT]
    test rdi, rdi
    jnz .merge
    mov rdi, [rbx+S_TYPE]
    call type_array_object
    mov rdi, rax
.merge:
    ; An initializer may have completed the entity type since its last use.
    mov rsi, [rbx+S_TYPE]
    call declaration_type_merge
    mov rdi, rax
    mov rsi, r12
    call declaration_type_merge
    mov [rbx+S_COMPAT], rax
    add rsp, 8
    pop r12
    pop rbx
    ret
