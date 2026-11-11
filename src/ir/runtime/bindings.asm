; Link referenced standard I/O declarations only when this unit owns main.
section .rodata
runtime_printf_name: db 'printf',0
runtime_scanf_name: db 'scanf',0
runtime_getchar_name: db 'getchar',0
runtime_names: dq runtime_printf_name,runtime_scanf_name,runtime_getchar_name
runtime_public_offsets: dq runtime_printf_offset,runtime_scanf_offset,runtime_getchar_offset
section .bss
align 8
runtime_active: resq 1 ; Nonzero requires code/data and a flushing exit.
runtime_symbols: resq 3 ; Canonical referenced public function declarations.
runtime_bases: resq 2 ; Anonymous code/data anchor labels.
runtime_offsets: resq 2 ; Anchor positions within the generated buffers.
runtime_exit_label: resq 1 ; Private startup exit target.
section .text
; Identify runtime-owned declarations before ordinary label/export generation.
runtime_prepare:
    push rbx
    push r12
    sub rsp, 8
    mov rbx, [global_symbols]
.main:
    test rbx, rbx
    jz .done
    test dword [rbx+S_FLAGS], SF_DEFINED
    jz .next_main
    test dword [rbx+S_FLAGS], SF_BLOCK_STATIC|SF_TYPEDEF|SF_ENUM
    jnz .next_main
    mov rax, [rbx+S_TYPE]
    cmp dword [rax+TY_KIND], K_FUNC
    jne .next_main
    mov rdi, [rbx+S_NAME]
    lea rsi, [kw_main]
    call token_is
    test eax, eax
    jnz .find
.next_main:
    mov rbx, [rbx+S_NEXT]
    jmp .main
.find:
    mov rbx, [global_symbols]
.symbol:
    test rbx, rbx
    jz .anchors
    test dword [rbx+S_FLAGS], SF_DEFINED|SF_STATIC|SF_TYPEDEF|SF_ENUM
    jnz .next
    test dword [rbx+S_FLAGS], SF_REFERENCED
    jz .next
    mov rax, [rbx+S_TYPE]
    cmp dword [rax+TY_KIND], K_FUNC
    jne .next
    xor r12d, r12d
.name:
    mov rdi, [rbx+S_NAME]
    lea rax, [runtime_names]
    mov rsi, [rax+r12*8]
    call token_is
    test eax, eax
    jnz .bind
    inc r12
    cmp r12, 3
    jb .name
    jmp .next
.bind:
    lea rax, [runtime_symbols]
    mov [rax+r12*8], rbx
    or dword [rbx+S_FLAGS], SF_RUNTIME
    mov qword [runtime_active], 1
.next:
    mov rbx, [rbx+S_NEXT]
    jmp .symbol
.anchors:
    cmp qword [runtime_active], 0
    je .done
    xor edi, edi
    call label_new
    mov [runtime_bases], rax
    xor edi, edi
    call label_new
    mov [runtime_bases+8], rax
    xor edi, edi
    call label_new
    mov [runtime_exit_label], rax
.done:
    add rsp, 8
    pop r12
    pop rbx
    ret

; Define label RDI as an alias at runtime-code offset RSI in both output modes.
runtime_alias:
    push rbx
    push r12
    sub rsp, 8
    mov rbx, rdi
    mov r12, rsi
    mov rax, [runtime_offsets]
    add rax, rsi
    mov [rdi+L_POS], rax
    mov qword [rdi+L_SECTION], 1
    call asm_label_name
    lea rsi, [runtime_equ]
    call asm_string
    mov rdi, [runtime_bases]
    call asm_label_name
    lea rsi, [asm_address_plus]
    call asm_string
    mov rdi, r12
    call asm_unsigned
    call asm_newline
    add rsp, 8
    pop r12
    pop rbx
    ret
