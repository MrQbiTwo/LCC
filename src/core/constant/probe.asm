; Optional integer folding reports failure without changing diagnostics.
section .text
; RDI expression -> ECX=0/8/11; RAX canonical bits and RDX=0 on success.
; Classify every operand first; only arithmetic in selected paths is evaluated.
constant_try_integer_expression:
    push rbx
    mov rbx, rdi
    call integer_constant_form
    test eax, eax
    jz .invalid
    mov rdi, rbx
    pop rbx
    jmp constant_probe_integer
.invalid:
    xor edx, edx
    mov ecx, 8
    pop rbx
    ret

; Walk an already classified integer expression using the shared value helpers.
constant_probe_integer:
    push rbx
    push r12
    sub rsp, 8
    mov rbx, rdi
    mov eax, [rbx+N_KIND]
    cmp eax, A_NUM
    je .literal
    cmp eax, A_CAST
    je .cast
    cmp eax, A_UNARY
    je .unary
    cmp eax, A_COND
    je .conditional
    cmp eax, A_BINARY
    jne .invalid
    mov rdi, [rbx+N_LEFT]
    call constant_probe_integer
    test ecx, ecx
    jnz .done
    cmp dword [rbx+N_OP], T_AND
    je .and
    cmp dword [rbx+N_OP], T_OR
    je .or
    mov r12, rax
    mov rdi, [rbx+N_RIGHT]
    call constant_probe_integer
    test ecx, ecx
    jnz .done
    mov rdx, rax
    mov rsi, r12
    mov rdi, rbx
    call constant_try_integer_binary
    jmp .done
.and:
    test rax, rax
    jz .boolean
    jmp .logical_right
.or:
    test rax, rax
    jnz .boolean
.logical_right:
    mov rdi, [rbx+N_RIGHT]
    call constant_probe_integer
    test ecx, ecx
    jnz .done
.boolean:
    test rax, rax
    setne al
    movzx eax, al
    jmp .normalize
.literal:
    mov rax, [rbx+N_VALUE]
    jmp .normalize
.cast:
    mov r12, [rbx+N_LEFT]
    mov rdi, [r12+N_TYPE]
    cmp dword [rdi+TY_KIND], K_FLOAT
    jb .integer_cast
    ; Classification permits only an immediate floating literal here.
    mov rax, [r12+N_VALUE]
    mov rdx, [r12+N_AUX]
    mov rsi, [rbx+N_TYPE]
    call constant_try_float_integer
    jmp .done
.integer_cast:
    mov rdi, r12
    call constant_probe_integer
    test ecx, ecx
    jnz .done
    jmp .normalize
.unary:
    mov rdi, [rbx+N_LEFT]
    call constant_probe_integer
    test ecx, ecx
    jnz .done
    mov rsi, rax
    mov rdi, rbx
    call constant_try_integer_unary
    jmp .done
.conditional:
    mov rdi, [rbx+N_COND]
    call constant_probe_integer
    test ecx, ecx
    jnz .done
    test rax, rax
    mov rdi, [rbx+N_THEN]
    cmovz rdi, [rbx+N_ELSE]
    call constant_probe_integer
    test ecx, ecx
    jnz .done
.normalize:
    mov rdi, [rbx+N_TYPE]
    call constant_integer_normalize
    xor edx, edx
    xor ecx, ecx
.done:
    add rsp, 8
    pop r12
    pop rbx
    ret
.invalid:
    xor edx, edx
    mov ecx, 8
    jmp .done
