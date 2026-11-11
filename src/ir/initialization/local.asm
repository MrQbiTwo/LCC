; Materialize the shared ordered initializer representation in code or data.
section .rodata
instruction i_zero_setup, `    mov rdi,rax\n    xor eax,eax`, 0x48,0x89,0xc7,0x31,0xc0
instruction i_zero_bytes, '    rep stosb', 0xf3,0xaa
section .text
; Initialize local symbol RDI using initializer root RSI.
generate_local_initializer:
    push rbx
    push r12
    push r13
    push r14
    push r15
    mov rbx, rdi
    mov r12, rsi
    mov r13, [r12+N_TYPE]
    cmp dword [r13+TY_KIND], K_ARRAY
    jb .leaves
    ; Only clear omitted regions: explicit volatile fields get one store.
    xor r14d, r14d
    mov r15, [r12+N_ARGS]
.gap:
    test r15, r15
    jz .tail_gap
    mov rsi, [r15+N_VALUE]
    sub rsi, r14
    jnc .clear_gap
    ; Consecutive fields can share the final byte of the previous leaf.
    cmp rsi, -1
    jne generation_error
    cmp qword [r15+N_SYM], 0
    je generation_error
    xor esi, esi
.clear_gap:
    mov rdi, [rbx+S_OFFSET]
    sub rdi, r14
    call generate_initializer_zeros
    mov r14, [r15+N_VALUE]
    mov rax, [r15+N_SYM]
    test rax, rax
    jz .ordinary_gap_size
    mov ecx, [rax+MEM_SHIFT]
    add ecx, [rax+MEM_BITS]
    add ecx, 7
    shr ecx, 3
    mov eax, ecx
    jmp .gap_size
.ordinary_gap_size:
    mov rax, [r15+N_AUX]
    test rax, rax
    jnz .gap_size
    mov rax, [r15+N_TYPE]
    mov eax, [rax+TY_BYTES]
.gap_size:
    add r14, rax
    mov r15, [r15+N_NEXT]
    jmp .gap
.tail_gap:
    mov esi, [r13+TY_BYTES]
    sub rsi, r14
    jc generation_error
    mov rdi, [rbx+S_OFFSET]
    sub rdi, r14
    call generate_initializer_zeros
.leaves:
    mov r12, [r12+N_ARGS]
    xor r14d, r14d
.leaf:
    test r12, r12
    jz .done
    mov rdi, [rbx+S_OFFSET]
    sub rdi, [r12+N_VALUE]
    call emit_frame_address
    call emit_push_value
    mov rdi, [r12+N_LEFT]
    call generate_expression
    cmp qword [r12+N_AUX], 0
    jne .string
    mov rax, [r12+N_LEFT]
    mov rdi, [rax+N_TYPE]
    mov rsi, [r12+N_TYPE]
    call generate_convert
    call emit_pop_rcx
    mov rdi, [r12+N_SYM]
    test rdi, rdi
    jnz .bitfield
    mov rdi, [r12+N_TYPE]
    call emit_store
    jmp .next
.bitfield:
    ; The first store to each byte initializes omitted bits too. A later
    ; field sharing that byte retains only already initialized low bits.
    xor esi, esi
    cmp [r12+N_VALUE], r14
    jae .bitfield_store
    mov ecx, [rdi+MEM_SHIFT]
    mov esi, 1
    shl rsi, cl
    dec rsi
.bitfield_store:
    call emit_bitfield_initializer
    jmp .next
.string:
    call emit_pop_rcx
    mov rdi, [r12+N_AUX]
    call emit_copy_count
.next:
    mov r14, [r12+N_VALUE]
    mov rax, [r12+N_SYM]
    test rax, rax
    jz .ordinary_size
    mov ecx, [rax+MEM_SHIFT]
    add ecx, [rax+MEM_BITS]
    add ecx, 7
    shr ecx, 3
    mov eax, ecx
    jmp .leaf_size
.ordinary_size:
    mov rax, [r12+N_AUX]
    test rax, rax
    jnz .leaf_size
    mov rax, [r12+N_TYPE]
    mov eax, [rax+TY_BYTES]
.leaf_size:
    add r14, rax
    mov r12, [r12+N_NEXT]
    jmp .leaf
.done:
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
; Clear RSI omitted bytes beginning at local-frame displacement RDI.
generate_initializer_zeros:
    test rsi, rsi
    jz .done
    push rbx
    mov rbx, rsi
    call emit_frame_address
    lea rdi, [i_zero_setup]
    call emit
    lea rdi, [p_mov_rcx]
    mov rsi, rbx
    mov edx, 8
    call emit_immediate
    lea rdi, [i_zero_bytes]
    call emit
    pop rbx
.done:
    ret
