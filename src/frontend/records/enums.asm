; Tag namespace and enumeration declarations; aggregate records extend this module.
section .bss
align 8
tag_symbols: resq 1 ; Tag records: name, type, lexical scope, next, tag kind.
section .text
; Look up a tag token RDI, nearest lexical scope first.
tag_find:
    push rbx
    push r12
    sub rsp, 8
    mov r12, rdi
    mov rbx, [tag_symbols]
.next:
    test rbx, rbx
    jz .done
    mov rdi, [rbx]
    mov rsi, r12
    call token_same
    test eax, eax
    jnz .done
    mov rbx, [rbx+24]
    jmp .next
.done:
    mov rax, rbx
    add rsp, 8
    pop r12
    pop rbx
    ret
; Parse an enum tag/reference or a definition after its enum keyword.
parse_enum:
    push rbx
    push r12
    push r13
    push r14
    push r15
    mov edi, T_IDENT
    call accept
    mov r12, rax
    xor r13d, r13d
    test r12, r12
    jz .brace
    mov rdi, r12
    call tag_find
    mov r13, rax
.brace:
    mov edi, '{'
    call accept
    test rax, rax
    jnz .definition
    test r13, r13
    jz .bad
    cmp qword [r13+32], 13
    jne .bad
    mov rbx, [r13+8]
    jmp .done
.definition:
    test r13, r13
    jz .type
    mov eax, [scope_depth]
    cmp rax, [r13+16]
    je .duplicate
.type:
    mov edi, TY_SIZE
    call arena_alloc
    mov rbx, rax
    mov dword [rbx+TY_KIND], K_INT
    mov dword [rbx+TY_BYTES], 4
    mov dword [rbx+TY_ALIGN], 4
    mov dword [rbx+TY_FLAGS], TF_ENUM
    ; An immutable identity token survives qualifier and temporary type copies.
    mov [rbx+TY_BASE], rbx
    test r12, r12
    jz .members
    mov edi, 40
    call arena_alloc
    mov [rax], r12
    mov [rax+8], rbx
    mov edx, [scope_depth]
    mov [rax+16], rdx
    mov rdx, [tag_symbols]
    mov [rax+24], rdx
    mov qword [rax+32], 13
    mov [tag_symbols], rax
.members:
    xor r15d, r15d
.member:
    mov edi, T_IDENT
    call expect
    mov r14, rax
    mov edi, '='
    call accept
    test rax, rax
    jz .value
    mov edi, 3
    call parse_expression
    mov rdi, rax
    mov rax, [rax+N_TYPE]
    cmp dword [rax+TY_KIND], K_CHAR
    jb .bad
    cmp dword [rax+TY_KIND], K_LONG
    ja .bad
    call evaluate_required_integer_constant
    mov r15, rax
.value:
    movsxd rax, r15d
    cmp rax, r15
    jne .range
    mov rdi, r14
    mov rsi, r15
    call enum_constant_new
    inc r15
    mov edi, '}'
    call accept
    test rax, rax
    jnz .done
    mov edi, ','
    call expect
    ; Accept a trailing comma as the same harmless extension as line comments.
    mov edi, '}'
    call accept
    test rax, rax
    jz .member
.done:
    mov rax, rbx
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
.bad:
    lea rdi, [msg_type]
    jmp fatal
.duplicate:
    lea rdi, [msg_name]
    jmp fatal
.range:
    lea rdi, [msg_range]
    jmp fatal
; Publish an integer enumeration constant in the ordinary identifier namespace.
enum_constant_new:
    push rbx
    push r12
    push r13
    mov r12, rdi
    mov r13, rsi
    call symbol_find
    test rax, rax
    jz .allocate
    mov ecx, [scope_depth]
    cmp ecx, [rax+S_SCOPE]
    je .duplicate
.allocate:
    mov edi, S_SIZE
    call arena_alloc
    mov rbx, rax
    mov [rbx+S_NAME], r12
    lea rax, [type_int]
    mov [rbx+S_TYPE], rax
    mov [rbx+S_OFFSET], r13
    mov dword [rbx+S_FLAGS], SF_ENUM
    mov ecx, [scope_depth]
    mov [rbx+S_SCOPE], ecx
    test ecx, ecx
    jz .global
    or dword [rbx+S_FLAGS], SF_LOCAL
    mov rax, [local_symbols]
    mov [rbx+S_NEXT], rax
    mov [local_symbols], rbx
    jmp .done
.global:
    mov rax, [global_symbols]
    mov [rbx+S_NEXT], rax
    mov [global_symbols], rbx
.done:
    mov rax, rbx
    pop r13
    pop r12
    pop rbx
    ret
.duplicate:
    mov [diag_token], r12
    lea rdi, [msg_name]
    jmp fatal

