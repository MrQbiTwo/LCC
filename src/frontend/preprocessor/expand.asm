; Expand a null-terminated token list RDI; return head RAX and tail RDX.
pp_expand:
    push rbx
    push r12
    push r13
    push r14
    push r15
    sub rsp, 32
    mov rbx, rdi
    inc qword [pp_expansion_depth]
    cmp qword [pp_expansion_depth], 256
    ja pp_failure
    xor r12d, r12d
    xor r13d, r13d
.next:
    test rbx, rbx
    jz .done
    cmp dword [rbx+TK_KIND], 0
    je .done
    cmp dword [rbx+TK_KIND], T_IDENT
    jne .ordinary
    mov rdi, rbx
    call macro_find
    test rax, rax
    jz .ordinary
    mov r14, rax
    mov rdi, [rbx+TK_HIDE]
    mov rsi, r14
    call hide_contains
    test eax, eax
    jnz .ordinary
    cmp qword [r14+M_KIND], 0
    je .object_macro
    mov rax, [rbx+TK_NEXT]
    test rax, rax
    jz .ordinary
    cmp dword [rax+TK_KIND], '('
    jne .ordinary
    mov rdi, rbx
    mov rsi, r14
    call macro_function
    mov rbx, rax
    jmp .next
.object_macro:
    mov edi, 16
    call arena_alloc
    mov [rax], r14
    mov rdx, [rbx+TK_HIDE]
    mov [rax+8], rdx
    mov [rsp], rax
    mov qword [rsp+8], 0
    mov qword [rsp+16], 0
    mov r15, [r14+M_BODY]
.body:
    test r15, r15
    jz .replace
    mov rdi, r15
    call token_clone
    mov rdx, [rsp]
    mov [rax+TK_HIDE], rdx
    mov rdx, [rbx+TK_FILE]
    mov [rax+TK_FILE], rdx
    mov rdx, [rbx+TK_LINE]
    mov [rax+TK_LINE], rdx
    mov rdx, [rsp+16]
    test rdx, rdx
    jz .first
    mov [rdx+TK_NEXT], rax
    jmp .linked
.first:
    mov [rsp+8], rax
.linked:
    mov [rsp+16], rax
    mov r15, [r15+TK_NEXT]
    jmp .body
.replace:
    mov rdx, [rsp+16]
    test rdx, rdx
    jz .empty
    mov rax, [rbx+TK_NEXT]
    mov [rdx+TK_NEXT], rax
    mov rbx, [rsp+8]
    jmp .next
.empty:
    mov rbx, [rbx+TK_NEXT]
    jmp .next
.ordinary:
    mov rdi, rbx
    call token_clone
    test r13, r13
    jz .output_first
    mov [r13+TK_NEXT], rax
    jmp .output_linked
.output_first:
    mov r12, rax
.output_linked:
    mov r13, rax
    mov rbx, [rbx+TK_NEXT]
    jmp .next
.done:
    mov rax, r12
    mov rdx, r13
    dec qword [pp_expansion_depth]
    add rsp, 32
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
