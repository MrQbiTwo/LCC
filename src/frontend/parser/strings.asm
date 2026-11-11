; Concatenate and decode adjacent string literal tokens into one array value.
parse_string:
    push rbx
    push r12
    push r13
    push r14
    push r15
    sub rsp, 48
    mov r12, [parse_token]
    mov qword [rsp], 0
    mov qword [rsp+8], 0
    mov qword [rsp+16], 0
.next:
    mov edi, T_STRING
    call accept
    test rax, rax
    jz .finish
    mov rbx, [rax+TK_TEXT]
    mov r13d, [rax+TK_LEN]
    lea r13, [rbx+r13-1]
    inc rbx
.characters:
    cmp rbx, r13
    jae .next
    mov rdi, rbx
    call decode_escape
    mov rbx, rdx
    cmp rbx, r13
    ja syntax_error
    mov [rsp+32], al
    mov rdi, rsp
    lea rsi, [rsp+32]
    mov edx, 1
    call buffer_append
    jmp .characters
.finish:
    mov byte [rsp+32], 0
    mov rdi, rsp
    lea rsi, [rsp+32]
    mov edx, 1
    call buffer_append
    mov edi, A_STRING
    mov rsi, r12
    call node_new
    mov r14, rax
    mov rdx, [rsp]
    mov [rax+N_VALUE], rdx
    mov rdx, [rsp+8]
    mov [rax+N_AUX], rdx
    mov edi, TY_SIZE
    call arena_alloc
    mov dword [rax+TY_KIND], K_ARRAY
    mov edx, [rsp+8]
    mov [rax+TY_BYTES], edx
    mov [rax+TY_COUNT], rdx
    mov dword [rax+TY_ALIGN], 1
    lea rdx, [type_char]
    mov [rax+TY_BASE], rdx
    mov [r14+N_TYPE], rax
    mov rax, r14
    add rsp, 48
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret

