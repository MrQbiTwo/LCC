; Constant conversion operates on target bit patterns under the compiler's
; owned nearest-even environment. It never calls or executes user functions.
section .text
; Convert RAX:RDX from type RDI to type RSI, retaining target-width rounding.
constant_convert:
    sub rsp, 40
    mov [rsp], rax
    mov [rsp+8], rdx
    mov r8d, [rdi+TY_KIND]
    mov r9d, [rsi+TY_KIND]
    cmp r8d, K_FLOAT
    jb .integer_source
    cmp r8d, K_LDOUBLE
    ja .integer_source
    cmp r9d, K_FLOAT
    jb .fp_integer
    cmp r9d, K_LDOUBLE
    ja generation_error
    cmp r8d, r9d
    je .done
    cmp r8d, K_LDOUBLE
    je .extended_source
    cmp r9d, K_LDOUBLE
    je .extended_target
    cmp r8d, K_FLOAT
    jne .narrow
    movd xmm0, eax
    cvtss2sd xmm0, xmm0
    movq rax, xmm0
    jmp .done
.narrow:
    movq xmm0, rax
    cvtsd2ss xmm0, xmm0
    movd eax, xmm0
    jmp .done
.extended_source:
    fld tword [rsp]
    cmp r9d, K_FLOAT
    jne .extended_double
    fstp dword [rsp]
    mov eax, [rsp]
    jmp .done
.extended_double:
    fstp qword [rsp]
    mov rax, [rsp]
    jmp .done
.extended_target:
    cmp r8d, K_FLOAT
    jne .double_extended
    fld dword [rsp]
    jmp .store_extended
.double_extended:
    fld qword [rsp]
.store_extended:
    fstp tword [rsp]
    mov rax, [rsp]
    movzx edx, word [rsp+8]
    jmp .done
.integer_source:
    cmp r9d, K_FLOAT
    jb .normalize
    cmp r9d, K_LDOUBLE
    ja .normalize
    je .integer_extended
    xor r10d, r10d
    cmp dword [rdi+TY_BYTES], 8
    jne .signed_fp
    test dword [rdi+TY_FLAGS], 1
    jz .signed_fp
    test rax, rax
    jns .signed_fp
    mov r10d, 1
    mov rcx, rax
    and ecx, 1
    shr rax, 1
    or rax, rcx
.signed_fp:
    cmp r9d, K_FLOAT
    jne .integer_double
    cvtsi2ss xmm0, rax
    test r10d, r10d
    jz .single_bits
    addss xmm0, xmm0
.single_bits:
    movd eax, xmm0
    jmp .done
.integer_double:
    cvtsi2sd xmm0, rax
    test r10d, r10d
    jz .double_bits
    addsd xmm0, xmm0
.double_bits:
    movq rax, xmm0
    jmp .done
.integer_extended:
    xor edx, edx
    test dword [rdi+TY_FLAGS], 1
    jnz .magnitude
    test rax, rax
    jns .magnitude
    neg rax
    mov edx, 0x8000
.magnitude:
    test rax, rax
    jz .done
    bsr rcx, rax
    lea r10d, [rcx+16383]
    or edx, r10d
    neg ecx
    add ecx, 63
    shl rax, cl
    jmp .done
.fp_integer:
    cmp r9d, K_VOID
    je .done
    call constant_try_float_integer
    test ecx, ecx
    jz .done
.range:
    lea rdi, [msg_range]
    jmp fatal
.normalize:
    cmp r9d, K_VOID
    je .done
    cmp r9d, K_PTR
    jae .done
    mov ecx, [rsi+TY_BYTES]
    test dword [rsi+TY_FLAGS], 1
    jnz .unsigned
    cmp ecx, 1
    je .signed_byte
    cmp ecx, 2
    je .signed_word
    cmp ecx, 4
    jne .done
    movsxd rax, eax
    jmp .done
.signed_byte:
    movsx rax, al
    jmp .done
.signed_word:
    movsx rax, ax
    jmp .done
.unsigned:
    cmp ecx, 1
    je .unsigned_byte
    cmp ecx, 2
    je .unsigned_word
    cmp ecx, 4
    jne .done
    mov eax, eax
    jmp .done
.unsigned_byte:
    movzx eax, al
    jmp .done
.unsigned_word:
    movzx eax, ax
.done:
    add rsp, 40
    ret
; Return boolean RAX from the bit pattern RAX:RDX and scalar type RDI.
constant_truth:
    cmp dword [rdi+TY_KIND], K_FLOAT
    jne .double
    and eax, 0x7fffffff
    jmp .test
.double:
    cmp dword [rdi+TY_KIND], K_DOUBLE
    jne .test
    btr rax, 63
.test:
    test rax, rax
    setne al
    movzx eax, al
    ret
