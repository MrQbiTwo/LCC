; Resolve a label in the function namespace, retaining unresolved forward uses.
section .text
parse_label_find:
    push rbx
    push r12
    sub rsp, 8
    mov r12, rdi
    mov rbx, [parse_labels]
.next:
    test rbx, rbx
    jz .new
    mov rdi, [rbx]
    mov rsi, r12
    call token_same
    test eax, eax
    jnz .found
    mov rbx, [rbx+8]
    jmp .next
.new:
    mov edi, 32
    call arena_alloc
    mov rbx, rax
    mov [rbx], r12
    mov rax, [parse_labels]
    mov [rbx+8], rax
    mov [parse_labels], rbx
    xor edi, edi
    call label_new
    mov [rbx+16], rax
.found:
    mov rax, rbx
    add rsp, 8
    pop r12
    pop rbx
    ret

; Install a block-scope typedef in the ordinary identifier namespace.
section .text
local_alias_new:
    push rbx
    push r12
    push r13
    mov r12, rdi
    mov r13, rsi
    call symbol_find
    test rax, rax
    jz .allocate
    test dword [rax+S_FLAGS], SF_LOCAL
    jz .allocate
    mov ecx, [scope_depth]
    cmp ecx, [rax+S_SCOPE]
    je .duplicate
.allocate:
    mov edi, S_SIZE
    call arena_alloc
    mov [rax+S_NAME], r12
    mov [rax+S_TYPE], r13
    mov dword [rax+S_FLAGS], SF_LOCAL|SF_TYPEDEF
    mov ecx, [scope_depth]
    mov [rax+S_SCOPE], ecx
    mov rdx, [local_symbols]
    mov [rax+S_NEXT], rdx
    mov [local_symbols], rax
    pop r13
    pop r12
    pop rbx
    ret
.duplicate:
    mov [diag_token], r12
    lea rdi, [msg_name]
    jmp fatal
