; Apply #line to subsequent token locations without changing include resolution.
pp_line_directive:
    push rbx
    push r12
    push r13
    mov rbx, rsi
    call pp_expand
    test rax, rax
    jz pp_failure
    cmp dword [rax+TK_KIND], T_NUMBER
    jne pp_failure
    mov r12, rax
    mov rdi, rax
    call parse_number
    test dword [r12+TK_FLAGS], 7168
    jnz pp_failure
    mov r13, [r12+TK_NEXT]
    mov r12, [r12+TK_VALUE]
    test r12, r12
    jz pp_failure
    cmp r12, 0x7fffffff
    ja pp_failure
    mov rax, [rbx+PP_NEXT]
    sub r12d, [rax+TK_LINE]
    xor edx, edx
    test r13, r13
    jz .locations
    cmp dword [r13+TK_KIND], T_STRING
    jne pp_failure
    cmp qword [r13+TK_NEXT], 0
    jne pp_failure
    mov edi, FILE_SIZE
    call arena_alloc
    mov rdx, rax
    mov rdi, rax
    mov rsi, [rbx+PP_FILE]
    mov ecx, FILE_SIZE/8
    rep movsq
    push rdx
    sub rsp, 8
    mov edi, [r13+TK_LEN]
    call arena_alloc
    mov rdi, rax
    mov rsi, [r13+TK_TEXT]
    inc rsi
    mov ecx, [r13+TK_LEN]
    sub ecx, 2
    rep movsb
    add rsp, 8
    pop rdx
    mov [rdx+FILE_PATH], rax
.locations:
    mov rax, [rbx+PP_NEXT]
.update:
    test rax, rax
    jz .done
    add [rax+TK_LINE], r12d
    test rdx, rdx
    jz .next
    mov [rax+TK_FILE], rdx
.next:
    mov rax, [rax+TK_NEXT]
    jmp .update
.done:
    pop r13
    pop r12
    pop rbx
    ret
