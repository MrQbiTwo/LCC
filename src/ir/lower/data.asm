; Append RDI zero bytes to data and emit the corresponding NASM reservation.
emit_data_zeros:
    push rbx
    mov rbx, rdi
    test rbx, rbx
    jz .done
    call arena_alloc
    lea rdi, [data_buffer]
    mov rsi, rax
    mov rdx, rbx
    call buffer_append
    lea rsi, [asm_zero]
    call asm_string
    mov rdi, rbx
    call asm_unsigned
    lea rsi, [asm_zero_tail]
    call asm_string
.done:
    pop rbx
    ret
; Append scalar RSI with width EDI and its exact NASM data declaration.
emit_data_integer:
    push rbx
    sub rsp, 16
    mov ebx, edi
    mov [rsp], rsi
    cmp ebx, 8
    ja generation_error
    lea rdi, [data_buffer]
    mov rsi, rsp
    mov edx, ebx
    call buffer_append
    lea rsi, [asm_db]
    cmp ebx, 1
    je .text
    lea rsi, [asm_dw]
    cmp ebx, 2
    je .text
    lea rsi, [asm_dd]
    cmp ebx, 4
    je .text
    lea rsi, [asm_dq]
.text:
    call asm_string
    mov rdi, [rsp]
    call asm_hex
    call asm_newline
    add rsp, 16
    pop rbx
    ret
section .bss
align 8
string_pool: resq 1 ; Immutable literal payload records: next, label, bytes, length.
section .text
; Intern a literal node RDI into the output pool and emit its address.
generate_string:
    sub rsp, 8
    call string_label
    mov rdi, rax
    call emit_label_address
    add rsp, 8
    ret
; Return a stable pooled label for literal node RDI without emitting code.
string_label:
    push rbx
    mov rbx, rdi
    cmp qword [rbx+N_SYM], 0
    jne .address
    xor edi, edi
    call label_new
    mov [rbx+N_SYM], rax
    mov edi, 32
    call arena_alloc
    mov rdx, [string_pool]
    mov [rax], rdx
    mov [string_pool], rax
    mov rdx, [rbx+N_SYM]
    mov [rax+8], rdx
    mov rdx, [rbx+N_VALUE]
    mov [rax+16], rdx
    mov rdx, [rbx+N_AUX]
    mov [rax+24], rdx
.address:
    mov rax, [rbx+N_SYM]
    pop rbx
    ret
; Emit literal bytes after their labels have been assigned during instruction selection.
generate_string_pool:
    push rbx
    push r12
    push r13
    mov rbx, [string_pool]
.next:
    test rbx, rbx
    jz .done
    mov rdi, [rbx+8]
    call label_define
    mov r12, [rbx+16]
    mov r13, [rbx+24]
.byte:
    test r13, r13
    jz .record
    movzx esi, byte [r12]
    mov edi, 1
    call emit_data_integer
    inc r12
    dec r13
    jmp .byte
.record:
    mov rbx, [rbx]
    jmp .next
.done:
    pop r13
    pop r12
    pop rbx
    ret
