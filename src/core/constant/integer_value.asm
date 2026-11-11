; Canonical integer bits and checked signed target ranges for constant folding.
section .text
; Normalize RAX to integer/pointer type RDI; only RCX is clobbered.
constant_integer_normalize:
    mov ecx, [rdi+TY_BYTES]
    test dword [rdi+TY_FLAGS], 1
    jnz .unsigned
    cmp ecx, 1
    je .i8
    cmp ecx, 2
    je .i16
    cmp ecx, 4
    jne .done
    movsxd rax, eax
    ret
.i8:
    movsx rax, al
    ret
.i16:
    movsx rax, ax
    ret
.unsigned:
    cmp ecx, 1
    je .u8
    cmp ecx, 2
    je .u16
    cmp ecx, 4
    jne .done
    mov eax, eax
    ret
.u8:
    movzx eax, al
    ret
.u16:
    movzx eax, ax
.done:
    ret
; Check signed RAX against promoted type RDI; ECX=0 or 11, RDX is scratch.
; The caller checks 64-bit CPU overflow before calling.
constant_try_integer_range:
    xor ecx, ecx
    cmp dword [rdi+TY_BYTES], 4
    jne .done
    movsxd rdx, eax
    cmp rdx, rax
    je .done
    mov ecx, 11
.done:
    ret

; Required unary arithmetic preserves the operator's diagnostic location.
constant_integer_unary:
    mov rax, [rdi+N_TOKEN]
    mov [diag_token], rax
    sub rsp, 8
    call constant_try_integer_unary
    add rsp, 8
    test ecx, ecx
    jnz constant_arithmetic_error
    ret

; RDI node, RSI operand bits -> RAX result, RDX=0 on success, ECX=0/8/11.
; No diagnostics or fatal branches; ordinary and optional folding share this.
constant_try_integer_unary:
    push rbx
    mov rbx, rdi
    mov rax, rsi
    cmp dword [rbx+N_OP], '!'
    je .not
    mov rdi, [rbx+N_TYPE]
    call constant_integer_normalize
    cmp dword [rbx+N_OP], '-'
    je .negate
    cmp dword [rbx+N_OP], '~'
    je .complement
    cmp dword [rbx+N_OP], '+'
    je .normalize
    mov ecx, 8
    jmp .done
.negate:
    neg rax
    seto dl
    test dword [rdi+TY_FLAGS], 1
    jnz .normalize
    test dl, dl
    jnz .range
    call constant_try_integer_range
    test ecx, ecx
    jnz .done
    jmp .normalize
.complement:
    not rax
    jmp .normalize
.not:
    test rax, rax
    sete al
    movzx eax, al
.normalize:
    mov rdi, [rbx+N_TYPE]
    call constant_integer_normalize
    xor edx, edx
    xor ecx, ecx
.done:
    pop rbx
    ret
.range:
    mov ecx, 11
    jmp .done

; Required wrappers translate a failed shared arithmetic status to diagnostics.
constant_arithmetic_error:
    cmp ecx, 11
    jne generation_error
; Report invalid constant arithmetic at the operator's restored source token.
constant_integer_error:
    lea rdi, [msg_range]
    jmp fatal
