; Return a linked list of tokens for the file record RDI, including EOF.
lex_source:
    push rbx
    push r12
    push r13
    push r14
    push r15
    sub rsp, 32
    mov r12, rdi
    mov r13, [r12+FILE_DATA]
    mov r14, r13
    add r14, [r12+FILE_LEN]
    xor ebx, ebx
    xor r15d, r15d
    mov dword [rsp], 1
    mov dword [rsp+4], 1
    mov dword [rsp+16], 2
.next:
    cmp r13, r14
    jae .token
    movzx eax, byte [r13]
    test al, al
    jz .invalid
    cmp al, ' '
    je .space
    cmp al, 9
    je .space
    cmp al, 10
    je .space
    cmp al, 13
    je .space
    cmp al, 11
    je .space
    cmp al, 12
    je .space
    cmp al, '/'
    jne .token
    cmp byte [r13+1], '*'
    je .comment
    cmp byte [r13+1], '/'
    je .line_comment
    jmp .token
.space:
    or dword [rsp+16], 1
    lex_step
    jmp .next
.comment:
    lex_step
    lex_step
.comment_loop:
    cmp r13, r14
    jae .invalid
    cmp byte [r13], '*'
    jne .comment_advance
    cmp byte [r13+1], '/'
    je .comment_end
.comment_advance:
    lex_step
    jmp .comment_loop
.comment_end:
    lex_step
    lex_step
    or dword [rsp+16], 1
    jmp .next
.line_comment:
    ; Accept line comments as an explicitly diagnosed extension later.
    cmp r13, r14
    jae .next
    cmp byte [r13], 10
    je .next
    lex_step
    jmp .line_comment
.token:
    mov edi, TK_SIZE
    call arena_alloc
    mov [rsp+8], rax
    mov [rax+TK_TEXT], r13
    mov [rax+TK_FILE], r12
    mov edx, [rsp]
    mov [rax+TK_LINE], edx
    mov edx, [rsp+4]
    mov [rax+TK_COL], edx
    mov rdx, [r12+FILE_MAP]
    test rdx, rdx
    jz .position_ready
    mov rcx, r13
    sub rcx, [r12+FILE_DATA]
    mov rdx, [rdx+rcx*8]
    mov [rax+TK_LINE], rdx
.position_ready:
    mov edx, [rsp+16]
    mov [rax+TK_FLAGS], edx
    mov dword [rsp+16], 0
    test r15, r15
    jz .first
    mov [r15+TK_NEXT], rax
    jmp .linked
.first:
    mov rbx, rax
.linked:
    mov r15, rax
    mov [diag_token], rax
    cmp r13, r14
    jae .done
    movzx eax, byte [r13]
    cmp al, '_'
    je .ident
    mov dl, al
    or dl, 32
    cmp dl, 'a'
    jb .not_ident
    cmp dl, 'z'
    jbe .ident
.not_ident:
    cmp al, '0'
    jb .quote
    cmp al, '9'
    jbe .number
.quote:
    cmp al, '"'
    je .string
    cmp al, 39
    je .character
    cmp al, '.'
    jne .operator
    mov dl, [r13+1]
    cmp dl, '0'
    jb .operator
    cmp dl, '9'
    jbe .number
.operator:
    lea r8, [lex_operators]
.op_loop:
    mov edx, [r8]
    test edx, edx
    jz .single
    lea r9, [r8+4]
    xor ecx, ecx
.op_match:
    mov al, [r9+rcx]
    test al, al
    jz .op_found
    cmp al, [r13+rcx]
    jne .op_skip
    inc ecx
    jmp .op_match
.op_skip:
    cmp byte [r9], 0
    je .op_next
    inc r9
    jmp .op_skip
.op_next:
    lea r8, [r9+1]
    jmp .op_loop
.op_found:
    mov [r15+TK_KIND], edx
    mov [r15+TK_LEN], ecx
    add r13, rcx
    add [rsp+4], ecx
    jmp .next
.single:
    lea r8, [lex_punctuation]
    mov al, [r13]
.single_find:
    cmp byte [r8], 0
    je .invalid
    cmp al, [r8]
    je .single_ok
    inc r8
    jmp .single_find
.single_ok:
    movzx eax, al
    mov [r15+TK_KIND], eax
    mov dword [r15+TK_LEN], 1
    lex_step
    jmp .next
.ident:
    mov dword [r15+TK_KIND], T_IDENT
.ident_loop:
    lex_step
    mov al, [r13]
    cmp al, '_'
    je .ident_loop
    mov dl, al
    or dl, 32
    cmp dl, 'a'
    jb .ident_digit
    cmp dl, 'z'
    jbe .ident_loop
.ident_digit:
    cmp al, '0'
    jb .finish
    cmp al, '9'
    jbe .ident_loop
    jmp .finish
.number:
    mov dword [r15+TK_KIND], T_NUMBER
.number_loop:
    lex_step
    mov al, [r13]
    cmp al, '.'
    je .number_loop
    cmp al, '_'
    je .number_loop
    cmp al, '0'
    jb .number_sign
    cmp al, '9'
    jbe .number_loop
    mov dl, al
    or dl, 32
    cmp dl, 'a'
    jb .number_sign
    cmp dl, 'z'
    jbe .number_loop
.number_sign:
    cmp al, '+'
    je .number_exponent
    cmp al, '-'
    jne .finish_number
.number_exponent:
    mov dl, [r13-1]
    or dl, 32
    cmp dl, 'e'
    je .number_loop
    cmp dl, 'p'
    je .number_loop
.finish_number:
    mov rax, r13
    sub rax, [r15+TK_TEXT]
    mov [r15+TK_LEN], eax
    jmp .next
.string:
    mov dword [r15+TK_KIND], T_STRING
    lex_step
.string_loop:
    cmp r13, r14
    jae .invalid
    mov al, [r13]
    cmp al, 10
    je .invalid
    cmp al, '"'
    je .string_end
    cmp al, 92
    jne .string_step
    lex_step
    cmp r13, r14
    jae .invalid
.string_step:
    lex_step
    jmp .string_loop
.string_end:
    lex_step
    jmp .finish
.character:
    mov dword [r15+TK_KIND], T_NUMBER
    lex_step
    cmp byte [r13], 39
    je .invalid
    mov rdi, r13
    call decode_escape
    mov [r15+TK_VALUE], rax
    or dword [r15+TK_FLAGS], TF_NUMBER_PARSED
    mov rcx, rdx
    sub rcx, r13
    add [rsp+4], ecx
    mov r13, rdx
    cmp byte [r13], 39
    jne .invalid
    lex_step
.finish:
    mov rax, r13
    sub rax, [r15+TK_TEXT]
    mov [r15+TK_LEN], eax
    jmp .next
.done:
    mov rax, rbx
    add rsp, 32
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
.invalid:
    lea rdi, [msg_lex]
    jmp fatal
