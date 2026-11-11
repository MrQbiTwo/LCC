; Integer input accepts a sign and the maximal valid d/i/u numeric prefix.
section .text

; Return 1 for a conversion or 0 for a matching failure. R15 owns scanf state.
lcc_rt_scan_integer:
    push rbx
    push r12
    push r13
    xor ebx, ebx
    mov qword [r15+LCC_IN_NEG], 0
    mov qword [r15+LCC_IN_BASE], 10
    mov qword [r15+LCC_IN_DIGITS], 0
    call lcc_rt_input_peek
    cmp eax, '-'
    jne .plus
    mov qword [r15+LCC_IN_NEG], 1
    jmp .sign
.plus:
    cmp eax, '+'
    jne .radix
.sign:
    call lcc_rt_scan_take
    cmp qword [r15+LCC_IN_WIDTH], 0
    je .failure
    call lcc_rt_input_peek
.radix:
    cmp qword [r15+LCC_IN_SPEC], 'i'
    jne .digit
    cmp eax, '0'
    jne .digit
    mov qword [r15+LCC_IN_BASE], 8
    mov qword [r15+LCC_IN_DIGITS], 1
    call lcc_rt_scan_take
    cmp qword [r15+LCC_IN_WIDTH], 0
    je .parsed
    call lcc_rt_input_peek
    mov edx, eax
    or edx, 32
    cmp edx, 'x'
    jne .digit
    mov qword [r15+LCC_IN_BASE], 16
    mov qword [r15+LCC_IN_DIGITS], 0
    call lcc_rt_scan_take
    jmp .next
.digit:
    mov edx, eax
    sub edx, '0'
    cmp edx, 9
    jbe .range
    mov edx, eax
    or edx, 32
    sub edx, 'a'
    cmp edx, 5
    ja .parsed
    add edx, 10
.range:
    cmp rdx, [r15+LCC_IN_BASE]
    jae .parsed
    imul rbx, [r15+LCC_IN_BASE]
    add rbx, rdx
    inc qword [r15+LCC_IN_DIGITS]
    call lcc_rt_scan_take
.next:
    cmp qword [r15+LCC_IN_WIDTH], 0
    je .parsed
    call lcc_rt_input_peek
    jmp .digit
.parsed:
    cmp qword [r15+LCC_IN_DIGITS], 0
    je .failure
    cmp qword [r15+LCC_IN_SUPPRESS], 0
    jne .success
    cmp qword [r15+LCC_IN_NEG], 0
    je .store
    neg rbx
.store:
    mov rdx, [r15+LCC_IN_DEST]
    cmp qword [r15+LCC_IN_LENGTH], 2
    je .long
    cmp qword [r15+LCC_IN_LENGTH], 1
    je .short
    mov [rdx], ebx
    jmp .success
.short:
    mov [rdx], bx
    jmp .success
.long:
    mov [rdx], rbx
.success:
    mov eax, 1
    jmp .return
.failure:
    xor eax, eax
.return:
    pop r13
    pop r12
    pop rbx
    ret
