; RDI type, RSI byte offset, RDX builder, ECX permits brace elision,
; R8 is the bit-field member for a scalar leaf, otherwise zero.
; Builder: root, tail, pending expression consumed by the first scalar leaf.
initializer_type:
    push rbx
    push r12
    push r13
    push r14
    push r15
    sub rsp, 32
    mov rbx, rdi
    mov r12, rsi
    mov r13, rdx
    mov [rsp+8], ecx
    mov [rsp+16], r8
    mov qword [rsp], 0
    inc qword [initializer_depth]
    cmp qword [initializer_depth], 128
    ja aggregate_error
    cmp qword [r13+16], 0
    jne .dispatch
    mov edi, '{'
    call accept
    mov [rsp], rax
.dispatch:
    mov eax, [rbx+TY_KIND]
    cmp eax, K_ARRAY
    je .array
    cmp eax, K_STRUCT
    jae .record
    ; Scalar braces may themselves contain another braced scalar.
    cmp qword [rsp], 0
    je .scalar
    mov rdi, rbx
    mov rsi, r12
    mov rdx, r13
    xor ecx, ecx
    mov r8, [rsp+16]
    call initializer_type
    jmp .close
.scalar:
    mov r14, [r13+16]
    mov qword [r13+16], 0
    test r14, r14
    jnz .scalar_ready
    mov edi, 2
    call parse_expression
    mov r14, rax
.scalar_ready:
    mov rax, [r14+N_TYPE]
    cmp dword [rax+TY_KIND], K_STRUCT
    jae aggregate_error
    jmp .leaf
.array:
    mov rax, [rbx+TY_BASE]
    cmp dword [rax+TY_KIND], K_CHAR
    jne .array_list
    cmp qword [r13+16], 0
    jne .array_list
    mov rax, [parse_token]
    cmp dword [rax+TK_KIND], T_STRING
    je .string
.array_list:
    cmp qword [rsp], 0
    jne .list
    cmp dword [rsp+8], 0
    je aggregate_error
    jmp .list
.record:
    cmp qword [rsp], 0
    jne .list
    mov r14, [r13+16]
    test r14, r14
    jnz .elided
    mov edi, 2
    call parse_expression
    mov r14, rax
    mov rsi, [r14+N_TYPE]
    cmp dword [rsi+TY_KIND], K_STRUCT
    jb .pending
    mov rdi, rbx
    call type_aggregate_common
    jmp .leaf
.pending:
    mov [r13+16], r14
.elided:
    cmp dword [rsp+8], 0
    je aggregate_error
.list:
    xor r14d, r14d
    mov r15, [rbx+TY_MEMBERS]
.element:
    xor r8d, r8d
    mov rsi, r12
    mov rdi, [rbx+TY_BASE]
    cmp dword [rbx+TY_KIND], K_ARRAY
    jne .member
    mov eax, [rdi+TY_BYTES]
    imul rax, r14
    add rsi, rax
    jmp .child
.member:
    test r15, r15
    jz aggregate_error
    cmp qword [r15+MEM_NAME], 0
    jne .named_member
    mov r15, [r15+MEM_NEXT]
    jmp .member
.named_member:
    mov rdi, [r15+MEM_TYPE]
    add rsi, [r15+MEM_OFFSET]
    cmp dword [r15+MEM_BITS], 0
    je .child
    mov r8, r15
.child:
    mov rdx, r13
    mov ecx, 1
    call initializer_type
    inc r14
    cmp dword [rbx+TY_KIND], K_ARRAY
    jne .next_member
    test dword [rbx+TY_FLAGS], TF_INCOMPLETE
    jnz .separator
    cmp r14, [rbx+TY_COUNT]
    jae .filled
    jmp .separator
.next_member:
    cmp dword [rbx+TY_KIND], K_UNION
    je .filled
    mov r15, [r15+MEM_NEXT]
.skip_unnamed:
    test r15, r15
    jz .filled
    cmp qword [r15+MEM_NAME], 0
    jne .separator
    mov r15, [r15+MEM_NEXT]
    jmp .skip_unnamed
.separator:
    mov rax, [parse_token]
    cmp dword [rax+TK_KIND], ','
    jne .filled
    mov rax, [rax+TK_NEXT]
    cmp dword [rax+TK_KIND], '}'
    je .filled
    mov edi, ','
    call expect
    jmp .element
.filled:
    cmp dword [rbx+TY_KIND], K_ARRAY
    jne .filled_size
    test dword [rbx+TY_FLAGS], TF_INCOMPLETE
    jz .filled_size
    mov rdi, rbx
    mov rsi, r14
    call type_array_complete
.filled_size:
    cmp qword [rsp], 0
    je .done
.close:
    mov edi, ','
    call accept
    mov edi, '}'
    call expect
    jmp .done
.string:
    call parse_string
    mov r14, rax
    test dword [rbx+TY_FLAGS], TF_INCOMPLETE
    jz .string_size
    mov rdi, rbx
    mov rsi, [r14+N_AUX]
    call type_array_complete
.string_size:
    mov rax, [r14+N_AUX]
    dec rax
    cmp rax, [rbx+TY_COUNT]
    ja aggregate_error
    mov rdi, rbx
    mov rsi, r12
    mov rdx, r13
    mov rcx, r14
    call initializer_leaf
    mov rcx, [r14+N_AUX]
    cmp rcx, [rbx+TY_COUNT]
    cmova rcx, [rbx+TY_COUNT]
    mov [rax+N_AUX], rcx
    cmp qword [rsp], 0
    jne .close
    jmp .done
.leaf:
    mov rdi, r14
    mov rsi, rbx
    call require_implicit_conversion
    mov rdi, rbx
    mov rsi, r12
    mov rdx, r13
    mov rcx, r14
    call initializer_leaf
    mov rcx, [rsp+16]
    mov [rax+N_SYM], rcx
.done:
    dec qword [initializer_depth]
    add rsp, 32
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
