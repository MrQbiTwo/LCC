; Numeric literals use integer-only, target-format conversion.
section .text
; Classify decimal floating syntax before selecting its exact target format.
parse_number:
    test dword [rdi+TK_FLAGS], TF_NUMBER_PARSED
    jnz .cached
    or dword [rdi+TK_FLAGS], TF_NUMBER_PARSED
    mov rsi, [rdi+TK_TEXT]
    mov ecx, [rdi+TK_LEN]
    cmp ecx, 2
    jb parse_integer
    cmp byte [rsi], '0'
    jne .scan
    mov al, [rsi+1]
    or al, 32
    cmp al, 'x'
    je parse_integer
.scan:
    xor edx, edx
.next:
    mov al, [rsi+rdx]
    cmp al, '.'
    je .floating
    or al, 32
    cmp al, 'e'
    je .floating
    inc edx
    cmp edx, ecx
    jb .next
    jmp parse_integer
.cached:
    ret
.floating:
    push rbx
    push r12
    sub rsp, 8
    mov rbx, rdi
    mov edx, 1
    mov r12d, 2048
    mov al, [rsi+rcx-1]
    or al, 32
    cmp al, 'f'
    je .single
    cmp al, 'l'
    jne .convert
    mov edx, 2
    mov r12d, 4096
    dec ecx
    jmp .convert
.single:
    xor edx, edx
    mov r12d, 1024
    dec ecx
.convert:
    mov rdi, rsi
    mov esi, ecx
    call decimal_parse
    cmp ecx, 1
    je .bad
    cmp ecx, 2
    je .range
    cmp ecx, 4
    je .resource
    mov [rbx+TK_VALUE], rax
    mov [rbx+TK_HIGH], edx
    or [rbx+TK_FLAGS], r12d
    add rsp, 8
    pop r12
    pop rbx
    ret
.bad:
    lea rdi, [msg_lex]
    jmp fatal
.range:
    lea rdi, [msg_range]
    jmp fatal
.resource:
    lea rdi, [msg_memory]
    jmp fatal
; Decode an integer token with checked target-width arithmetic.
parse_integer:
    mov r8, rdi
    mov rsi, [r8+TK_TEXT]
    mov ecx, [r8+TK_LEN]
    lea r9, [rsi+rcx]
    mov r10d, 10
    xor r11d, r11d
    cmp byte [rsi], '0'
    jne .digits
    mov r10d, 8
    cmp ecx, 2
    jb .digits
    mov al, [rsi+1]
    or al, 32
    cmp al, 'x'
    jne .digits
    add rsi, 2
    mov r10d, 16
.digits:
    xor eax, eax
    xor ecx, ecx
.loop:
    cmp rsi, r9
    jae .end
    movzx edi, byte [rsi]
    sub edi, '0'
    cmp edi, 9
    jbe .digit
    movzx edi, byte [rsi]
    or edi, 32
    sub edi, 'a'-10
    cmp edi, 10
    jb .suffix
    cmp edi, 15
    ja .suffix
.digit:
    cmp edi, r10d
    jae .bad
    mul r10
    test rdx, rdx
    jnz .range
    add rax, rdi
    jc .range
    inc rsi
    inc ecx
    jmp .loop
.suffix:
    mov dl, [rsi]
    or dl, 32
    cmp dl, 'u'
    je .unsigned
    cmp dl, 'l'
    je .long
    jmp .bad
.unsigned:
    test r11d, 256
    jnz .bad
    or r11d, 256
    inc rsi
    jmp .suffix_next
.long:
    test r11d, 512
    jnz .bad
    or r11d, 512
    inc rsi
.suffix_next:
    cmp rsi, r9
    jb .suffix
.end:
    test ecx, ecx
    jz .bad
    cmp rax, 0x7fffffff
    jbe .store
    test r11d, 256
    jnz .unsigned_width
    cmp r10d, 10
    je .signed_width
    mov edx, 0xffffffff
    cmp rax, rdx
    ja .signed_width
    test r11d, 512
    jnz .signed_width
    or r11d, 256
    jmp .store
.unsigned_width:
    mov edx, 0xffffffff
    cmp rax, rdx
    jbe .store
.signed_width:
    or r11d, 512
    test rax, rax
    jns .store
    cmp r10d, 10
    je .decimal_unsigned
    or r11d, 256
    jmp .store
.decimal_unsigned:
    test r11d, 256
    jz .range
.store:
    mov [r8+TK_VALUE], rax
    or [r8+TK_FLAGS], r11d
    ret
.bad:
    lea rdi, [msg_lex]
    jmp fatal
.range:
    lea rdi, [msg_range]
    jmp fatal
