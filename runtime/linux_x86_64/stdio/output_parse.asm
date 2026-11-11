; Parse one conversion at R12. Return the conversion byte or -1 and advance R12.
; Width and precision are bounded at 4096, including values supplied through '*'.
lcc_fmt_parse:
    push r13
    mov qword [rbx+PF_FLAGS], 0
    mov qword [rbx+PF_WIDTH], 0
    mov qword [rbx+PF_PREC], -1
    mov qword [rbx+PF_MOD], 0
.flags:
    mov al, [r12]
    mov edx, 1
    cmp al, '-'
    je .flag
    mov edx, 2
    cmp al, '+'
    je .flag
    mov edx, 4
    cmp al, ' '
    je .flag
    mov edx, 8
    cmp al, '0'
    jne .width
.flag:
    or [rbx+PF_FLAGS], rdx
    inc r12
    jmp .flags
.width:
    cmp byte [r12], '*'
    jne .width_number
    inc r12
    call lcc_fmt_gp
    movsxd rax, eax
    test rax, rax
    jns .width_store
    neg rax
    or qword [rbx+PF_FLAGS], 1
    jmp .width_store
.width_number:
    call lcc_fmt_number
    test rax, rax
    js .error
.width_store:
    cmp rax, 4096
    ja .error
    mov [rbx+PF_WIDTH], rax
    cmp byte [r12], '.'
    jne .modifier
    inc r12
    cmp byte [r12], '*'
    jne .precision_number
    inc r12
    call lcc_fmt_gp
    movsxd rax, eax
    test rax, rax
    js .modifier
    jmp .precision_store
.precision_number:
    call lcc_fmt_number
    test rax, rax
    js .error
.precision_store:
    cmp rax, 4096
    ja .error
    mov [rbx+PF_PREC], rax
.modifier:
    cmp byte [r12], 'h'
    jne .long
    mov qword [rbx+PF_MOD], 1
    inc r12
    jmp .conversion
.long:
    cmp byte [r12], 'l'
    jne .conversion
    mov qword [rbx+PF_MOD], 2
    inc r12
.conversion:
    movzx eax, byte [r12]
    test eax, eax
    jz .error
    inc r12
    cmp al, 'd'
    je .done
    cmp al, 'i'
    je .done
    cmp al, 'u'
    je .done
    cmp al, 'f'
    je .float
    cmp qword [rbx+PF_MOD], 0
    jne .error
    cmp al, 's'
    je .done
    cmp al, 'c'
    je .done
    cmp al, '%'
    jne .error
    cmp qword [rbx+PF_FLAGS], 0
    jne .error
    cmp qword [rbx+PF_WIDTH], 0
    jne .error
    cmp qword [rbx+PF_PREC], -1
    jne .error
    jmp .done
.float:
    cmp qword [rbx+PF_MOD], 1
    je .error
    jmp .done
.error:
    mov eax, -1
.done:
    pop r13
    ret

; Parse optional decimal digits without overflowing before the resource check.
lcc_fmt_number:
    xor eax, eax
.loop:
    movzx edx, byte [r12]
    sub edx, '0'
    cmp edx, 9
    ja .done
    imul rax, rax, 10
    add rax, rdx
    cmp rax, 4096
    ja .error
    inc r12
    jmp .loop
.error:
    mov rax, -1
.done:
    ret
