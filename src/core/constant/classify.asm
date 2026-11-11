; Integer constant expression syntax is stricter than numerical foldability.
section .text
; Validate RDI before folding; disallowed syntax is E008, arithmetic keeps E011.
evaluate_required_integer_constant:
    push rbx
    mov rbx, rdi
    call integer_constant_form
    test eax, eax
    jz .invalid
    mov rdi, rbx
    pop rbx
    jmp evaluate_constant
.invalid:
    mov rax, [rdx+N_TOKEN]
    mov [diag_token], rax
    lea rdi, [msg_type]
    pop rbx
    jmp fatal

; RDI node -> EAX boolean; RDX identifies the first disallowed node on failure.
; Inspect every operand without evaluating arithmetic or changing diagnostics.
; sizeof, enumeration constants and offsetof already arrive as integer A_NUM.
integer_constant_form:
    push rbx
.node:
    mov rbx, rdi
    mov rdx, rdi
    mov rax, [rbx+N_TYPE]
    cmp dword [rax+TY_KIND], K_CHAR
    jb .no
    cmp dword [rax+TY_KIND], K_LONG
    ja .no
    mov eax, [rbx+N_KIND]
    cmp eax, A_NUM
    je .yes
    cmp eax, A_CAST
    je .cast
    cmp eax, A_UNARY
    je .unary
    cmp eax, A_BINARY
    je .binary
    cmp eax, A_COND
    jne .no
    mov rdi, [rbx+N_COND]
    call integer_constant_form
    test eax, eax
    jz .done
    mov rdi, [rbx+N_THEN]
    call integer_constant_form
    test eax, eax
    jz .done
    mov rdi, [rbx+N_ELSE]
    jmp .node
.binary:
    mov rdi, [rbx+N_RIGHT]
    call integer_constant_form
    test eax, eax
    jz .done
.left:
    ; Iteration avoids additional recursion along flat left-associated trees.
    mov rdi, [rbx+N_LEFT]
    jmp .node
.unary:
    mov eax, [rbx+N_OP]
    cmp eax, '+'
    je .left
    cmp eax, '-'
    je .left
    cmp eax, '~'
    je .left
    cmp eax, '!'
    je .left
    jmp .no
.cast:
    mov rdi, [rbx+N_LEFT]
    cmp dword [rdi+N_KIND], A_NUM
    jne .node
    mov rax, [rdi+N_TYPE]
    cmp dword [rax+TY_KIND], K_FLOAT
    jb .node
    cmp dword [rax+TY_KIND], K_LDOUBLE
    ja .node
    ; C89 permits a floating literal only as an immediate integer-cast operand.
.yes:
    mov eax, 1
    jmp .done
.no:
    xor eax, eax
.done:
    pop rbx
    ret
