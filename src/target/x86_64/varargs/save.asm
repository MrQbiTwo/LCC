; Preserve an incoming SSE slot only when its index EDI is below AL in R10B.
generate_va_snapshot_fp:
    push rbx
    push r12
    push r13
    mov ebx, edi
    xor edi, edi
    call label_new
    mov r12, rax
    xor edi, edi
    call label_new
    mov r13, rax
    lea rdi, [p_va_al]
    mov rsi, rbx
    mov edx, 1
    call emit_immediate
    lea rdi, [p_jbe]
    mov rsi, r12
    call emit_branch
    lea rax, [abi_from_fp]
    mov rdi, [rax+rbx*8]
    call emit
    lea rdi, [p_jump]
    mov rsi, r13
    call emit_branch
    mov rdi, r12
    call label_define
    lea rdi, [i_zero]
    call emit
    mov rdi, r13
    call label_define
    call emit_push_value
    pop r13
    pop r12
    pop rbx
    ret
; Copy GP snapshots and live vector registers into the persistent ABI save area.
generate_va_save:
    push rbx
    push r12
    push r13
    mov r12, [current_function]
    xor ebx, ebx
.gp:
    lea edi, [rbx+1]
    xor esi, esi
    call abi_load_saved
    call emit_push_value
    mov rdi, [r12+S_VA_AREA]
    lea rax, [rbx*8]
    sub rdi, rax
    call emit_frame_address
    lea rdi, [i_rax_rcx]
    call emit
    call emit_pop_value
    lea rdi, [i_store_64]
    call emit
    inc ebx
    cmp ebx, 6
    jb .gp
    xor edi, edi
    call label_new
    mov r13, rax
    xor ebx, ebx
.fp:
    lea rdi, [p_va_al]
    mov rsi, rbx
    mov edx, 1
    call emit_immediate
    lea rdi, [p_jbe]
    mov rsi, r13
    call emit_branch
    mov rdi, [r12+S_VA_AREA]
    mov rax, rbx
    shl rax, 4
    add rax, 48
    sub rdi, rax
    call emit_frame_address
    lea rax, [va_xmm_stores]
    mov rdi, [rax+rbx*8]
    call emit
    inc ebx
    cmp ebx, 8
    jb .fp
    mov rdi, r13
    call label_define
    pop r13
    pop r12
    pop rbx
    ret
