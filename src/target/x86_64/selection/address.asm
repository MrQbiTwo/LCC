; Typed instruction selection for expressions and addressable objects.
section .text
; Emit the address of an lvalue node RDI into generated RAX.
generate_address:
    push rbx
    mov rbx, rdi
    mov eax, [rbx+N_KIND]
    cmp eax, A_DEREF
    je .deref
    cmp eax, A_MEMBER
    je .member
    cmp eax, A_STRING
    je .string
    cmp eax, A_VAR
    jne generation_error
    mov rdi, [rbx+N_SYM]
    call symbol_entity
    mov rbx, rax
    test dword [rbx+S_FLAGS], SF_LOCAL
    jz .global
    mov rdi, [rbx+S_OFFSET]
    call emit_frame_address
    jmp .done
.global:
    mov rdi, [rbx+S_LABEL]
    call emit_label_address
    jmp .done
.deref:
    mov rdi, [rbx+N_LEFT]
    call generate_expression
    jmp .done
.string:
    call generate_string
    jmp .done
.member:
    mov rdi, [rbx+N_LEFT]
    call generate_expression
    lea rdi, [p_add_rax]
    mov rsi, [rbx+N_VALUE]
    mov edx, 4
    call emit_immediate
.done:
    pop rbx
    ret
