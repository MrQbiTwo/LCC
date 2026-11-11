; Set flags from a scalar value of type RDI without misreading negative zero.
generate_truth:
    cmp dword [rdi+TY_KIND], K_STRUCT
    jae aggregate_error
    mov eax, [rdi+TY_KIND]
    cmp eax, K_FLOAT
    je .single
    cmp eax, K_DOUBLE
    je .double
    lea rdi, [i_test]
    jmp emit
.single:
    lea rdi, [i_f32_truth]
    jmp emit
.double:
    lea rdi, [i_f64_truth]
    jmp emit
; Evaluate a condition node RDI and leave ZF set exactly when its value is zero.
generate_test_expression:
    push rbx
    mov rbx, rdi
    call generate_expression
    mov rdi, [rbx+N_TYPE]
    call generate_truth
    pop rbx
    ret
; Select arithmetic or ordered/unordered comparison for common floating type.
generate_fp_binary:
    push rbx
    push r12
    sub rsp, 8
    mov rbx, rdi
    mov r12d, esi
    mov rax, [rbx+N_AUX]
    mov eax, [rax+TY_KIND]
    sub eax, K_FLOAT
    imul eax, 5
    xor ecx, ecx
    cmp r12d, '+'
    je .select
    inc ecx
    cmp r12d, '-'
    je .select
    inc ecx
    cmp r12d, '*'
    je .select
    inc ecx
    cmp r12d, '/'
    je .select
    inc ecx
    cmp r12d, '%'
    je generation_error
.select:
    add eax, ecx
    lea rdx, [fp_operation_table]
    mov rdi, [rdx+rax*8]
    mov ebx, ecx
    call emit
    cmp ebx, 4
    jne .done
    lea rdi, [i_ordered_eq]
    cmp r12d, T_EQ
    je .compare
    lea rdi, [i_unordered_ne]
    cmp r12d, T_NE
    je .compare
    lea rdi, [i_ordered_lt]
    cmp r12d, '<'
    je .compare
    lea rdi, [i_ordered_le]
    cmp r12d, T_LE
    je .compare
    lea rdi, [i_ordered_gt]
    cmp r12d, '>'
    je .compare
    lea rdi, [i_ordered_ge]
    cmp r12d, T_GE
    jne generation_error
.compare:
    call emit
.done:
    add rsp, 8
    pop r12
    pop rbx
    ret

