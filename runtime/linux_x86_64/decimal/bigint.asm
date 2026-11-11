; Exact decimal-to-binary conversion shared by the compiler and native runtime.
; All arithmetic here is integer arithmetic; x87 and MXCSR state are untouched.
%define DEC_CAP 1024
%define DEC_BIG 8208
%define DEC_CTX (DEC_BIG*3)
%define DC_FORMAT (DEC_CTX+0)
%define DC_SCALE (DEC_CTX+8)
%define DC_FRAC (DEC_CTX+16)
%define DC_DIGITS (DEC_CTX+24)
%define DC_DOT (DEC_CTX+32)
%define DC_SIGN (DEC_CTX+40)
%define DC_K (DEC_CTX+48)
%define DC_E (DEC_CTX+56)
%define DC_P (DEC_CTX+64)
%define DC_BIAS (DEC_CTX+72)
%define DC_MIN (DEC_CTX+80)
%define DC_MAX (DEC_CTX+88)
%define DC_STATUS (DEC_CTX+96)
%define DC_EXP_NEG (DEC_CTX+104)
%define DC_EXP (DEC_CTX+112)
%define DEC_FRAME (DEC_CTX+264)
section .rodata
; Precision, bias, minimum/maximum binary exponent, and decimal early-out bounds.
dec_formats:
    dq 24,127,-126,127,39,-46
    dq 53,1023,-1022,1023,309,-325
    dq 64,16383,-16382,16383,4933,-4952
section .text
; Copy big integer RSI to RDI, including its active limb count.
dec_copy:
    mov rcx, [rsi]
    inc rcx
    rep movsq
    ret
; Multiply big integer RDI by RSI; return nonzero on capacity overflow.
dec_multiply:
    mov rcx, [rdi]
    xor r8d, r8d
    xor r9d, r9d
.loop:
    cmp r9, rcx
    jae .carry
    mov rax, [rdi+r9*8+8]
    mul rsi
    add rax, r8
    adc rdx, 0
    mov [rdi+r9*8+8], rax
    mov r8, rdx
    inc r9
    jmp .loop
.carry:
    test r8, r8
    jz .ok
    cmp rcx, DEC_CAP
    jae .overflow
    mov [rdi+rcx*8+8], r8
    inc rcx
    mov [rdi], rcx
.ok:
    xor eax, eax
    ret
.overflow:
    mov eax, 1
    ret
; Add small unsigned RSI to big integer RDI.
dec_add_small:
    test rsi, rsi
    jz .ok
    mov rcx, [rdi]
    xor edx, edx
.loop:
    cmp rdx, rcx
    jae .append
    add [rdi+rdx*8+8], rsi
    jnc .ok
    mov esi, 1
    inc rdx
    jmp .loop
.append:
    cmp rcx, DEC_CAP
    jae .overflow
    mov [rdi+rcx*8+8], rsi
    inc rcx
    mov [rdi], rcx
.ok:
    xor eax, eax
    ret
.overflow:
    mov eax, 1
    ret
; Return the exact bit length of big integer RDI.
dec_bits:
    mov rcx, [rdi]
    xor eax, eax
    test rcx, rcx
    jz .done
    mov rdx, [rdi+rcx*8]
    bsr rax, rdx
    dec rcx
    shl rcx, 6
    lea rax, [rax+rcx+1]
.done:
    ret
; Compare big integers RDI and RSI, returning -1, 0, or 1.
dec_compare:
    mov rcx, [rdi]
    cmp rcx, [rsi]
    jb .below
    ja .above
.loop:
    test rcx, rcx
    jz .equal
    mov rax, [rdi+rcx*8]
    cmp rax, [rsi+rcx*8]
    jb .below
    ja .above
    dec rcx
    jmp .loop
.below:
    mov eax, -1
    ret
.above:
    mov eax, 1
    ret
.equal:
    xor eax, eax
    ret
; Subtract RSI from RDI, requiring RDI >= RSI.
dec_subtract:
    mov r10, [rdi]
    mov r11, [rsi]
    xor ecx, ecx
    xor r8d, r8d
.loop:
    cmp rcx, r10
    jae .trim
    xor edx, edx
    cmp rcx, r11
    jae .subtract
    mov rdx, [rsi+rcx*8+8]
.subtract:
    mov rax, [rdi+rcx*8+8]
    sub rax, rdx
    setb dl
    sub rax, r8
    setb r9b
    or dl, r9b
    movzx r8d, dl
    mov [rdi+rcx*8+8], rax
    inc rcx
    jmp .loop
.trim:
    test r10, r10
    jz .done
    cmp qword [rdi+r10*8], 0
    jne .done
    dec r10
    jmp .trim
.done:
    mov [rdi], r10
    ret
; Shift a big integer RDI left by RSI bits, checking its bounded capacity.
dec_shift_left:
    push rbx
    push r12
    sub rsp, 8
    mov rbx, rdi
    mov r12, rsi
    mov rcx, [rbx]
    test rcx, rcx
    jz .ok
    mov r8, rsi
    shr r8, 6
    lea r9, [rcx+r8]
    cmp r9, DEC_CAP
    ja .overflow
    mov [rbx], r9
.move:
    test rcx, rcx
    jz .zeros
    mov rax, [rbx+rcx*8]
    mov [rbx+r9*8], rax
    dec rcx
    dec r9
    jmp .move
.zeros:
    test r8, r8
    jz .bits
    mov qword [rbx+r8*8], 0
    dec r8
    jmp .zeros
.bits:
    and r12d, 63
    jz .ok
    mov ecx, r12d
    mov esi, 1
    shl rsi, cl
    mov rdi, rbx
    call dec_multiply
    jmp .done
.ok:
    xor eax, eax
    jmp .done
.overflow:
    mov eax, 1
.done:
    add rsp, 8
    pop r12
    pop rbx
    ret
; Shift a big integer RDI right by one bit.
dec_shift_right:
    mov rcx, [rdi]
    xor r8d, r8d
.loop:
    test rcx, rcx
    jz .trim
    mov rax, [rdi+rcx*8]
    mov rdx, rax
    shl rdx, 63
    shr rax, 1
    or rax, r8
    mov [rdi+rcx*8], rax
    mov r8, rdx
    dec rcx
    jmp .loop
.trim:
    mov rcx, [rdi]
    test rcx, rcx
    jz .done
    cmp qword [rdi+rcx*8], 0
    jne .done
    dec qword [rdi]
.done:
    ret
