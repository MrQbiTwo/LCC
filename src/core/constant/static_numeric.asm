; Fold static numeric subtrees through existing target-width arithmetic rules.
section .text
; Replace already evaluated children with temporary literal nodes, preserving
; numeric bits/types without mutating the AST or losing nested pointer scaling.
static_numeric_expression:
    push rbx
    push r12
    push r13
    sub rsp, N_SIZE*3
    mov rbx, rdi
    mov rsi, rdi
    mov rdi, rsp
    mov ecx, N_SIZE/8
    rep movsq
    mov r12, [rbx+N_LEFT]
    mov rsi, r12
    lea rdi, [rsp+N_SIZE]
    mov ecx, N_SIZE/8
    rep movsq
    mov rdi, r12
    call evaluate_static_constant
    test r8, r8
    jnz generation_error
    mov dword [rsp+N_SIZE+N_KIND], A_NUM
    mov [rsp+N_SIZE+N_VALUE], rax
    mov [rsp+N_SIZE+N_AUX], rdx
    lea rax, [rsp+N_SIZE]
    mov [rsp+N_LEFT], rax
    cmp dword [rbx+N_KIND], A_UNARY
    je .fold
    mov r12, [rbx+N_RIGHT]
    mov rsi, r12
    lea rdi, [rsp+N_SIZE*2]
    mov ecx, N_SIZE/8
    rep movsq
    mov rdi, r12
    call evaluate_static_constant
    test r8, r8
    jnz generation_error
    mov dword [rsp+N_SIZE*2+N_KIND], A_NUM
    mov [rsp+N_SIZE*2+N_VALUE], rax
    mov [rsp+N_SIZE*2+N_AUX], rdx
    lea rax, [rsp+N_SIZE*2]
    mov [rsp+N_RIGHT], rax
.fold:
    mov rdi, rsp
    call evaluate_constant
    xor r8d, r8d
    add rsp, N_SIZE*3
    pop r13
    pop r12
    pop rbx
    ret
; Evaluate numeric &&/|| without evaluating an unused static operand.
static_logical:
    push rbx
    push r12
    sub rsp, 8
    mov rbx, rdi
    mov r12, [rbx+N_LEFT]
    mov rdi, r12
    call evaluate_static_constant
    test r8, r8
    jnz generation_error
    mov rdi, [r12+N_TYPE]
    call constant_truth
    cmp dword [rbx+N_OP], T_AND
    jne .or
    test eax, eax
    jz .done
    jmp .right
.or:
    test eax, eax
    jnz .done
.right:
    mov r12, [rbx+N_RIGHT]
    mov rdi, r12
    call evaluate_static_constant
    test r8, r8
    jnz generation_error
    mov rdi, [r12+N_TYPE]
    call constant_truth
.done:
    xor r8d, r8d
    xor edx, edx
    add rsp, 8
    pop r12
    pop rbx
    ret
