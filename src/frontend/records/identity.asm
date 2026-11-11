; Resolve a qualified forward tag once its canonical definition becomes complete.
section .text
type_resolve:
    mov rax, rdi
    cmp dword [rax+TY_KIND], K_STRUCT
    jb .done
    test dword [rax+TY_FLAGS], TF_INCOMPLETE
    jz .done
    mov rdx, [rax+TY_BASE]
    test rdx, rdx
    jz .done
    test dword [rdx+TY_FLAGS], TF_INCOMPLETE
    jnz .done
    mov ecx, [rdx+TY_BYTES]
    mov [rax+TY_BYTES], ecx
    mov ecx, [rdx+TY_ALIGN]
    mov [rax+TY_ALIGN], ecx
    mov rcx, [rdx+TY_MEMBERS]
    mov [rax+TY_MEMBERS], rcx
    and dword [rax+TY_FLAGS], ~TF_INCOMPLETE
    mov ecx, [rdx+TY_FLAGS]
    and ecx, TF_CONST_MEMBER
    or [rax+TY_FLAGS], ecx
.done:
    ret
; Reject objects and layout queries requiring a still-incomplete target type.
; Canonical identity, not equal size or field names, determines tag compatibility.
type_aggregate_common:
    cmp dword [rdi+TY_KIND], K_STRUCT
    jb aggregate_error
    cmp dword [rsi+TY_KIND], K_STRUCT
    jb aggregate_error
    mov rax, rdi
.left:
    cmp qword [rdi+TY_BASE], 0
    je .right
    mov rdi, [rdi+TY_BASE]
    jmp .left
.right:
    cmp qword [rsi+TY_BASE], 0
    je .compare
    mov rsi, [rsi+TY_BASE]
    jmp .right
.compare:
    cmp rdi, rsi
    jne aggregate_error
    ret
; Resolve and require an object layout.
type_require_complete:
    sub rsp, 8
    call type_resolve
    add rsp, 8
    test dword [rax+TY_FLAGS], TF_INCOMPLETE
    jnz aggregate_error
    ret
; Find a member token RSI in the complete structure/union type RDI.
member_find:
    push rbx
    push r12
    sub rsp, 8
    mov r12, rsi
    call type_require_complete
    mov rbx, [rax+TY_MEMBERS]
.next:
    test rbx, rbx
    jz .done
    cmp qword [rbx+MEM_NAME], 0
    je .unnamed
    mov rdi, [rbx+MEM_NAME]
    mov rsi, r12
    call token_same
    test eax, eax
    jnz .done
.unnamed:
    mov rbx, [rbx+MEM_NEXT]
    jmp .next
.done:
    mov rax, rbx
    add rsp, 8
    pop r12
    pop rbx
    ret
