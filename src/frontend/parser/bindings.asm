; Lexical names can refer to storage entities without owning stack homes.
section .text
; Resolve symbol RDI to its canonical storage/function identity in RAX.
symbol_entity:
    mov rax, rdi
    test dword [rax+S_FLAGS], SF_BINDING
    jz .done
    mov rax, [rax+S_ENTITY]
.done:
    ret
; Find a linked entity by token RDI, including declarations hidden outside blocks.
symbol_linkage_find:
    push rbx
    push r12
    sub rsp, 8
    mov r12, rdi
    mov rbx, [global_symbols]
.next:
    test rbx, rbx
    jz .done
    test dword [rbx+S_FLAGS], SF_TYPEDEF|SF_ENUM|SF_BLOCK_STATIC
    jnz .skip
    mov rdi, [rbx+S_NAME]
    mov rsi, r12
    call token_same
    test eax, eax
    jnz .done
.skip:
    mov rbx, [rbx+S_NEXT]
    jmp .next
.done:
    mov rax, rbx
    add rsp, 8
    pop r12
    pop rbx
    ret
; Find a name RDI only in the current block, retaining the raw lexical binding.
local_find_current:
    push rbx
    push r12
    sub rsp, 8
    mov r12, rdi
    mov rbx, [local_symbols]
.next:
    test rbx, rbx
    jz .done
    mov eax, [scope_depth]
    cmp eax, [rbx+S_SCOPE]
    jne .absent
    mov rdi, [rbx+S_NAME]
    mov rsi, r12
    call token_same
    test eax, eax
    jnz .done
    mov rbx, [rbx+S_NEXT]
    jmp .next
.absent:
    xor ebx, ebx
.done:
    mov rax, rbx
    add rsp, 8
    pop r12
    pop rbx
    ret
; Install name RDI/type RSI in this scope, referring to entity RDX.
local_binding_new:
    push rbx
    push r12
    push r13
    mov rbx, rdi
    mov r12, rsi
    mov r13, rdx
    mov edi, S_SIZE
    call arena_alloc
    mov [rax+S_NAME], rbx
    mov [rax+S_TYPE], r12
    mov [rax+S_ENTITY], r13
    mov dword [rax+S_FLAGS], SF_LOCAL|SF_BINDING
    mov ecx, [scope_depth]
    mov [rax+S_SCOPE], ecx
    mov rdx, [local_symbols]
    mov [rax+S_NEXT], rdx
    mov [local_symbols], rax
    pop r13
    pop r12
    pop rbx
    ret
; Create block-static storage for name RDI/type RSI and publish a lexical alias.
local_static_new:
    push rbx
    push r12
    push r13
    mov r12, rdi
    mov r13, rsi
    call local_find_current
    test rax, rax
    jnz storage_duplicate
    mov rdi, r13
    call type_array_object
    mov r13, rax
    mov rdi, r13
    call type_object_pending
    cmp dword [r13+TY_KIND], K_VOID
    je aggregate_error
    cmp dword [r13+TY_KIND], K_FUNC
    je aggregate_error
    mov edi, S_SIZE
    call arena_alloc
    mov rbx, rax
    mov [rbx+S_NAME], r12
    mov [rbx+S_TYPE], r13
    mov dword [rbx+S_FLAGS], SF_STATIC|SF_DEFINED|SF_HIDDEN|SF_BLOCK_STATIC
    mov rax, [global_symbols]
    mov [rbx+S_NEXT], rax
    mov [global_symbols], rbx
    mov rdi, r12
    mov rsi, r13
    mov rdx, rbx
    call local_binding_new
    pop r13
    pop r12
    pop rbx
    ret
; Duplicate lexical declarations use the stable name diagnostic.
storage_duplicate:
    lea rdi, [msg_name]
    jmp fatal
