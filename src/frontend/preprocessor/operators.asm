; Stringify raw argument tokens with normalized whitespace and escaped spellings.
macro_stringify:
    push rbx
    push r12
    push r13
    push r14
    push r15
    sub rsp, 48
    mov rbx, rdi
    mov r12, rsi
    mov qword [rsp], 0
    mov qword [rsp+8], 0
    mov qword [rsp+16], 0
    mov byte [rsp+32], '"'
    mov rdi, rsp
    lea rsi, [rsp+32]
    mov edx, 1
    call buffer_append
    xor r15d, r15d
.token:
    test rbx, rbx
    jz .close
    test r15d, r15d
    jz .spelling
    test dword [rbx+TK_FLAGS], 3
    jz .spelling
    mov byte [rsp+32], ' '
    mov rdi, rsp
    lea rsi, [rsp+32]
    mov edx, 1
    call buffer_append
.spelling:
    mov r13, [rbx+TK_TEXT]
    mov r14d, [rbx+TK_LEN]
.character:
    test r14d, r14d
    jz .next
    mov al, [r13]
    cmp al, '"'
    je .escape
    cmp al, 92
    jne .copy
.escape:
    mov byte [rsp+32], 92
    mov rdi, rsp
    lea rsi, [rsp+32]
    mov edx, 1
    call buffer_append
.copy:
    mov rdi, rsp
    mov rsi, r13
    mov edx, 1
    call buffer_append
    inc r13
    dec r14d
    jmp .character
.next:
    mov r15d, 1
    mov rbx, [rbx+TK_NEXT]
    jmp .token
.close:
    mov byte [rsp+32], '"'
    mov rdi, rsp
    lea rsi, [rsp+32]
    mov edx, 1
    call buffer_append
    mov rdi, r12
    call token_clone
    mov dword [rax+TK_KIND], T_STRING
    mov rdx, [rsp]
    mov [rax+TK_TEXT], rdx
    mov edx, [rsp+8]
    mov [rax+TK_LEN], edx
    add rsp, 48
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
; Re-lex pasted token spellings and require exactly one resulting token.
macro_paste:
    push rbx
    push r12
    push r13
    push r14
    push r15
    mov r12, rdi
    mov r13, rsi
    mov edi, [r12+TK_LEN]
    add edi, [r13+TK_LEN]
    add rdi, 2
    call arena_alloc
    mov r14, rax
    mov rdi, rax
    mov rsi, [r12+TK_TEXT]
    mov ecx, [r12+TK_LEN]
    rep movsb
    mov rsi, [r13+TK_TEXT]
    mov ecx, [r13+TK_LEN]
    rep movsb
    sub rdi, r14
    mov r15, rdi
    mov edi, FILE_SIZE
    call arena_alloc
    mov rdx, [r12+TK_FILE]
    mov rdx, [rdx+FILE_PATH]
    mov [rax+FILE_PATH], rdx
    mov [rax+FILE_DATA], r14
    mov [rax+FILE_LEN], r15
    mov rdi, rax
    call lex_source
    mov rbx, rax
    cmp dword [rbx+TK_KIND], 0
    je pp_failure
    mov rax, [rbx+TK_NEXT]
    cmp dword [rax+TK_KIND], 0
    jne pp_failure
    mov qword [rbx+TK_NEXT], 0
    mov rax, [r12+TK_FILE]
    mov [rbx+TK_FILE], rax
    mov rax, [r12+TK_LINE]
    mov [rbx+TK_LINE], rax
    mov rdi, [r12+TK_HIDE]
    mov rsi, [r13+TK_HIDE]
    call hide_union
    mov [rbx+TK_HIDE], rax
    mov rax, rbx
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
