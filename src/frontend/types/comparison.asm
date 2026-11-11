; Equality and relational constraints precede common-type selection.
section .text
; RDI=left node, RSI=right node, EDX=operator. Return on valid operands.
; Preserve callee-saved registers; do not mutate types or evaluate operands.
require_comparison_operands:
    push rbx
    push r12
    push r13
    push r14
    push r15
    mov rbx, rdi
    mov r12, rsi
    mov r13d, edx
    mov r14, [rdi+N_TYPE]
    mov r15, [rsi+N_TYPE]
    mov eax, [r14+TY_KIND]
    mov edx, [r15+TY_KIND]
    ; Array/function operands decay for comparisons without allocating types.
    cmp eax, K_CHAR
    jb .bad
    cmp eax, K_FUNC
    ja .bad
    cmp edx, K_CHAR
    jb .bad
    cmp edx, K_FUNC
    ja .bad
    cmp eax, K_PTR
    jae .pointer
    cmp edx, K_PTR
    jb .done
    xchg r14, r15
    xchg rbx, r12
.pointer:
    cmp dword [r15+TY_KIND], K_PTR
    jae .pointers
    ; A non-pointer operand must be an integer null pointer constant.
    cmp r13d, T_EQ
    je .integer_null
    cmp r13d, T_NE
    jne .bad
.integer_null:
    cmp dword [r15+TY_KIND], K_LONG
    ja .bad
    mov rdi, r12
    call comparison_null
    test eax, eax
    jz .bad
    jmp .done
.pointers:
    cmp r13d, T_EQ
    je .null_pointers
    cmp r13d, T_NE
    jne .pointees
.null_pointers:
    ; An unqualified void-pointer cast of integer ICE zero also permits
    ; comparison with a function pointer. Other void pointers do not.
    mov rdi, rbx
    call conditional_null
    test eax, eax
    jnz .done
    mov rdi, r12
    call conditional_null
    test eax, eax
    jnz .done
.pointees:
    cmp dword [r14+TY_KIND], K_FUNC
    je .left_base
    mov r14, [r14+TY_BASE]
.left_base:
    cmp dword [r15+TY_KIND], K_FUNC
    je .right_base
    mov r15, [r15+TY_BASE]
.right_base:
    cmp r13d, T_EQ
    je .equality
    cmp r13d, T_NE
    je .equality
    ; Relational pointers may name compatible incomplete types, including
    ; void, but never functions. No complete-object stride is needed here.
    cmp dword [r14+TY_KIND], K_FUNC
    je .bad
    cmp dword [r15+TY_KIND], K_FUNC
    je .bad
    ; Relational pairs must both be objects or both incomplete types.
    ; Compatible arrays can differ in completeness; canonical record identity
    ; already implies the same completion state, even for old qualified copies.
    cmp dword [r14+TY_KIND], K_ARRAY
    jne .compatible
    mov eax, [r14+TY_FLAGS]
    xor eax, [r15+TY_FLAGS]
    test eax, TF_INCOMPLETE
    jnz .bad
    jmp .compatible
.equality:
    cmp dword [r14+TY_KIND], K_VOID
    je .void_left
    cmp dword [r15+TY_KIND], K_VOID
    jne .compatible
    cmp dword [r14+TY_KIND], K_FUNC
    je .bad
    jmp .done
.void_left:
    cmp dword [r15+TY_KIND], K_FUNC
    je .bad
    jmp .done
.compatible:
    mov rdi, r14
    mov rsi, r15
    call type_pointee_compatible
    test eax, eax
    jz .bad
.done:
    ; Callers consume only success/failure.
    mov rax, rbx
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
.bad:
    mov rax, [r12+N_TOKEN]
    mov [diag_token], rax
    jmp aggregate_error

; RDI=integer node -> EAX=whether it is an integer constant expression zero.
; Failed optional probes, including invalid constant arithmetic, mean false.
comparison_null:
    sub rsp, 8
    call constant_try_integer_expression
    test ecx, ecx
    jnz .no
    test rax, rax
    sete al
    movzx eax, al
    add rsp, 8
    ret
.no:
    xor eax, eax
    add rsp, 8
    ret
