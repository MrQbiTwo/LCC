; Decimal float input separates stream-prefix recognition from exact conversion.
section .rodata
lcc_rt_input_inf: db 'inf'
lcc_rt_input_inity: db 'inity'
lcc_rt_input_nan: db 'nan'

section .text

; Return 1 or a matching failure (0). Incomplete signs/exponents stay consumed.
lcc_rt_scan_float:
    push rbx
    push r12
    push r13
    mov qword [r15+LCC_IN_NEG], 0
    mov qword [r15+LCC_IN_DIGITS], 0
    mov qword [r15+LCC_IN_STATE], 0
    call lcc_rt_input_peek
    cmp eax, '-'
    jne .plus
    mov qword [r15+LCC_IN_NEG], 1
    jmp .sign
.plus:
    cmp eax, '+'
    jne .special
.sign:
    call lcc_rt_scan_take
    cmp qword [r15+LCC_IN_WIDTH], 0
    je .failure
    call lcc_rt_input_peek
.special:
    mov edx, eax
    or edx, 32
    cmp edx, 'i'
    je .infinity
    cmp edx, 'n'
    je .nan
.mantissa:
    lea edx, [rax-'0']
    cmp edx, 9
    jbe .digit
    cmp eax, '.'
    je .point
    or eax, 32
    cmp eax, 'e'
    je .exponent
    jmp .parsed
.digit:
    inc qword [r15+LCC_IN_DIGITS]
    call lcc_rt_scan_take
    jmp .next
.point:
    cmp qword [r15+LCC_IN_STATE], 0
    jne .parsed
    mov qword [r15+LCC_IN_STATE], 1
    call lcc_rt_scan_take
.next:
    cmp qword [r15+LCC_IN_WIDTH], 0
    je .parsed
    call lcc_rt_input_peek
    jmp .mantissa
.exponent:
    cmp qword [r15+LCC_IN_DIGITS], 0
    je .parsed
    call lcc_rt_scan_take
    cmp qword [r15+LCC_IN_WIDTH], 0
    je .failure
    call lcc_rt_input_peek
    cmp eax, '+'
    je .exponent_sign
    cmp eax, '-'
    jne .exponent_first
.exponent_sign:
    call lcc_rt_scan_take
    cmp qword [r15+LCC_IN_WIDTH], 0
    je .failure
    call lcc_rt_input_peek
.exponent_first:
    lea edx, [rax-'0']
    cmp edx, 9
    ja .failure
.exponent_digit:
    call lcc_rt_scan_take
    cmp qword [r15+LCC_IN_WIDTH], 0
    je .parsed
    call lcc_rt_input_peek
    lea edx, [rax-'0']
    cmp edx, 9
    jbe .exponent_digit
.parsed:
    cmp qword [r15+LCC_IN_DIGITS], 0
    je .failure
    lea rdi, [r15+LCC_IN_TOKEN]
    mov rsi, [r15+LCC_IN_USED]
    xor edx, edx
    cmp qword [r15+LCC_IN_LENGTH], 2
    sete dl
    call decimal_parse
    cmp ecx, 1
    je .failure
    cmp ecx, 4
    je .failure
    jmp .store
.infinity:
    lea rdi, [rel lcc_rt_input_inf]
    mov esi, 3
    call lcc_rt_scan_word
    test eax, eax
    jz .failure
    cmp qword [r15+LCC_IN_WIDTH], 0
    je .infinity_bits
    call lcc_rt_input_peek
    or eax, 32
    cmp eax, 'i'
    jne .infinity_bits
    lea rdi, [rel lcc_rt_input_inity]
    mov esi, 5
    call lcc_rt_scan_word
    test eax, eax
    jz .failure
.infinity_bits:
    mov rax, 0x7ff0000000000000
    mov edx, 0x7f800000
    jmp .special_bits
.nan:
    lea rdi, [rel lcc_rt_input_nan]
    mov esi, 3
    call lcc_rt_scan_word
    test eax, eax
    jz .failure
    mov rax, 0x7ff8000000000000
    mov edx, 0x7fc00000
.special_bits:
    cmp qword [r15+LCC_IN_LENGTH], 2
    je .double_sign
    mov eax, edx
    cmp qword [r15+LCC_IN_NEG], 0
    je .store
    btc eax, 31
    jmp .store
.double_sign:
    cmp qword [r15+LCC_IN_NEG], 0
    je .store
    btc rax, 63
.store:
    cmp qword [r15+LCC_IN_SUPPRESS], 0
    jne .success
    mov rdx, [r15+LCC_IN_DEST]
    cmp qword [r15+LCC_IN_LENGTH], 2
    je .store_double
    mov [rdx], eax
    jmp .success
.store_double:
    mov [rdx], rax
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

; Consume a case-insensitive spelling RDI/RSI, leaving its first mismatch unread.
lcc_rt_scan_word:
    push rbx
    push r12
    push r13
    mov r12, rdi
    mov r13, rsi
.next:
    cmp qword [r15+LCC_IN_WIDTH], 0
    je .failure
    call lcc_rt_input_peek
    mov edx, eax
    or edx, 32
    cmp dl, [r12]
    jne .failure
    call lcc_rt_scan_take
    inc r12
    dec r13
    jnz .next
    mov eax, 1
    jmp .return
.failure:
    xor eax, eax
.return:
    pop r13
    pop r12
    pop rbx
    ret
