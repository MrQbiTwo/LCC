; Aggregate value snapshots and result registers share exact byte-copy paths.
section .rodata
prefix p_stack_addr, '    lea rax,[rsp+', 0x48,0x8d,0x84,0x24
prefix p_stack_addr_rcx, '    lea rcx,[rsp+', 0x48,0x8d,0x8c,0x24
prefix p_incoming_addr, '    lea rax,[rbp+', 0x48,0x8d,0x85
instruction i_rax_r10, '    mov r10,rax', 0x49,0x89,0xc2
instruction i_r10_rax, '    mov rax,r10', 0x4c,0x89,0xd0
section .text
; Copy generated source RAX to local-frame home RDI, for RSI exact bytes.
abi_copy_home:
    push rbx
    push r12
    sub rsp, 8
    mov rbx, rdi
    mov r12, rsi
    call emit_push_value
    mov rdi, rbx
    call emit_frame_address
    lea rdi, [i_rax_rcx]
    call emit
    call emit_pop_value
    mov rdi, r12
    call emit_copy_count
    add rsp, 8
    pop r12
    pop rbx
    ret
; Load saved incoming register slot EDI from the prologue snapshot into RAX.
; RSI is extra generated-stack space below the 224-byte snapshot.
abi_load_saved:
    mov eax, 14
    cmp edi, 9
    jb .index
    mov eax, 16
.index:
    sub eax, edi
    shl eax, 4
    add rsi, rax
    lea rdi, [p_stack_load]
    jmp emit_offset_load
; Put the temporary at stack offset RSI in encoded argument register EDI.
abi_place_chunk:
    test edi, edi
    jz .done
    push rbx
    mov ebx, edi
    lea rdi, [p_stack_load]
    call emit_offset_load
    lea rax, [abi_to_arg]
    dec ebx
    cmp ebx, 8
    jb .move
    lea rax, [abi_to_fp]
    sub ebx, 8
.move:
    mov rdi, [rax+rbx*8]
    call emit
    pop rbx
.done:
    ret
