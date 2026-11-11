; Incomplete array ownership, completion, and declaration composition.
section .text
; Clone an unknown outer bound for one object, leaving typedef aliases unchanged.
; RDI is the declaration type; RAX is the object's type, RDX is preserved.
type_array_object:
    mov rax, rdi
    cmp dword [rdi+TY_KIND], K_ARRAY
    jne .done
    test dword [rdi+TY_FLAGS], TF_INCOMPLETE
    jz .done
    push rdx
    push rdi
    sub rsp, 8
    mov edi, TY_SIZE
    call arena_alloc
    mov rsi, [rsp+8]
    mov rdi, rax
    mov ecx, TY_SIZE/8
    rep movsq
    add rsp, 8
    pop rdi
    pop rdx
.done:
    ret
; Permit only a complete object or an outer array awaiting its bound.
type_object_pending:
    cmp dword [rdi+TY_KIND], K_ARRAY
    jne type_require_complete
    mov rax, rdi
    ret
; Complete array RDI with positive element count RSI, bounded by MAX_MEMORY.
type_array_complete:
    test rsi, rsi
    jz aggregate_error
    cmp rsi, MAX_MEMORY
    ja aggregate_error
    mov rdx, [rdi+TY_BASE]
    mov eax, [rdx+TY_BYTES]
    imul rax, rsi
    cmp rax, MAX_MEMORY
    ja aggregate_error
    mov [rdi+TY_BYTES], eax
    mov [rdi+TY_COUNT], rsi
    and dword [rdi+TY_FLAGS], ~TF_INCOMPLETE
    mov rax, rdi
    ret
; Merge compatible declarations into the stable per-object array record RDI.
; RSI is the new array declaration; complete bounds must agree.
type_array_redeclare:
    push rbx
    push r12
    sub rsp, 8
    mov rbx, rdi
    mov r12, rsi
    cmp dword [rdi+TY_KIND], K_ARRAY
    jne aggregate_error
    cmp dword [rsi+TY_KIND], K_ARRAY
    jne aggregate_error
    mov rdi, [rbx+TY_BASE]
    mov rsi, [r12+TY_BASE]
    call type_array_element_match
    mov rsi, [r12+TY_COUNT]
    test rsi, rsi
    jz .done
    cmp qword [rbx+TY_COUNT], 0
    je .complete
    cmp [rbx+TY_COUNT], rsi
    jne aggregate_error
    jmp .done
.complete:
    mov rdi, rbx
    call type_array_complete
.done:
    mov rax, rbx
    add rsp, 8
    pop r12
    pop rbx
    ret
; Compare array element shapes, qualifiers and canonical aggregate identity.
; Function types here require matching signatures; wider compatibility is separate.
type_array_element_match:
    cmp rdi, rsi
    je .done
    mov eax, [rdi+TY_KIND]
    cmp eax, [rsi+TY_KIND]
    jne aggregate_error
    cmp eax, K_FUNC
    je .function
    mov edx, [rdi+TY_FLAGS]
    xor edx, [rsi+TY_FLAGS]
    test edx, 1|TF_CONST|TF_VOLATILE
    jnz aggregate_error
    cmp eax, K_STRUCT
    jae type_aggregate_common
    cmp eax, K_PTR
    je .base
    cmp eax, K_ARRAY
    jne .primitive
    mov rax, [rdi+TY_COUNT]
    cmp rax, [rsi+TY_COUNT]
    jne aggregate_error
.base:
    mov rdi, [rdi+TY_BASE]
    mov rsi, [rsi+TY_BASE]
    jmp type_array_element_match
.primitive:
    sub rsp, 8
    call type_primitive_compatible
    add rsp, 8
    test eax, eax
    jz aggregate_error
    ret
.function:
    mov eax, [rdi+TY_FLAGS]
    xor eax, [rsi+TY_FLAGS]
    test eax, 3
    jnz aggregate_error
    push rbx
    push r12
    sub rsp, 8
    mov rbx, [rdi+TY_PARAMS]
    mov r12, [rsi+TY_PARAMS]
    mov rdi, [rdi+TY_BASE]
    mov rsi, [rsi+TY_BASE]
    call type_array_element_match
.parameter:
    test rbx, rbx
    jz .parameters_done
    test r12, r12
    jz aggregate_error
    mov rdi, [rbx+S_TYPE]
    mov rsi, [r12+S_TYPE]
    call type_array_element_match
    mov rbx, [rbx+S_PARAM_NEXT]
    mov r12, [r12+S_PARAM_NEXT]
    jmp .parameter
.parameters_done:
    test r12, r12
    jnz aggregate_error
    add rsp, 8
    pop r12
    pop rbx
.done:
    ret
