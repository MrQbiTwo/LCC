section .text
; Emit one template RDI to the native and text buffers.
emit:
    push rbx
    push r12
    push r13
    mov rbx, rdi
    mov r12d, [rbx]
    mov r13d, [rbx+4]
    lea rdi, [code_buffer]
    lea rsi, [rbx+8]
    mov rdx, r12
    call buffer_append
    cmp dword [assembly_mode], 0
    je .done
    lea rdi, [asm_buffer]
    lea rsi, [rbx+r12+8]
    mov rdx, r13
    call buffer_append
.done:
    pop r13
    pop r12
    pop rbx
    ret
; Load exact scalar bits RSI using the shorter zero-extending form when possible.
emit_value_bits:
    lea rdi, [p_mov64]
    mov edx, 8
    mov eax, esi
    cmp rax, rsi
    jne emit_immediate
    lea rdi, [p_mov32]
    mov edx, 4
    jmp emit_immediate
; Emit template RDI followed by the immediate RSI of width EDX.
emit_immediate:
    push rbx
    push r12
    sub rsp, 24
    mov [rsp], rsi
    mov r12d, edx
    call emit
    lea rdi, [code_buffer]
    mov rsi, rsp
    mov edx, r12d
    call buffer_append
    mov rdi, [rsp]
    call asm_hex
    call asm_newline
    add rsp, 24
    pop r12
    pop rbx
    ret
; Emit a frame address for positive stack-home offset RDI.
emit_frame_address:
    push rbx
    sub rsp, 16
    mov rbx, rdi
    lea rdi, [p_frame_addr]
    call emit
    mov eax, ebx
    neg eax
    mov [rsp], eax
    lea rdi, [code_buffer]
    mov rsi, rsp
    mov edx, 4
    call buffer_append
    mov rdi, rbx
    call asm_unsigned
    lea rsi, [asm_bracket]
    call asm_string
    add rsp, 16
    pop rbx
    ret
; Emit a load using a positive displacement RSI and prefix template RDI.
emit_offset_load:
    push rbx
    sub rsp, 16
    mov rbx, rsi
    call emit
    mov [rsp], ebx
    lea rdi, [code_buffer]
    mov rsi, rsp
    mov edx, 4
    call buffer_append
    mov rdi, rbx
    call asm_unsigned
    lea rsi, [asm_bracket]
    call asm_string
    add rsp, 16
    pop rbx
    ret
