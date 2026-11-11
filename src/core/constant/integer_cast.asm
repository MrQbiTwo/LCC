; Nonfatal floating-to-integer constant conversion, using target bit patterns.
section .text
; RAX:RDX source bits, RDI floating type, RSI integer type (char through long).
; Return canonical RAX, RDX=0 and ECX=0; ECX=11 on range, 8 on unsupported type.
; Failure leaves RAX unspecified. Preserve callee-saved registers and diagnostics.
constant_try_float_integer:
    mov r8d, [rdi+TY_KIND]
    mov r9d, [rsi+TY_KIND]
    cmp r8d, K_FLOAT
    jb .unsupported
    cmp r8d, K_LDOUBLE
    ja .unsupported
    cmp r9d, K_CHAR
    jb .unsupported
    cmp r9d, K_LONG
    ja .unsupported
    ; Extract sign, unbiased exponent, and a left-aligned significand. Range
    ; checks use the truncated magnitude, including negative fractions near zero.
    xor r10d, r10d
    cmp r8d, K_LDOUBLE
    je .extended
    cmp r8d, K_FLOAT
    jne .double
    mov r10d, eax
    shr r10d, 31
    mov ecx, eax
    shr ecx, 23
    and ecx, 255
    cmp ecx, 255
    je .range
    sub ecx, 127
    and eax, 0x7fffff
    or eax, 0x800000
    shl rax, 40
    jmp .magnitude
.double:
    mov r10, rax
    shr r10, 63
    mov rcx, rax
    shr rcx, 52
    and ecx, 2047
    cmp ecx, 2047
    je .range
    sub ecx, 1023
    shl rax, 12
    shr rax, 1
    bts rax, 63
    jmp .magnitude
.extended:
    mov r10d, edx
    shr r10d, 15
    mov ecx, edx
    and ecx, 0x7fff
    cmp ecx, 0x7fff
    je .range
    sub ecx, 16383
.magnitude:
    test ecx, ecx
    js .tiny
    cmp ecx, 64
    jae .range
    neg ecx
    add ecx, 63
    shr rax, cl
    mov ecx, [rsi+TY_BYTES]
    shl ecx, 3
    test dword [rsi+TY_FLAGS], 1
    jz .signed_range
    test r10d, r10d
    jnz .range
    cmp ecx, 64
    je .normalize
    mov r11d, 1
    shl r11, cl
    cmp rax, r11
    jae .range
    jmp .normalize
.signed_range:
    dec ecx
    mov r11d, 1
    shl r11, cl
    test r10d, r10d
    jnz .negative_range
    cmp rax, r11
    jae .range
    jmp .normalize
.negative_range:
    cmp rax, r11
    ja .range
    neg rax
    jmp .normalize
.tiny:
    xor eax, eax
.normalize:
    sub rsp, 8
    mov rdi, rsi
    call constant_integer_normalize
    add rsp, 8
    xor edx, edx
    xor ecx, ecx
    ret
.range:
    mov ecx, 11
    xor edx, edx
    ret
.unsupported:
    mov ecx, 8
    xor edx, edx
    ret
