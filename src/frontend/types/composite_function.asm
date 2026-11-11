; Function composites own their new parameter list, not declaration/body homes.
section .text
; Compatible function types RDI/RSI, ECX=remaining depth -> RAX=composite.
; Retain the available prototype and recursively compose returns/parameters.
type_composite_function:
    push rbx
    push r12
    push r13
    push r14
    push r15
    sub rsp, 32
    mov rbx, rdi
    mov r12, rsi
    lea r13d, [rcx-1]
    mov r14, rbx
    test dword [rbx+TY_FLAGS], 2
    jz .result
    mov r14, r12
.result:
    mov rdi, [rbx+TY_BASE]
    mov rsi, [r12+TY_BASE]
    mov ecx, r13d
    call type_composite_build
    mov [rsp], rax
    mov rdi, r14
    call type_composite_copy
    mov r15, rax
    mov rax, [rsp]
    mov [r15+TY_BASE], rax
    ; If either list is unspecified, compose the prototype's parameters with
    ; themselves to strip only their top qualifiers on fresh parameter records.
    mov eax, [rbx+TY_FLAGS]
    or eax, [r12+TY_FLAGS]
    test eax, 2
    jz .lists
    mov rbx, r14
    mov r12, r14
.lists:
    mov rbx, [rbx+TY_PARAMS]
    mov r12, [r12+TY_PARAMS]
    mov qword [r15+TY_PARAMS], 0
    xor r14d, r14d
.parameter:
    test rbx, rbx
    jz .done
    mov rdi, [rbx+S_TYPE]
    mov rsi, [r12+S_TYPE]
    mov ecx, r13d
    call type_composite_build
    test dword [rax+TY_FLAGS], TF_CONST|TF_VOLATILE
    jz .parameter_type
    mov rdi, rax
    call type_composite_copy
    and dword [rax+TY_FLAGS], ~(TF_CONST|TF_VOLATILE)
.parameter_type:
    mov [rsp+8], rax
    mov edi, S_SIZE
    call arena_alloc
    mov rsi, rbx
    mov rdi, rax
    mov ecx, S_SIZE/8
    rep movsq
    mov rdx, [rsp+8]
    mov [rax+S_TYPE], rdx
    mov qword [rax+S_PARAM_NEXT], 0
    test r14, r14
    jz .first
    mov [r14+S_PARAM_NEXT], rax
    jmp .linked
.first:
    mov [r15+TY_PARAMS], rax
.linked:
    mov r14, rax
    mov rbx, [rbx+S_PARAM_NEXT]
    mov r12, [r12+S_PARAM_NEXT]
    jmp .parameter
.done:
    mov rax, r15
    add rsp, 32
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
