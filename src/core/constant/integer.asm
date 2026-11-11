; Integer tree traversal delegates target arithmetic to the shared value helper.
section .text
; RDI integer node -> canonical RAX bits and zero RDX.
; Casts, conditionals, logical operators and floating operands route elsewhere.
evaluate_integer_constant:
    push rbx
    push r12
    sub rsp, 8
    mov rbx, rdi
    mov rax, [rbx+N_TOKEN]
    mov [diag_token], rax
    mov eax, [rbx+N_KIND]
    cmp eax, A_NUM
    je .literal
    cmp eax, A_UNARY
    je .unary
    cmp eax, A_BINARY
    jne generation_error
    mov rdi, [rbx+N_LEFT]
    call evaluate_constant
    mov r12, rax
    mov rdi, [rbx+N_RIGHT]
    call evaluate_constant
    mov rdx, rax
    mov rsi, r12
    mov rdi, rbx
    call constant_integer_binary
    jmp .done
.literal:
    mov rax, [rbx+N_VALUE]
    jmp .normalize
.unary:
    mov rdi, [rbx+N_LEFT]
    call evaluate_constant
    mov rsi, rax
    mov rdi, rbx
    call constant_integer_unary
    jmp .done
.normalize:
    mov rdi, [rbx+N_TYPE]
    call constant_integer_normalize
.done:
    xor edx, edx
    add rsp, 8
    pop r12
    pop rbx
    ret
