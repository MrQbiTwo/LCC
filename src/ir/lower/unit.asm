; Lower structured control flow to explicit labels and stack-home operations.
section .bss
align 8
return_label: resq 1 ; Common exit label of the current function.
break_label: resq 1 ; Innermost break destination.
continue_label: resq 1 ; Innermost loop continuation destination.
entry_label: resq 1 ; Executable process entry, absent for object-only text.
main_symbol: resq 1 ; Defined main declaration, if present.
section .rodata
start_spelling: db '_start',0
start_token: dd T_IDENT,6
    dq start_spelling
    times TK_SIZE-16 db 0
section .text
; Assign symbols, lower each function, then serialize the selected output mode.
generate_unit:
    push rbx
    push r12
    push r13
    mov qword [emit_section], 1
    lea rsi, [asm_header]
    call asm_string
    call runtime_prepare
    mov rbx, [global_symbols]
.labels:
    test rbx, rbx
    jz .entry
    test dword [rbx+S_FLAGS], SF_TYPEDEF|SF_ENUM
    jnz .next_label
    mov rdi, [rbx+S_NAME]
    test dword [rbx+S_FLAGS], SF_BLOCK_STATIC
    jz .label_name
    xor edi, edi
.label_name:
    call label_new
    mov [rbx+S_LABEL], rax
    test dword [rbx+S_FLAGS], SF_BLOCK_STATIC
    jnz .next_label
    mov rdi, [rbx+S_NAME]
    lea rsi, [kw_main]
    call token_is
    test eax, eax
    jz .declaration
    mov rax, [rbx+S_TYPE]
    cmp dword [rax+TY_KIND], K_FUNC
    jne .declaration
    test dword [rbx+S_FLAGS], SF_DEFINED
    jz .declaration
    mov [main_symbol], rbx
.declaration:
    test dword [rbx+S_FLAGS], SF_RUNTIME
    jnz .next_label
    test dword [rbx+S_FLAGS], SF_STATIC
    jnz .next_label
    lea rsi, [asm_global]
    test dword [rbx+S_FLAGS], SF_DEFINED
    jnz .export
    lea rsi, [asm_extern]
.export:
    call asm_string
    mov rdi, [rbx+S_LABEL]
    call asm_label_name
    call asm_newline
.next_label:
    mov rbx, [rbx+S_NEXT]
    jmp .labels
.entry:
    cmp qword [main_symbol], 0
    jne .startup
    cmp dword [assembly_mode], 0
    jne .functions
    lea rdi, [msg_main]
    jmp fatal
.startup:
    lea rdi, [start_token]
    call label_new
    mov [entry_label], rax
    lea rsi, [asm_global]
    call asm_string
    mov rdi, [entry_label]
    call asm_label_name
    call asm_newline
    mov rdi, [entry_label]
    call label_define
    lea rdi, [i_fp_startup]
    call emit
    lea rdi, [i_argc]
    call emit
    lea rdi, [i_argv]
    call emit
    lea rdi, [p_call]
    mov rax, [main_symbol]
    mov rsi, [rax+S_LABEL]
    call emit_branch
    lea rdi, [i_exit_result]
    call emit
    cmp qword [runtime_active], 0
    je .plain_exit
    lea rdi, [p_call]
    mov rsi, [runtime_exit_label]
    call emit_branch
.plain_exit:
    lea rdi, [i_exit_number]
    call emit
    lea rdi, [i_syscall]
    call emit
.functions:
    mov rbx, [global_symbols]
.function:
    test rbx, rbx
    jz .data
    test dword [rbx+S_FLAGS], SF_TYPEDEF|SF_ENUM
    jnz .next_function
    mov rax, [rbx+S_TYPE]
    cmp dword [rax+TY_KIND], K_FUNC
    jne .next_function
    test dword [rbx+S_FLAGS], SF_DEFINED
    jz .next_function
    mov [current_function], rbx
    mov rdi, [rbx+S_LABEL]
    call label_define
    xor edi, edi
    call label_new
    mov [return_label], rax
    lea rdi, [i_prologue]
    call emit
    lea rdi, [i_frame]
    call emit
    lea rdi, [p_stack_sub]
    mov rsi, [rbx+S_OFFSET]
    mov edx, 4
    call emit_immediate
    call generate_parameters
    mov rdi, [rbx+S_BODY]
    call generate_statement
    lea rdi, [i_zero]
    call emit
    mov rdi, [return_label]
    call label_define
    lea rdi, [i_leave]
    call emit
    lea rdi, [i_ret]
    call emit
.next_function:
    mov rbx, [rbx+S_NEXT]
    jmp .function
.data:
    call runtime_emit
    mov qword [emit_section], 2
    lea rsi, [asm_data]
    call asm_string
    mov rbx, [global_symbols]
.object:
    test rbx, rbx
    jz .serialize
    test dword [rbx+S_FLAGS], SF_TYPEDEF|SF_ENUM
    jnz .next_object
    mov r12, [rbx+S_TYPE]
    cmp dword [r12+TY_KIND], K_FUNC
    je .next_object
    test dword [rbx+S_FLAGS], SF_DEFINED
    jz .next_object
    mov ecx, [r12+TY_ALIGN]
    dec ecx
    mov rax, [data_buffer+BUF_LEN]
    neg rax
    and rax, rcx
    mov rdi, rax
    call emit_data_zeros
    mov rdi, [rbx+S_LABEL]
    call label_define
    mov rdi, [rbx+S_BODY]
    test rdi, rdi
    jz .zero_object
    call generate_global_initializer
    jmp .next_object
.zero_object:
    mov edi, [r12+TY_BYTES]
    call emit_data_zeros
.next_object:
    mov rbx, [rbx+S_NEXT]
    jmp .object
.serialize:
    call generate_string_pool
    lea rsi, [asm_stack]
    call asm_string
    cmp dword [assembly_mode], 0
    jne .done
    call elf_create
.done:
    pop r13
    pop r12
    pop rbx
    ret
