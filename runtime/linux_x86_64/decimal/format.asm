; Exact binary64 fixed notation. RDI bits, RSI precision, RDX output (4410 bytes).
; RAX length or -1; EDX identifies inf/nan. Sign is handled by the caller.
%define FMT_REVERSE DEC_BIG
%define FMT_FRAME (DEC_BIG+4424)
section .text
lcc_fmt_fixed:
    push rbp
    mov rbp, rsp
    push rbx
    push r12
    push r13
    push r14
    push r15
    sub rsp, FMT_FRAME
    mov rbx, rsp
    mov r12, rsi
    mov r13, rdx
    cmp r12, 4096
    ja .error
    mov rax, rdi
    shr rax, 52
    and eax, 2047
    mov r14, 0x000fffffffffffff
    and r14, rdi
    cmp eax, 2047
    je .special
    test eax, eax
    jz .subnormal
    bts r14, 52
    sub eax, 1075
    jmp .exponent
.subnormal:
    mov eax, -1074
.exponent:
    movsxd r15, eax
    add r15, r12
    mov qword [rbx], 1
    mov [rbx+8], r14
    test r14, r14
    jz .digits
    mov r14, r12
.multiply:
    ; Batch powers of five to reduce full bigint passes at large precision.
    cmp r14, 27
    jb .multiply_tail
    mov rdi, rbx
    mov rsi, 7450580596923828125
    call dec_multiply
    test eax, eax
    jnz .error
    sub r14, 27
    jmp .multiply
.multiply_tail:
    test r14, r14
    jz .scale
    mov rdi, rbx
    mov esi, 5
    call dec_multiply
    test eax, eax
    jnz .error
    dec r14
    jmp .multiply_tail
.scale:
    test r15, r15
    js .fraction
    mov rdi, rbx
    mov rsi, r15
    call dec_shift_left
    test eax, eax
    jnz .error
    jmp .digits
.fraction:
    neg r15
    xor r14d, r14d
    dec r15
.sticky:
    test r15, r15
    jz .round_bit
    mov eax, [rbx+8]
    and eax, 1
    or r14d, eax
    mov rdi, rbx
    call dec_shift_right
    dec r15
    jmp .sticky
.round_bit:
    mov r15, [rbx+8]
    and r15d, 1
    mov rdi, rbx
    call dec_shift_right
    test r15d, r15d
    jz .digits
    mov eax, [rbx+8]
    and eax, 1
    or eax, r14d
    jz .digits
    mov rdi, rbx
    mov esi, 1
    call dec_add_small
    test eax, eax
    jnz .error
.digits:
    mov rdi, rbx
    lea rsi, [rbx+FMT_REVERSE]
    call lcc_fmt_digits
    mov r14, rax
    mov r15, r13
    cmp r14, r12
    ja .integer
    mov byte [r15], '0'
    inc r15
    test r12, r12
    jz .done
    mov byte [r15], '.'
    inc r15
    mov rcx, r12
    sub rcx, r14
.leading:
    test rcx, rcx
    jz .copy
    mov byte [r15], '0'
    inc r15
    dec rcx
    jmp .leading
.integer:
    mov rcx, r14
    sub rcx, r12
.integer_loop:
    dec r14
    mov al, [rbx+FMT_REVERSE+r14]
    mov [r15], al
    inc r15
    dec rcx
    jnz .integer_loop
    test r12, r12
    jz .done
    mov byte [r15], '.'
    inc r15
.copy:
    test r14, r14
    jz .done
    dec r14
    mov al, [rbx+FMT_REVERSE+r14]
    mov [r15], al
    inc r15
    jmp .copy
.done:
    mov rax, r15
    sub rax, r13
    xor edx, edx
    jmp .return
.special:
    mov word [r13], 'in'
    mov byte [r13+2], 'f'
    test r14, r14
    jz .special_done
    mov word [r13], 'na'
    mov byte [r13+2], 'n'
.special_done:
    mov eax, 3
    mov edx, 1
    jmp .return
.error:
    mov rax, -1
.return:
    add rsp, FMT_FRAME
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    pop rbp
    ret

%include "runtime/linux_x86_64/decimal/format_digits.asm"
