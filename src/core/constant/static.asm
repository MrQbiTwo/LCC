; Static initializers retain a symbolic label separately from numeric bits.
section .text
; RDI expression -> RAX value/addend, RDX numeric high bits, R8 optional label.
; No static expression may read the stored value of an object.
evaluate_static_constant:
    push rbx
    push r12
    push r13
    mov rbx, rdi
    mov rax, [rbx+N_TOKEN]
    mov [diag_token], rax
    mov eax, [rbx+N_KIND]
    cmp eax, A_ADDR
    je .address
    cmp eax, A_STRING
    je .lvalue
    cmp eax, A_VAR
    je .decay
    cmp eax, A_MEMBER
    je .decay
    cmp eax, A_DEREF
    je .decay
    cmp eax, A_CAST
    je .cast
    cmp eax, A_COND
    je .conditional
    cmp eax, A_UNARY
    je .numeric_tree
    cmp eax, A_BINARY
    jne .numeric
    cmp dword [rbx+N_OP], T_AND
    je .logical
    cmp dword [rbx+N_OP], T_OR
    je .logical
    cmp dword [rbx+N_OP], '+'
    je .binary
    cmp dword [rbx+N_OP], '-'
    jne .numeric_tree
.binary:
    call static_binary
    jmp .done
.decay:
    mov rax, [rbx+N_TYPE]
    cmp dword [rax+TY_KIND], K_ARRAY
    je .lvalue
    cmp dword [rax+TY_KIND], K_FUNC
    jne generation_error
.lvalue:
    mov rdi, rbx
    call static_lvalue
    jmp .done
.address:
    mov rdi, [rbx+N_LEFT]
    call static_lvalue
    jmp .done
.cast:
    mov r12, [rbx+N_LEFT]
    jmp .converted_child
.conditional:
    mov r12, [rbx+N_COND]
    mov rdi, r12
    call evaluate_static_constant
    test r8, r8
    jnz generation_error
    mov rdi, [r12+N_TYPE]
    call constant_truth
    test eax, eax
    mov r12, [rbx+N_THEN]
    cmovz r12, [rbx+N_ELSE]
.converted_child:
    mov rdi, r12
    call evaluate_static_constant
    mov rdi, [r12+N_TYPE]
    mov rsi, [rbx+N_TYPE]
    call static_convert
    jmp .done
.logical:
    call static_logical
    jmp .done
.numeric_tree:
    mov rdi, rbx
    call static_numeric_expression
    jmp .done
.numeric:
    mov rdi, rbx
    call evaluate_constant
    xor r8d, r8d
.done:
    pop r13
    pop r12
    pop rbx
    ret
; Convert static bits or preserve an address through a pointer/64-bit integer.
; A symbolic value cannot be narrowed or converted to floating point.
static_convert:
    test r8, r8
    jz .numeric
    cmp dword [rsi+TY_KIND], K_PTR
    je .symbolic
    cmp dword [rsi+TY_KIND], K_LONG
    jne generation_error
    cmp dword [rsi+TY_BYTES], 8
    jne generation_error
.symbolic:
    xor edx, edx
    ret
.numeric:
    sub rsp, 8
    call constant_convert
    xor r8d, r8d
    add rsp, 8
    ret
; Integer offsets must fit the signed relocation-addend representation.
; RAX is the offset; RDI is its integer type. Return it unchanged.
static_offset:
    cmp dword [rdi+TY_KIND], K_CHAR
    jb generation_error
    cmp dword [rdi+TY_KIND], K_LONG
    ja generation_error
    test dword [rdi+TY_FLAGS], 1
    jz .done
    test rax, rax
    js static_range
.done:
    ret
; Report an unrepresentable static address or signed addend.
static_range:
    lea rdi, [msg_range]
    jmp fatal
