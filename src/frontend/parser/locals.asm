section .bss
align 8
parse_loops: resq 1 ; Active loop nesting for break and continue validation.
parse_switch: resq 1 ; Innermost switch node, restored across nested switches.
parse_labels: resq 1 ; Function-wide label records: name, next, target, defined.
statement_depth: resq 1 ; Bounded recursive statement nesting.
section .text
; Create a local symbol from name RDI and type RSI, assigning its stack home.
local_new:
    push rbx
    push r12
    push r13
    mov r12, rdi
    mov r13, rsi
    mov rbx, [local_symbols]
.check:
    test rbx, rbx
    jz .allocate
    mov eax, [scope_depth]
    cmp eax, [rbx+S_SCOPE]
    jne .allocate
    mov rdi, [rbx+S_NAME]
    mov rsi, r12
    call token_same
    test eax, eax
    jnz .duplicate
    mov rbx, [rbx+S_NEXT]
    jmp .check
.allocate:
    mov rdi, r13
    call type_array_object
    mov r13, rax
    mov rdi, r13
    call type_object_pending
    cmp dword [r13+TY_KIND], K_VOID
    je .bad_type
    cmp dword [r13+TY_KIND], K_FUNC
    je .bad_type
    mov edi, S_SIZE
    call arena_alloc
    mov [rax+S_NAME], r12
    mov [rax+S_TYPE], r13
    mov dword [rax+S_FLAGS], SF_LOCAL
    mov edx, [scope_depth]
    mov [rax+S_SCOPE], edx
    mov rdx, [local_symbols]
    mov [rax+S_NEXT], rdx
    mov [local_symbols], rax
    test dword [r13+TY_FLAGS], TF_INCOMPLETE
    jnz .done
    mov rdi, rax
    call local_assign_home
.done:
    pop r13
    pop r12
    pop rbx
    ret
.duplicate:
    lea rdi, [msg_name]
    jmp fatal
.bad_type:
    lea rdi, [msg_type]
    jmp fatal
; Assign symbol RDI a home once its initializer has completed any array bound.
local_assign_home:
    push rbx
    mov rbx, rdi
    mov rdi, [rbx+S_TYPE]
    call type_require_complete
    cmp qword [rbx+S_OFFSET], 0
    jne .done
    mov rdi, rax
    mov edx, [rdi+TY_BYTES]
    add rdx, [local_bytes]
    mov ecx, [rdi+TY_ALIGN]
    dec ecx
    add rdx, rcx
    not rcx
    and rdx, rcx
    cmp rdx, MAX_MEMORY
    ja aggregate_error
    mov [local_bytes], rdx
    mov [rbx+S_OFFSET], rdx
.done:
    mov rax, rbx
    pop rbx
    ret
; Reserve an aligned, padded frame temporary for an aggregate call result.
local_temporary:
    xor eax, eax
    cmp qword [current_function], 0
    je .done
    mov eax, [rdi+TY_BYTES]
    add rax, 15
    and rax, -16
    add rax, [local_bytes]
    add rax, 15
    and rax, -16
    cmp rax, MAX_MEMORY
    ja aggregate_error
    mov [local_bytes], rax
.done:
    ret
