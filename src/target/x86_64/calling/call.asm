; Snapshot arguments at evaluation time, then place stack values before registers.
generate_call:
    push rbx
    push r12
    push r13
    push r14
    push r15
    sub rsp, 64
    mov rbx, rdi
    mov qword [rsp], 0
    mov qword [rsp+8], 0
    mov qword [rsp+16], 0
    mov qword [rsp+40], 16
    mov qword [rsp+48], 0
    mov rdi, [rbx+N_TYPE]
    call abi_shape
    cmp eax, AC_MEMORY
    jne .metadata
    mov qword [rsp], 1
    mov qword [rsp+48], 1
.metadata:
    mov rdi, [rbx+N_VALUE]
    shl rdi, 5
    add rdi, AP_SIZE
    call arena_alloc
    mov r14, rax
    mov rdi, [rbx+N_LEFT]
    call generate_expression
    call emit_push_value
    mov r12, [rbx+N_ARGS]
    mov rax, [rbx+N_AUX]
    mov r13, [rax+TY_PARAMS]
    xor r15d, r15d
.arguments:
    test r12, r12
    jz .placement
    mov rdi, r12
    call generate_expression
    mov rsi, [r12+N_TYPE]
    test r13, r13
    jz .promote
    mov rsi, [r13+S_TYPE]
    mov r13, [r13+S_PARAM_NEXT]
    jmp .converted_type
.promote:
    cmp dword [rsi+TY_KIND], K_FLOAT
    jne .promote_integer
    lea rsi, [type_double]
    jmp .converted_type
.promote_integer:
    mov rdi, rsi
    call type_promote
    mov rsi, rax
.converted_type:
    mov rax, r15
    shl rax, 5
    mov [r14+rax+AP_TYPE], rsi
    mov [rsp+24], rsi
    mov rdi, [r12+N_TYPE]
    call generate_convert
    mov rax, [rsp+24]
    cmp dword [rax+TY_KIND], K_STRUCT
    jae .aggregate_snapshot
    call emit_push_value
    add qword [rsp+40], 16
    jmp .classify
.aggregate_snapshot:
    mov esi, [rax+TY_BYTES]
    add rsi, 15
    and rsi, -16
    add [rsp+40], rsi
    cmp qword [rsp+40], MAX_MEMORY
    ja generation_error
    lea rdi, [p_stack_sub]
    mov edx, 4
    call emit_immediate
    lea rdi, [p_stack_addr_rcx]
    xor esi, esi
    call emit_offset_load
    mov rax, [rsp+24]
    mov edi, [rax+TY_BYTES]
    call emit_copy_count
.classify:
    mov rdi, [rsp+24]
    mov rsi, rsp
    call abi_classify
    mov rcx, r15
    shl rcx, 5
    mov [r14+rcx+AP_KIND], rax
    mov [r14+rcx+AP_LOCATION], rdx
    mov rax, [rsp+40]
    mov [r14+rcx+AP_END], rax
    inc r15
    mov r12, [r12+N_NEXT]
    jmp .arguments
.placement:
    mov rax, [rsp+16]
    add rax, 15
    and rax, -16
    mov [rsp+16], rax
    lea rdi, [p_stack_sub]
    mov rsi, rax
    mov edx, 4
    call emit_immediate
    xor r12d, r12d
.stack_pass:
    cmp r12, r15
    jae .register_start
    call .argument_record
    cmp qword [r13+AP_KIND], 2
    jne .stack_next
    mov rax, [r13+AP_TYPE]
    cmp dword [rax+TY_KIND], K_STRUCT
    jae .stack_aggregate
    lea rdi, [p_stack_load]
    mov rsi, [rsp+32]
    call emit_offset_load
    mov rdi, [r13+AP_LOCATION]
    call emit_stack_store
    mov rax, [r13+AP_TYPE]
    cmp dword [rax+TY_KIND], K_LDOUBLE
    jne .stack_next
    mov rsi, [rsp+32]
    add rsi, 8
    lea rdi, [p_stack_load]
    call emit_offset_load
    mov rdi, [r13+AP_LOCATION]
    add rdi, 8
    call emit_stack_store
    jmp .stack_next
.stack_aggregate:
    lea rdi, [p_stack_addr]
    mov rsi, [rsp+32]
    call emit_offset_load
    lea rdi, [p_stack_addr_rcx]
    mov rsi, [r13+AP_LOCATION]
    call emit_offset_load
    mov rax, [r13+AP_TYPE]
    mov edi, [rax+TY_BYTES]
    call emit_copy_count
.stack_next:
    inc r12
    jmp .stack_pass
.register_start:
    xor r12d, r12d
.register_pass:
    cmp r12, r15
    jae .invoke
    call .argument_record
    mov rax, [r13+AP_KIND]
    cmp eax, 2
    je .register_next
    cmp eax, 3
    je .register_aggregate
    mov rdi, [r13+AP_LOCATION]
    inc edi
    test eax, eax
    jz .register_scalar
    add edi, 8
.register_scalar:
    mov rsi, [rsp+32]
    call abi_place_chunk
    jmp .register_next
.register_aggregate:
    mov edi, [r13+AP_LOCATION]
    mov rsi, [rsp+32]
    call abi_place_chunk
    mov rdi, [r13+AP_LOCATION]
    shr rdi, 32
    mov rsi, [rsp+32]
    add rsi, 8
    call abi_place_chunk
.register_next:
    inc r12
    jmp .register_pass
.invoke:
    cmp qword [rsp+48], 0
    je .callee
    mov rdi, [rbx+N_TEMP]
    call emit_frame_address
    lea rdi, [i_rax_rdi]
    call emit
.callee:
    mov rsi, [rsp+40]
    sub rsi, 16
    add rsi, [rsp+16]
    lea rdi, [p_stack_load]
    call emit_offset_load
    lea rdi, [i_rax_r11]
    call emit
    lea rdi, [p_mov64]
    mov rsi, [rsp+8]
    mov edx, 8
    call emit_immediate
    lea rdi, [i_call_r11]
    call emit
    mov rsi, [rsp+40]
    add rsi, [rsp+16]
    lea rdi, [p_stack_add]
    mov edx, 4
    call emit_immediate
    mov rdi, [rbx+N_TYPE]
    mov eax, [rdi+TY_KIND]
    cmp eax, K_STRUCT
    jae .receive_aggregate
    cmp eax, K_FLOAT
    je .receive_float
    cmp eax, K_DOUBLE
    je .receive_double
    cmp eax, K_LDOUBLE
    je .receive_extended
    call emit_normalize
    jmp .done
.receive_aggregate:
    mov rdi, rbx
    call generate_aggregate_receive
    jmp .done
.receive_float:
    lea rdi, [i_xmm_to_f32]
    jmp .receive
.receive_double:
    lea rdi, [i_xmm_to_f64]
    jmp .receive
.receive_extended:
    lea rdi, [i_ld_receive]
.receive:
    call emit
.done:
    add rsp, 64
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
; Locate argument R12's metadata and its snapshot offset from generated RSP.
.argument_record:
    mov r13, r12
    shl r13, 5
    add r13, r14
    mov rax, [rsp+48]
    sub rax, [r13+AP_END]
    add rax, [rsp+24]
    mov [rsp+40], rax
    ret
