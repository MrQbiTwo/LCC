; System V argument placement uses the shared scalar/aggregate classifier.
section .rodata
abi_to_arg: dq i_rax_rdi,i_rax_rsi,i_rax_rdx,i_rax_rcx,i_rax_r8,i_rax_r9
abi_from_arg: dq i_rdi_rax,i_rsi_rax,i_rdx_rax,i_rcx_rax,i_r8_rax,i_r9_rax
abi_to_fp: dq i_rax_xmm0,i_rax_xmm1,i_rax_xmm2,i_rax_xmm3,i_rax_xmm4,i_rax_xmm5,i_rax_xmm6,i_rax_xmm7
abi_from_fp: dq i_xmm0_rax,i_xmm1_rax,i_xmm2_rax,i_xmm3_rax,i_xmm4_rax,i_xmm5_rax,i_xmm6_rax,i_xmm7_rax
prefix p_incoming_high, '    movzx edx,word [rbp+', 0x0f,0xb7,0x95
section .text
; Preserve incoming registers before reconstructing scalar and aggregate homes.
generate_parameters:
    push rbx
    push r12
    push r13
    push r14
    push r15
    sub rsp, 32
    mov qword [rsp], 0
    mov qword [rsp+8], 0
    mov qword [rsp+16], 0
    mov rax, [current_function]
    cmp qword [rax+S_VA_AREA], 0
    je .signature
    lea rdi, [i_rax_r10]
    call emit
    mov rax, [current_function]
.signature:
    mov rax, [rax+S_TYPE]
    mov r12, [rax+TY_PARAMS]
    xor ebx, ebx
.save_gp:
    lea rax, [abi_from_arg]
    mov rdi, [rax+rbx*8]
    call emit
    call emit_push_value
    inc ebx
    cmp ebx, 6
    jb .save_gp
    xor ebx, ebx
.save_fp:
    mov rax, [current_function]
    cmp qword [rax+S_VA_AREA], 0
    je .normal_fp
    mov edi, ebx
    call generate_va_snapshot_fp
    jmp .fp_saved
.normal_fp:
    lea rax, [abi_from_fp]
    mov rdi, [rax+rbx*8]
    call emit
    call emit_push_value
.fp_saved:
    inc ebx
    cmp ebx, 8
    jb .save_fp
    mov rax, [current_function]
    cmp qword [rax+S_VA_AREA], 0
    je .hidden_result
    call generate_va_save
.hidden_result:
    mov rax, [current_function]
    cmp qword [rax+S_RETURN], 0
    je .parameter
    mov qword [rsp], 1
    mov edi, 1
    xor esi, esi
    call abi_load_saved
    call emit_push_value
    mov rax, [current_function]
    mov rdi, [rax+S_RETURN]
    call emit_frame_address
    lea rdi, [i_rax_rcx]
    call emit
    call emit_pop_value
    lea rdi, [i_store_64]
    call emit
.parameter:
    test r12, r12
    jz .done
    mov r13, [r12+S_TYPE]
    mov rdi, r13
    mov rsi, rsp
    call abi_classify
    mov r14, rdx
    cmp dword [r13+TY_KIND], K_STRUCT
    jae .aggregate
    cmp eax, 2
    je .stack
    lea edi, [rdx+1]
    test eax, eax
    jz .saved
    add edi, 8
.saved:
    xor esi, esi
    call abi_load_saved
    jmp .store
.stack:
    lea rdi, [p_incoming]
    lea rsi, [r14+16]
    call emit_offset_load
    cmp dword [r13+TY_KIND], K_LDOUBLE
    jne .store
    lea rdi, [p_incoming_high]
    lea rsi, [r14+24]
    call emit_offset_load
.store:
    mov rdi, r13
    call emit_normalize
    call emit_push_value
    mov rdi, [r12+S_OFFSET]
    call emit_frame_address
    lea rdi, [i_rax_rcx]
    call emit
    call emit_pop_value
    mov rdi, r13
    call emit_store
    jmp .next
.aggregate:
    cmp eax, 2
    je .aggregate_stack
    lea rdi, [p_stack_sub]
    mov esi, 16
    mov edx, 4
    call emit_immediate
    xor r15d, r15d
.chunk:
    mov edi, r14d
    test edi, edi
    jz .chunk_next
    mov esi, 16
    call abi_load_saved
    mov rdi, r15
    call emit_stack_store
.chunk_next:
    shr r14, 32
    add r15, 8
    cmp r15, 16
    jb .chunk
    lea rdi, [p_stack_addr]
    xor esi, esi
    call emit_offset_load
    mov rdi, [r12+S_OFFSET]
    mov esi, [r13+TY_BYTES]
    call abi_copy_home
    lea rdi, [p_stack_add]
    mov esi, 16
    mov edx, 4
    call emit_immediate
    jmp .next
.aggregate_stack:
    lea rdi, [p_incoming_addr]
    lea rsi, [r14+16]
    call emit_offset_load
    mov rdi, [r12+S_OFFSET]
    mov esi, [r13+TY_BYTES]
    call abi_copy_home
.next:
    mov r12, [r12+S_PARAM_NEXT]
    jmp .parameter
.done:
    mov rax, [current_function]
    mov rdx, [rsp]
    shl rdx, 3
    mov [rax+S_VA_GP], rdx
    mov rdx, [rsp+8]
    shl rdx, 4
    add rdx, 48
    mov [rax+S_VA_FP], rdx
    mov rdx, [rsp+16]
    mov [rax+S_VA_STACK], rdx
    lea rdi, [p_stack_add]
    mov esi, 224
    mov edx, 4
    call emit_immediate
    add rsp, 32
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
