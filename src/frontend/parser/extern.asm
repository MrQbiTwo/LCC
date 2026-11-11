; Block extern/function declarations share linked identity, not lexical scope.
section .text
; Bind name RDI/type RSI to a linked entity; return the scoped declaration.
local_extern_new:
    push rbx
    push r12
    push r13
    push r14
    push r15
    mov r12, rdi
    mov r13, rsi
    xor ebx, ebx                 ; Existing same-scope binding, if any.
    mov rdi, r13
    call type_array_object
    mov r13, rax
    cmp dword [r13+TY_KIND], K_VOID
    je aggregate_error
    mov rdi, r12
    call local_find_current
    test rax, rax
    jz .visible
    test dword [rax+S_FLAGS], SF_BINDING
    jz storage_duplicate
    mov rbx, rax
    mov r15, [rax+S_ENTITY]
    test dword [r15+S_FLAGS], SF_BLOCK_STATIC
    jnz storage_duplicate
    mov r14, rax
    jmp .validate
.visible:
    mov rdi, r12
    call symbol_find
    mov r14, rax
    test rax, rax
    jz .external
    test dword [rax+S_FLAGS], SF_TYPEDEF|SF_ENUM
    jnz .external
    test dword [rax+S_FLAGS], SF_BINDING
    jnz .binding
    test dword [rax+S_FLAGS], SF_LOCAL
    jnz .external
    mov r15, rax
    jmp .validate
.binding:
    mov r15, [rax+S_ENTITY]
    test dword [r15+S_FLAGS], SF_BLOCK_STATIC
    jz .validate
.external:
    xor r14d, r14d               ; No visible declaration with linkage.
    mov rdi, r12
    call symbol_linkage_find
    mov r15, rax
    test rax, rax
    jz .new
    test dword [rax+S_FLAGS], SF_STATIC
    jnz aggregate_error
    jmp .validate
.new:
    mov edi, S_SIZE
    call arena_alloc
    mov r15, rax
    mov [rax+S_NAME], r12
    mov dword [rax+S_FLAGS], SF_HIDDEN
    mov rdi, r13
    call type_array_object
    mov [r15+S_TYPE], rax
    mov rax, [global_symbols]
    mov [r15+S_NEXT], rax
    mov [global_symbols], r15
.validate:
    mov rdi, r15
    mov rsi, r13
    call entity_type_merge
    test rbx, rbx
    jnz .repeat
    test r14, r14
    jz .publish
    ; Compose into this scope's type without completing an outer array type.
    mov rdi, r13
    mov rsi, [r14+S_TYPE]
    call declaration_type_merge
    mov r13, rax
.publish:
    mov rdi, r12
    mov rsi, r13
    mov rdx, r15
    call local_binding_new
    jmp .done
.repeat:
    mov rdi, [rbx+S_TYPE]
    mov rsi, r13
    call declaration_type_merge
    mov [rbx+S_TYPE], rax
    mov rax, rbx
.done:
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
