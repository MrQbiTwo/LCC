; Parse a structure or union with target alignment; EDI selects its kind.
parse_aggregate:
    push rbx
    push r12
    push r13
    push r14
    push r15
    sub rsp, 48
    mov [rsp], edi
    mov qword [rsp+8], 0
    mov qword [rsp+16], 0
    mov qword [rsp+40], 0
    inc qword [aggregate_depth]
    cmp qword [aggregate_depth], 128
    ja aggregate_error
    mov edi, T_IDENT
    call accept
    mov r12, rax
    xor r13d, r13d
    test r12, r12
    jz .brace
    mov rdi, r12
    call tag_find
    mov r13, rax
.brace:
    mov edi, '{'
    call accept
    mov [rsp+24], rax
    test r13, r13
    jz .new_type
    mov eax, [scope_depth]
    cmp rax, [r13+16]
    je .existing
    cmp qword [rsp+24], 0
    jne .new_type
    mov rax, [parse_token]
    cmp dword [rax+TK_KIND], ';'
    je .new_type
.existing:
    mov eax, [rsp]
    cmp rax, [r13+32]
    jne aggregate_error
    mov rbx, [r13+8]
    cmp qword [rsp+24], 0
    je .done
    test dword [rbx+TY_FLAGS], TF_INCOMPLETE
    jz .duplicate
    jmp .fields
.new_type:
    test r12, r12
    jnz .allocate
    cmp qword [rsp+24], 0
    je aggregate_error
.allocate:
    mov edi, TY_SIZE
    call arena_alloc
    mov rbx, rax
    mov eax, [rsp]
    mov [rbx+TY_KIND], eax
    mov dword [rbx+TY_ALIGN], 1
    mov dword [rbx+TY_FLAGS], TF_INCOMPLETE
    test r12, r12
    jz .fields
    mov edi, 40
    call arena_alloc
    mov [rax], r12
    mov [rax+8], rbx
    mov ecx, [scope_depth]
    mov [rax+16], rcx
    mov rcx, [tag_symbols]
    mov [rax+24], rcx
    mov ecx, [rsp]
    mov [rax+32], rcx
    mov [tag_symbols], rax
    cmp qword [rsp+24], 0
    je .done
.fields:
    call parse_declspec
    mov r12, rax
    cmp qword [decl_storage], 0
    jne aggregate_error
.declarator:
    mov rdi, r12
    mov esi, 1
    call parse_declarator
    mov r13, rax
    mov r14, rdx
    mov rdi, r13
    call type_require_complete
    cmp dword [r13+TY_KIND], K_VOID
    je aggregate_error
    cmp dword [r13+TY_KIND], K_FUNC
    je aggregate_error
    mov qword [rsp+32], -1
    mov edi, ':'
    call accept
    test rax, rax
    jz .ordinary_member
    cmp dword [r13+TY_KIND], K_INT
    jne aggregate_error
    test dword [r13+TY_FLAGS], TF_ENUM
    jnz aggregate_error
    mov edi, 3
    call parse_expression
    mov rdi, rax
    mov rax, [rax+N_TYPE]
    cmp dword [rax+TY_KIND], K_CHAR
    jb aggregate_error
    cmp dword [rax+TY_KIND], K_LONG
    ja aggregate_error
    call evaluate_required_integer_constant
    cmp rax, 32
    ja aggregate_error
    mov [rsp+32], rax
    test rax, rax
    jnz .bitfield_type
    test r14, r14
    jnz aggregate_error
    cmp dword [rbx+TY_KIND], K_UNION
    je .more
    add qword [rsp+8], 31
    and qword [rsp+8], -32
    jmp .more
.bitfield_type:
    ; Field-only type metadata keeps declared sign separate from promotions.
    mov edi, TY_SIZE
    call arena_alloc
    mov rdi, rax
    mov rsi, r13
    mov ecx, TY_SIZE/8
    rep movsq
    mov r13, rax
    or dword [r13+TY_FLAGS], TF_BITFIELD
    mov rax, [rsp+32]
    mov [r13+TY_COUNT], rax
    jmp .check_name
.ordinary_member:
    test r14, r14
    jz aggregate_error
.check_name:
    test r14, r14
    jz .member
    inc qword [rsp+40]
    ; Check duplicate names even while the containing type is incomplete.
    mov r15, [rbx+TY_MEMBERS]
.check:
    test r15, r15
    jz .member
    cmp qword [r15+MEM_NAME], 0
    je .check_next
    mov rdi, [r15+MEM_NAME]
    mov rsi, r14
    call token_same
    test eax, eax
    jnz .duplicate
.check_next:
    mov r15, [r15+MEM_NEXT]
    jmp .check
.member:
    mov edi, MEM_SIZE
    call arena_alloc
    mov r15, rax
    mov [r15+MEM_NAME], r14
    mov [r15+MEM_TYPE], r13
    mov rax, [rsp+16]
    test rax, rax
    jz .first
    mov [rax+MEM_NEXT], r15
    jmp .linked
.first:
    mov [rbx+TY_MEMBERS], r15
.linked:
    mov [rsp+16], r15
    cmp qword [rsp+32], -1
    jne .bitfield_layout
    mov ecx, [r13+TY_ALIGN]
    cmp ecx, [rbx+TY_ALIGN]
    jbe .alignment
    mov [rbx+TY_ALIGN], ecx
.alignment:
    mov rax, [rsp+8]
    cmp dword [rbx+TY_KIND], K_UNION
    je .union
    add rax, 7
    shr rax, 3
    dec ecx
    add rax, rcx
    not rcx
    and rax, rcx
    mov [r15+MEM_OFFSET], rax
    mov edx, [r13+TY_BYTES]
    add rax, rdx
    shl rax, 3
    jmp .size
.union:
    mov edx, [r13+TY_BYTES]
    shl rdx, 3
    cmp rax, rdx
    cmovb rax, rdx
    jmp .size
.bitfield_layout:
    test r14, r14
    jz .bitfield_position
    cmp dword [rbx+TY_ALIGN], 4
    jae .bitfield_position
    mov dword [rbx+TY_ALIGN], 4
.bitfield_position:
    mov rax, [rsp+8]
    mov edx, [rsp+32]
    mov [r15+MEM_BITS], edx
    cmp dword [rbx+TY_KIND], K_UNION
    je .bitfield_union
    mov ecx, eax
    and ecx, 31
    add ecx, edx
    cmp ecx, 32
    jbe .bitfield_offset
    add rax, 31
    and rax, -32
.bitfield_offset:
    mov ecx, eax
    and ecx, 7
    mov [r15+MEM_SHIFT], ecx
    mov rcx, rax
    shr rcx, 3
    mov [r15+MEM_OFFSET], rcx
    add rax, rdx
    jmp .size
.bitfield_union:
    cmp rax, rdx
    cmovb rax, rdx
.size:
    mov rcx, MAX_MEMORY*8
    cmp rax, rcx
    ja aggregate_error
    mov [rsp+8], rax
    test r14, r14
    jz .more
    mov rax, r13
.const_array:
    test dword [rax+TY_FLAGS], TF_CONST|TF_CONST_MEMBER
    jnz .const_member
    cmp dword [rax+TY_KIND], K_ARRAY
    jne .more
    mov rax, [rax+TY_BASE]
    jmp .const_array
.const_member:
    or dword [rbx+TY_FLAGS], TF_CONST_MEMBER
.more:
    mov edi, ','
    call accept
    test rax, rax
    jnz .declarator
    mov edi, ';'
    call expect
    mov edi, '}'
    call accept
    test rax, rax
    jz .fields
    cmp qword [rsp+40], 0
    je aggregate_error
    mov ecx, [rbx+TY_ALIGN]
    dec ecx
    mov rax, [rsp+8]
    add rax, 7
    shr rax, 3
    add rax, rcx
    not rcx
    and rax, rcx
    cmp rax, MAX_MEMORY
    ja aggregate_error
    mov [rbx+TY_BYTES], eax
    and dword [rbx+TY_FLAGS], ~TF_INCOMPLETE
.done:
    mov rax, rbx
    dec qword [aggregate_depth]
    add rsp, 48
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
.duplicate:
    lea rdi, [msg_name]
    jmp fatal
; Shared semantic failure for incomplete or invalid aggregate operations.
aggregate_error:
    lea rdi, [msg_type]
    jmp fatal
section .bss
align 8
aggregate_depth: resq 1 ; Bounded recursive aggregate declaration nesting.
