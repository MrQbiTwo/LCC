; Conditional expressions retain decayed pointer types and pointee qualifiers.
section .text
; Decay array/function type RDI to a pointer; leave other types unchanged.
type_decay:
    mov rax, rdi
    cmp dword [rdi+TY_KIND], K_FUNC
    je type_pointer
    cmp dword [rdi+TY_KIND], K_ARRAY
    jne .done
    mov rdi, [rdi+TY_BASE]
    jmp type_pointer
.done:
    ret
; Choose the conditional result type for expression nodes RDI and RSI.
type_conditional:
    push rbx
    push r12
    push r13
    push r14
    push r15
    mov r12, rdi
    mov r13, rsi
    call require_complete_value
    mov rdi, r13
    call require_complete_value
    mov rdi, [r12+N_TYPE]
    call type_decay
    mov rbx, rax
    mov rdi, [r13+N_TYPE]
    call type_decay
    mov r14, rax
    ; Two void arms produce void, never an arithmetic common type. Preserve
    ; that result so enclosing operators cannot consume a fictitious int.
    cmp dword [rbx+TY_KIND], K_VOID
    jne .nonvoid_left
    cmp dword [r14+TY_KIND], K_VOID
    jne .bad_arms
    lea rax, [type_void]
    jmp .done
.nonvoid_left:
    cmp dword [r14+TY_KIND], K_VOID
    je .bad_arms
    cmp dword [rbx+TY_KIND], K_PTR
    je .pointer
    cmp dword [r14+TY_KIND], K_PTR
    je .swap
    mov rdi, rbx
    mov rsi, r14
    call type_common
    jmp .done
.swap:
    xchg rbx, r14
    xchg r12, r13
.pointer:
    cmp dword [r14+TY_KIND], K_PTR
    je .pointees
    cmp dword [r14+TY_KIND], K_CHAR
    jb aggregate_error
    cmp dword [r14+TY_KIND], K_LONG
    ja aggregate_error
    mov rdi, r13
    call evaluate_required_integer_constant
    test rax, rax
    jnz aggregate_error
    mov rax, rbx
    jmp .done
.pointees:
    mov rdi, r12
    call conditional_null
    test eax, eax
    jnz .right_result
    mov rdi, r13
    call conditional_null
    test eax, eax
    jz .non_null_pointees
    mov rax, rbx
    jmp .done
.right_result:
    mov rax, r14
    jmp .done
.non_null_pointees:
    mov rbx, [rbx+TY_BASE]
    mov r14, [r14+TY_BASE]
    mov r15d, [rbx+TY_FLAGS]
    or r15d, [r14+TY_FLAGS]
    and r15d, TF_CONST|TF_VOLATILE
    cmp dword [rbx+TY_KIND], K_VOID
    je .void_left
    cmp dword [r14+TY_KIND], K_VOID
    je .void_right
    mov rdi, rbx
    mov rsi, r14
    call type_composite_pointee
    mov rbx, rax
    jmp .qualified
.void_right:
    xchg rbx, r14
.void_left:
    cmp dword [r14+TY_KIND], K_FUNC
    je aggregate_error
.qualified:
    test r15d, r15d
    jz .pointer_result
    mov rdi, rbx
    mov esi, r15d
    call type_qualify
    mov rbx, rax
.pointer_result:
    mov rdi, rbx
    call type_pointer
    jmp .done
.bad_arms:
    mov rax, [r13+N_TOKEN]
    mov [diag_token], rax
    jmp aggregate_error
.done:
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
; Compare pointees ignoring only their outer const/volatile qualification.
; Keep record identity canonical; nested pointer qualifiers still must agree.
conditional_pointee_match:
    mov eax, [rdi+TY_KIND]
    cmp eax, [rsi+TY_KIND]
    jne aggregate_error
    cmp eax, K_STRUCT
    jae type_aggregate_common
    sub rsp, TY_SIZE*2+8
    mov rdx, rsi
    mov rsi, rdi
    mov rdi, rsp
    mov ecx, TY_SIZE/8
    rep movsq
    mov rsi, rdx
    lea rdi, [rsp+TY_SIZE]
    mov ecx, TY_SIZE/8
    rep movsq
    and dword [rsp+TY_FLAGS], ~(TF_CONST|TF_VOLATILE)
    and dword [rsp+TY_SIZE+TY_FLAGS], ~(TF_CONST|TF_VOLATILE)
    mov rdi, rsp
    lea rsi, [rsp+TY_SIZE]
    call type_array_element_match
    add rsp, TY_SIZE*2+8
    ret
; Recognize unqualified void-pointer casts of integer constant zero without
; evaluating runtime pointer expressions used in otherwise valid conditionals.
conditional_null:
    push rbx
    xor eax, eax
    cmp dword [rdi+N_KIND], A_CAST
    jne .done
    mov rdx, [rdi+N_TYPE]
    cmp dword [rdx+TY_KIND], K_PTR
    jne .done
    mov rdx, [rdx+TY_BASE]
    cmp dword [rdx+TY_KIND], K_VOID
    jne .done
    ; Qualified void pointers must take normal pointee/qualifier composition.
    test dword [rdx+TY_FLAGS], TF_CONST|TF_VOLATILE
    jnz .done
    mov rdi, [rdi+N_LEFT]
    call constant_try_integer_expression
    test ecx, ecx
    jnz .not_null
    test rax, rax
    sete al
    movzx eax, al
    jmp .done
.not_null:
    xor eax, eax
.done:
    pop rbx
    ret
