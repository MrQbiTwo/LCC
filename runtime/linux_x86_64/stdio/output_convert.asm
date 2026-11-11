; Convert EAX's parsed specifier and emit its sign, precision, and field padding.
lcc_fmt_convert:
    push r13
    mov r13d, eax
    mov qword [rbx+PF_SIGN], 0
    mov qword [rbx+PF_ZEROS], 0
    mov qword [rbx+PF_SPECIAL], 0
    lea rax, [rbx+PF_BUFFER]
    mov [rbx+PF_TEXT], rax
    cmp r13b, 'f'
    je .float
    cmp r13b, 's'
    je .string
    cmp r13b, '%'
    je .percent
    call lcc_fmt_gp
    cmp r13b, 'c'
    je .character
    cmp r13b, 'u'
    je .unsigned
    cmp qword [rbx+PF_MOD], 2
    je .signed
    movsxd rax, eax
    cmp qword [rbx+PF_MOD], 1
    jne .signed
    movsx rax, ax
.signed:
    test rax, rax
    jns .integer_sign
    mov byte [rbx+PF_SIGN], '-'
    neg rax
.integer_sign:
    call lcc_fmt_sign
    jmp .integer
.unsigned:
    cmp qword [rbx+PF_MOD], 2
    je .integer
    mov eax, eax
    cmp qword [rbx+PF_MOD], 1
    jne .integer
    movzx eax, ax
.integer:
    lea rdi, [rbx+PF_BUFFER+64]
    xor ecx, ecx
    test rax, rax
    jnz .integer_loop
    cmp qword [rbx+PF_PREC], 0
    je .integer_done
.integer_loop:
    xor edx, edx
    mov esi, 10
    div rsi
    add dl, '0'
    dec rdi
    mov [rdi], dl
    inc rcx
    test rax, rax
    jnz .integer_loop
.integer_done:
    mov [rbx+PF_TEXT], rdi
    mov [rbx+PF_LENGTH], rcx
    mov rax, [rbx+PF_PREC]
    test rax, rax
    js .field
    and qword [rbx+PF_FLAGS], ~8
    sub rax, rcx
    jle .field
    mov [rbx+PF_ZEROS], rax
    jmp .field
.float:
    call lcc_fmt_fp
    mov rdi, rax
    test rax, rax
    jns .float_sign
    mov byte [rbx+PF_SIGN], '-'
.float_sign:
    call lcc_fmt_sign
    mov rsi, [rbx+PF_PREC]
    test rsi, rsi
    jns .float_precision
    mov esi, 6
.float_precision:
    lea rdx, [rbx+PF_BUFFER]
    call lcc_fmt_fixed
    test rax, rax
    js .error
    mov [rbx+PF_LENGTH], rax
    test edx, edx
    jz .field
    and qword [rbx+PF_FLAGS], ~8
    jmp .field
.string:
    call lcc_fmt_gp
    test rax, rax
    jz .error
    mov [rbx+PF_TEXT], rax
    xor ecx, ecx
.string_loop:
    cmp rcx, [rbx+PF_PREC]
    je .string_done
    cmp byte [rax+rcx], 0
    je .string_done
    inc rcx
    jmp .string_loop
.string_done:
    mov [rbx+PF_LENGTH], rcx
    and qword [rbx+PF_FLAGS], ~8
    jmp .field
.percent:
    mov eax, '%'
.character:
    mov [rbx+PF_BUFFER], al
    mov qword [rbx+PF_LENGTH], 1
    and qword [rbx+PF_FLAGS], ~8
.field:
    mov rax, [rbx+PF_WIDTH]
    sub rax, [rbx+PF_LENGTH]
    sub rax, [rbx+PF_ZEROS]
    cmp byte [rbx+PF_SIGN], 0
    je .padding
    dec rax
.padding:
    test rax, rax
    jns .padding_store
    xor eax, eax
.padding_store:
    mov [rbx+PF_PAD], rax
    test qword [rbx+PF_FLAGS], 1
    jnz .sign
    test qword [rbx+PF_FLAGS], 8
    jnz .zero_padding
    mov rsi, rax
    mov al, ' '
    call lcc_fmt_pad
    test eax, eax
    js .error
    jmp .sign
.zero_padding:
    add [rbx+PF_ZEROS], rax
.sign:
    cmp byte [rbx+PF_SIGN], 0
    je .zeroes
    lea rdi, [rbx+PF_SIGN]
    mov esi, 1
    call lcc_fmt_emit
    test eax, eax
    js .error
.zeroes:
    mov rsi, [rbx+PF_ZEROS]
    mov al, '0'
    call lcc_fmt_pad
    test eax, eax
    js .error
    mov rdi, [rbx+PF_TEXT]
    mov rsi, [rbx+PF_LENGTH]
    call lcc_fmt_emit
    test eax, eax
    js .error
    test qword [rbx+PF_FLAGS], 1
    jz .done
    mov rsi, [rbx+PF_PAD]
    mov al, ' '
    call lcc_fmt_pad
    jmp .done
.error:
    mov eax, -1
.done:
    pop r13
    ret

; Choose positive sign decoration without changing the integer/FP argument bits.
lcc_fmt_sign:
    cmp byte [rbx+PF_SIGN], 0
    jne .done
    test qword [rbx+PF_FLAGS], 2
    jz .space
    mov byte [rbx+PF_SIGN], '+'
    ret
.space:
    test qword [rbx+PF_FLAGS], 4
    jz .done
    mov byte [rbx+PF_SIGN], ' '
.done:
    ret
