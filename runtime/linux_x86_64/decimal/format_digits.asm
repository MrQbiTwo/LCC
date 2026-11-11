; Destructively convert bigint RDI into reversed decimal bytes at RSI.
; Extract nine decimal digits per bigint division and retain one digit for zero.
; Return the byte count in RAX. Only the final group omits leading zero digits.
lcc_fmt_digits:
    mov r10, rdi
    mov r11, rsi
    xor r8d, r8d
    mov r9d, 1000000000
.digit:
    mov rcx, [r10]
    xor edx, edx
.divide:
    test rcx, rcx
    jz .store
    mov rax, [r10+rcx*8]
    div r9
    mov [r10+rcx*8], rax
    dec rcx
    jmp .divide
.store:
    mov eax, edx
    mov rcx, [r10]
.trim:
    test rcx, rcx
    jz .unpack
    cmp qword [r10+rcx*8], 0
    jne .unpack
    dec rcx
    jmp .trim
.unpack:
    mov [r10], rcx
    mov esi, 9
    mov edi, 10
.remainder:
    xor edx, edx
    div rdi
    add dl, '0'
    mov [r11+r8], dl
    inc r8
    dec esi
    jz .next
    test rcx, rcx
    jnz .remainder
    test rax, rax
    jnz .remainder
.next:
    test rcx, rcx
    jz .done
    jmp .digit
.done:
    mov rax, r8
    ret
