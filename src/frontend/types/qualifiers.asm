; Qualify a fresh type record; array aliases qualify their elements as well.
section .text
type_qualify:
    push rbx
    push r12
    push r13
    mov rbx, rdi
    mov r12d, esi
    cmp dword [rbx+TY_KIND], K_FUNC
    je .bad
    mov edi, TY_SIZE
    call arena_alloc
    mov r13, rax
    mov rdi, rax
    mov rsi, rbx
    mov ecx, TY_SIZE/8
    rep movsq
    or [r13+TY_FLAGS], r12d
    cmp dword [r13+TY_KIND], K_STRUCT
    jb .array
    mov rax, rbx
.canonical:
    cmp qword [rax+TY_BASE], 0
    je .canonical_found
    mov rax, [rax+TY_BASE]
    jmp .canonical
.canonical_found:
    mov [r13+TY_BASE], rax
.array:
    cmp dword [r13+TY_KIND], K_ARRAY
    jne .done
    mov rdi, [r13+TY_BASE]
    mov esi, r12d
    call type_qualify
    mov [r13+TY_BASE], rax
.done:
    mov rax, r13
    pop r13
    pop r12
    pop rbx
    ret
.bad:
    lea rdi, [msg_type]
    jmp fatal
; Require an assignable object, excluding const, array, function, and void types.
require_modifiable:
    mov rax, [rdi+N_TOKEN]
    mov [diag_token], rax
    sub rsp, 8
    call require_lvalue
    add rsp, 8
    mov rdi, [rdi+N_TYPE]
    sub rsp, 8
    call type_require_complete
    add rsp, 8
    test dword [rax+TY_FLAGS], TF_CONST|TF_CONST_MEMBER
    jnz .bad
    cmp dword [rax+TY_KIND], K_VOID
    je .bad
    cmp dword [rax+TY_KIND], K_ARRAY
    je .bad
    cmp dword [rax+TY_KIND], K_FUNC
    je .bad
    ret
.bad:
    lea rdi, [msg_type]
    jmp fatal
; A dot member is an lvalue only when its aggregate operand is one.
require_lvalue:
    mov rax, rdi
.next:
    mov ecx, [rax+N_KIND]
    cmp ecx, A_VAR
    je .done
    cmp ecx, A_DEREF
    je .done
    cmp ecx, A_STRING
    je .done
    cmp ecx, A_MEMBER
    jne aggregate_error
    cmp dword [rax+N_OP], T_ARROW
    je .done
    mov rax, [rax+N_LEFT]
    jmp .next
.done:
    ret
; Validate increment/decrement operands, including complete pointer targets.
require_step_type:
    cmp dword [rdi+TY_KIND], K_STRUCT
    jae aggregate_error
    cmp dword [rdi+TY_KIND], K_PTR
    jne .done
    mov rdi, [rdi+TY_BASE]
    cmp dword [rdi+TY_KIND], K_VOID
    je aggregate_error
    cmp dword [rdi+TY_KIND], K_FUNC
    je aggregate_error
    jmp type_require_complete
.done:
    ret
