; Complete file-scope tentative storage before lowering reads object layouts.
section .text
; SF_DEFINED means storage is owed; unused externs and type names owe none.
finalize_objects:
    push rbx
    mov rbx, [global_symbols]
.object:
    test rbx, rbx
    jz .done
    test dword [rbx+S_FLAGS], SF_TYPEDEF|SF_ENUM
    jnz .next
    test dword [rbx+S_FLAGS], SF_DEFINED
    jz .next
    mov rdi, [rbx+S_TYPE]
    cmp dword [rdi+TY_KIND], K_FUNC
    je .next
    mov rax, [rbx+S_NAME]
    mov [diag_token], rax
    cmp dword [rdi+TY_KIND], K_ARRAY
    je .array
    ; Qualified forward tags need their canonical layout copied before emission.
    call type_require_complete
    jmp .next
.array:
    test dword [rdi+TY_FLAGS], TF_INCOMPLETE
    jz .array_complete
    mov esi, 1
    call type_array_complete
.array_complete:
    mov rdi, [rbx+S_TYPE]
    call type_require_complete
    ; Check any inferred/final bound against declarations from other scopes.
    mov rdi, rbx
    mov rsi, [rbx+S_TYPE]
    call entity_type_merge
.next:
    mov rbx, [rbx+S_NEXT]
    jmp .object
.done:
    pop rbx
    ret
