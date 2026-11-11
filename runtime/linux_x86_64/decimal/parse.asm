; Parse decimal bytes RDI/RSI into binary32 (EDX=0), binary64 (1), or binary80 (2).
; Return bits in RAX/RDX and ECX status: 0 success, 1 syntax, 2 overflow,
; 3 rounded to zero, 4 resource limit. Binary80 uses the low 16 bits of RDX.
decimal_parse:
    push rbx
    push rbp
    push r12
    push r13
    push r14
    push r15
    sub rsp, DEC_FRAME
    mov r12, rdi
    lea r13, [rdi+rsi]
    mov r14d, edx
    mov rbp, rsp
    mov rdi, rbp
    xor eax, eax
    mov ecx, DEC_FRAME/8
    rep stosq
    mov [rbp+DC_FORMAT], r14
    cmp r14d, 2
    ja .invalid
    cmp r12, r13
    jae .invalid
    cmp byte [r12], '-'
    jne .plus
    mov qword [rbp+DC_SIGN], 1
    inc r12
    jmp .sign_done
.plus:
    cmp byte [r12], '+'
    jne .sign_done
    inc r12
.sign_done:
    cmp r12, r13
    jae .invalid
    imul rax, r14, 48
    lea rsi, [dec_formats]
    add rsi, rax
    mov rax, [rsi]
    mov [rbp+DC_P], rax
    mov rax, [rsi+8]
    mov [rbp+DC_BIAS], rax
    mov rax, [rsi+16]
    mov [rbp+DC_MIN], rax
    mov rax, [rsi+24]
    mov [rbp+DC_MAX], rax
    xor r15d, r15d
.digits:
    cmp r12, r13
    jae .parsed
    movzx eax, byte [r12]
    cmp al, '.'
    je .dot
    or al, 32
    cmp al, 'e'
    je .exponent
    movzx esi, byte [r12]
    sub esi, '0'
    cmp esi, 9
    ja .invalid
    inc r15
    cmp qword [rbp+DC_DOT], 0
    je .fraction_done
    inc qword [rbp+DC_FRAC]
.fraction_done:
    cmp qword [rbp+DC_DIGITS], 0
    jne .significant
    test esi, esi
    jz .next_digit
.significant:
    inc qword [rbp+DC_DIGITS]
    cmp qword [rbp+DC_DIGITS], 8192
    ja .capacity
    mov rbx, rsi
    mov rdi, rbp
    mov esi, 10
    call dec_multiply
    test eax, eax
    jnz .capacity
    mov rdi, rbp
    mov rsi, rbx
    call dec_add_small
    test eax, eax
    jnz .capacity
.next_digit:
    inc r12
    jmp .digits
.dot:
    cmp qword [rbp+DC_DOT], 0
    jne .invalid
    mov qword [rbp+DC_DOT], 1
    inc r12
    jmp .digits
.exponent:
    inc r12
    cmp r12, r13
    jae .invalid
    cmp byte [r12], '-'
    jne .exponent_plus
    mov qword [rbp+DC_EXP_NEG], 1
    inc r12
    jmp .exponent_digits
.exponent_plus:
    cmp byte [r12], '+'
    jne .exponent_digits
    inc r12
.exponent_digits:
    cmp r12, r13
    jae .invalid
.exponent_loop:
    cmp r12, r13
    jae .parsed
    movzx eax, byte [r12]
    sub eax, '0'
    cmp eax, 9
    ja .invalid
    mov rdx, [rbp+DC_EXP]
    cmp rdx, 1000000
    jae .exponent_next
    imul rdx, 10
    add rdx, rax
    mov [rbp+DC_EXP], rdx
.exponent_next:
    inc r12
    jmp .exponent_loop
.parsed:
    test r15, r15
    jz .invalid
    cmp qword [rbp], 0
    je .zero
    mov rax, [rbp+DC_EXP]
    cmp qword [rbp+DC_EXP_NEG], 0
    je .scale
    neg rax
.scale:
    sub rax, [rbp+DC_FRAC]
    mov [rbp+DC_SCALE], rax
    add rax, [rbp+DC_DIGITS]
    dec rax
    imul rcx, [rbp+DC_FORMAT], 48
    lea rdx, [dec_formats]
    add rdx, rcx
    cmp rax, [rdx+32]
    jg .infinity
    cmp rax, [rdx+40]
    jl .underflow
    mov qword [rbp+DEC_BIG], 1
    mov qword [rbp+DEC_BIG+8], 1
    mov r12, [rbp+DC_SCALE]
    mov rbx, rbp
    test r12, r12
    jns .powers
    neg r12
    lea rbx, [rbp+DEC_BIG]
.powers:
    test r12, r12
    jz .binary_exponent
    mov rdi, rbx
    mov esi, 5
    call dec_multiply
    test eax, eax
    jnz .capacity
    dec r12
    jmp .powers
.binary_exponent:
    mov rdi, rbp
    call dec_bits
    mov r12, rax
    lea rdi, [rbp+DEC_BIG]
    call dec_bits
    sub r12, rax
    mov [rbp+DC_K], r12
    lea rdi, [rbp+DEC_BIG*2]
    test r12, r12
    js .negative_k
    lea rsi, [rbp+DEC_BIG]
    call dec_copy
    lea rdi, [rbp+DEC_BIG*2]
    mov rsi, r12
    call dec_shift_left
    test eax, eax
    jnz .capacity
    mov rdi, rbp
    lea rsi, [rbp+DEC_BIG*2]
    call dec_compare
    jmp .compare_k
.negative_k:
    mov rsi, rbp
    call dec_copy
    lea rdi, [rbp+DEC_BIG*2]
    mov rsi, r12
    neg rsi
    call dec_shift_left
    test eax, eax
    jnz .capacity
    lea rdi, [rbp+DEC_BIG*2]
    lea rsi, [rbp+DEC_BIG]
    call dec_compare
.compare_k:
    test eax, eax
    jns .k_ready
    dec qword [rbp+DC_K]
.k_ready:
    mov rax, [rbp+DC_K]
    add rax, [rbp+DC_SCALE]
    cmp rax, [rbp+DC_MAX]
    jg .infinity
    cmp rax, [rbp+DC_MIN]
    jl .subnormal_scale
    mov [rbp+DC_E], rax
    mov r12, [rbp+DC_P]
    dec r12
    sub r12, [rbp+DC_K]
    jmp .quotient_scale
.subnormal_scale:
    mov rax, [rbp+DC_MIN]
    mov [rbp+DC_E], rax
    mov r12, [rbp+DC_P]
    dec r12
    sub r12, rax
    add r12, [rbp+DC_SCALE]
.quotient_scale:
    mov rdi, rbp
    test r12, r12
    jns .scale_integer
    lea rdi, [rbp+DEC_BIG]
    neg r12
.scale_integer:
    mov rsi, r12
    call dec_shift_left
    test eax, eax
    jnz .capacity
    mov rdi, rbp
    call dec_bits
    mov r12, rax
    lea rdi, [rbp+DEC_BIG]
    call dec_bits
    sub r12, rax
    xor r14d, r14d
    test r12, r12
    js .round
    cmp r12, 64
    ja .capacity
    lea rdi, [rbp+DEC_BIG*2]
    lea rsi, [rbp+DEC_BIG]
    call dec_copy
    lea rdi, [rbp+DEC_BIG*2]
    mov rsi, r12
    call dec_shift_left
    test eax, eax
    jnz .capacity
.divide:
    mov rdi, rbp
    lea rsi, [rbp+DEC_BIG*2]
    call dec_compare
    test eax, eax
    js .divide_next
    cmp r12, 64
    je .capacity
    mov rdi, rbp
    lea rsi, [rbp+DEC_BIG*2]
    call dec_subtract
    bts r14, r12
.divide_next:
    lea rdi, [rbp+DEC_BIG*2]
    call dec_shift_right
    dec r12
    jns .divide
.round:
    mov rdi, rbp
    mov esi, 2
    call dec_multiply
    test eax, eax
    jnz .capacity
    mov rdi, rbp
    lea rsi, [rbp+DEC_BIG]
    call dec_compare
    test eax, eax
    js .rounded
    jnz .round_up
    test r14b, 1
    jz .rounded
.round_up:
    add r14, 1
    jnc .rounded
    mov r14, 0x8000000000000000
    inc qword [rbp+DC_E]
.rounded:
    mov rcx, [rbp+DC_P]
    cmp ecx, 64
    je .exponent_check
    bt r14, rcx
    jnc .exponent_check
    shr r14, 1
    inc qword [rbp+DC_E]
.exponent_check:
    mov rax, [rbp+DC_E]
    cmp rax, [rbp+DC_MAX]
    jg .infinity
    test r14, r14
    jz .underflow
    mov ecx, [rbp+DC_P]
    dec ecx
    xor edx, edx
    bt r14, rcx
    jnc .pack
    mov rdx, [rbp+DC_E]
    add rdx, [rbp+DC_BIAS]
.pack:
    cmp qword [rbp+DC_FORMAT], 2
    je .pack_extended
    btr r14, rcx
    shl rdx, cl
    mov rax, r14
    or rax, rdx
    mov rcx, [rbp+DC_SIGN]
    cmp qword [rbp+DC_FORMAT], 0
    je .sign_single
    shl rcx, 63
    jmp .sign_scalar
.sign_single:
    shl rcx, 31
.sign_scalar:
    or rax, rcx
    xor edx, edx
    jmp .return
.pack_extended:
    mov rax, r14
    mov rcx, [rbp+DC_SIGN]
    shl rcx, 15
    or rdx, rcx
    jmp .return
.infinity:
    mov qword [rbp+DC_STATUS], 2
    cmp qword [rbp+DC_FORMAT], 2
    je .extended_inf
    mov eax, 0x7f800000
    cmp qword [rbp+DC_FORMAT], 0
    je .scalar_inf
    mov rax, 0x7ff0000000000000
.scalar_inf:
    mov rcx, [rbp+DC_SIGN]
    cmp qword [rbp+DC_FORMAT], 0
    je .sign_single
    shl rcx, 63
    jmp .sign_scalar
.extended_inf:
    mov r14, 0x8000000000000000
    mov edx, 0x7fff
    jmp .pack_extended
.underflow:
    mov qword [rbp+DC_STATUS], 3
.zero:
    xor eax, eax
    xor edx, edx
    cmp qword [rbp+DC_FORMAT], 2
    je .extended_zero
    mov rcx, [rbp+DC_SIGN]
    cmp qword [rbp+DC_FORMAT], 0
    je .sign_single
    shl rcx, 63
    jmp .sign_scalar
.extended_zero:
    xor r14d, r14d
    jmp .pack_extended
.invalid:
    mov qword [rbp+DC_STATUS], 1
    xor eax, eax
    xor edx, edx
    jmp .return
.capacity:
    mov qword [rbp+DC_STATUS], 4
    xor eax, eax
    xor edx, edx
.return:
    mov ecx, [rbp+DC_STATUS]
    add rsp, DEC_FRAME
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbp
    pop rbx
    ret
