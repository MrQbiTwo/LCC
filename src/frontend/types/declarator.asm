; Parse pointers, parenthesized declarators, arrays, and function signatures.
; RDI is the base type, ESI permits unnamed declarators; RAX/type, RDX/name.
parse_declarator:
    push rbx
    push r12
    push r13
    push r14
    push r15
    mov rbx, rdi
    mov r12d, esi
.pointer:
    mov edi, '*'
    call accept
    test rax, rax
    jz .direct
    mov rdi, rbx
    call type_pointer
    mov rbx, rax
.qualifier:
    lea rdi, [kw_const]
    call accept_keyword
    test rax, rax
    jz .volatile
    or dword [rbx+TY_FLAGS], TF_CONST
    jmp .qualifier
.volatile:
    lea rdi, [kw_volatile]
    call accept_keyword
    test rax, rax
    jz .pointer
    or dword [rbx+TY_FLAGS], TF_VOLATILE
    jmp .qualifier
    jmp .pointer
.direct:
    mov edi, '('
    call accept
    test rax, rax
    jz .name
    mov r13, [parse_token]
    lea rdi, [type_int]
    mov esi, r12d
    call parse_declarator
    mov edi, ')'
    call expect
    mov rdi, rbx
    call parse_type_suffix
    mov r14, [parse_token]
    mov [parse_token], r13
    mov rdi, rax
    mov esi, r12d
    call parse_declarator
    mov [parse_token], r14
    jmp .done
.name:
    mov edi, T_IDENT
    call accept
    mov r15, rax
    test rax, rax
    jnz .suffix
    test r12d, r12d
    jz syntax_error
.suffix:
    mov rdi, rbx
    call parse_type_suffix
    mov rdx, r15
.done:
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
