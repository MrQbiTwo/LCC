; Block declarations distinguish automatic homes, static storage and extern names.
section .text
; Parse a declaration at parse_token and return its automatic initializer block.
parse_local_declaration:
    push rbx
    push r12
    push r13
    push r14
    push r15
    sub rsp, 16
    mov r12, [parse_token]
    call parse_declspec
    mov r13, rax
    mov rax, [decl_storage]
    mov [rsp], rax
    mov edi, A_BLOCK
    mov rsi, r12
    call node_new
    mov rbx, rax
    xor r14d, r14d
    mov edi, ';'
    call accept
    test rax, rax
    jz .next
    test dword [r13+TY_FLAGS], TF_ENUM
    jnz .done
    cmp dword [r13+TY_KIND], K_STRUCT
    jb aggregate_error
    jmp .done
.next:
    mov rdi, r13
    xor esi, esi
    call parse_declarator
    mov rsi, rax
    mov rdi, rdx
    mov [diag_token], rdx
    test qword [rsp], 8192
    jnz .alias
    cmp dword [rax+TY_KIND], K_FUNC
    je .function
    test qword [rsp], 4096
    jnz .extern
    test qword [rsp], 2048
    jnz .static
    call local_new
    mov r15, rax
    test qword [rsp], 32768
    jz .automatic
    or dword [r15+S_FLAGS], SF_REGISTER
.automatic:
    mov edi, A_DECL
    mov rsi, r12
    call node_new
    mov [rax+N_SYM], r15
    test r14, r14
    jz .first
    mov [r14+N_NEXT], rax
    jmp .linked
.first:
    mov [rbx+N_LEFT], rax
.linked:
    mov r14, rax
    mov edi, '='
    call accept
    test rax, rax
    jz .home
    mov rdi, [r15+S_TYPE]
    call parse_initializer
    mov [r14+N_LEFT], rax
.home:
    mov rdi, r15
    call local_assign_home
    jmp .more
.function:
    test qword [rsp], 2048|16384|32768
    jnz aggregate_error
.extern:
    call local_extern_new
    mov rax, [parse_token]
    cmp dword [rax+TK_KIND], '='
    je aggregate_error
    jmp .more
.static:
    call local_static_new
    mov r15, [rax+S_ENTITY]
    mov edi, '='
    call accept
    test rax, rax
    jz .static_complete
    mov rdi, [r15+S_TYPE]
    call parse_initializer
    mov [r15+S_BODY], rax
.static_complete:
    mov rdi, [r15+S_TYPE]
    call type_require_complete
    jmp .more
.alias:
    call local_alias_new
.more:
    mov edi, ','
    call accept
    test rax, rax
    jnz .next
    mov edi, ';'
    call expect
.done:
    mov rax, rbx
    add rsp, 16
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
