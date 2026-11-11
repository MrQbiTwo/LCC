; Reserved stdarg operations retain types and named-parameter identity in the AST.
section .rodata
va_start_name: db '__lcc_va_start',0
va_arg_name: db '__lcc_va_arg',0
va_end_name: db '__lcc_va_end',0
va_copy_name: db '__lcc_va_copy',0
va_names: dq va_start_name,va_arg_name,va_end_name,va_copy_name
va_field_names: dq va_gp_name,va_fp_name,va_overflow_name,va_save_name
va_gp_name: db 'gp_offset',0
va_fp_name: db 'fp_offset',0
va_overflow_name: db 'overflow_arg_area',0
va_save_name: db 'reg_save_area',0
va_field_offsets: dd 0,4,8,16
section .text
; Recognize a builtin at the current token; return zero for an ordinary identifier.
parse_vararg:
    push rbx
    push r12
    push r13
    xor r12d, r12d
.find:
    lea rax, [va_names]
    mov rsi, [rax+r12*8]
    mov rdi, [parse_token]
    call token_is
    test eax, eax
    jnz .found
    inc r12
    cmp r12, 4
    jb .find
    xor eax, eax
    jmp .done
.found:
    inc r12
    mov edi, A_VARARG
    mov rsi, [parse_token]
    call node_new
    mov rbx, rax
    mov [rbx+N_OP], r12d
    lea rax, [type_void]
    mov [rbx+N_TYPE], rax
    mov edi, T_IDENT
    call expect
    mov edi, '('
    call expect
    mov edi, 2
    call parse_expression
    mov [rbx+N_LEFT], rax
    mov rdi, rax
    mov esi, 1
    call require_va_list
    cmp r12d, 3
    je .close
    mov edi, ','
    call expect
    cmp r12d, 2
    je .arg_type
    mov edi, 2
    call parse_expression
    mov [rbx+N_RIGHT], rax
    cmp r12d, 4
    je .copy
    mov r13, [current_function]
    test r13, r13
    jz aggregate_error
    mov rdx, [r13+S_TYPE]
    test dword [rdx+TY_FLAGS], 1
    jz aggregate_error
    mov rdx, [rdx+TY_PARAMS]
.last:
    test rdx, rdx
    jz aggregate_error
    cmp qword [rdx+S_PARAM_NEXT], 0
    je .last_found
    mov rdx, [rdx+S_PARAM_NEXT]
    jmp .last
.last_found:
    cmp dword [rax+N_KIND], A_VAR
    jne aggregate_error
    mov rax, [rax+N_SYM]
    mov rcx, [rax+S_NAME]
    cmp rcx, [rdx+S_NAME]
    jne aggregate_error
    mov rcx, [rax+S_OFFSET]
    cmp rcx, [rdx+S_OFFSET]
    jne aggregate_error
    mov [rbx+N_SYM], r13
    jmp .close
.copy:
    mov rdi, rax
    xor esi, esi
    call require_va_list
    jmp .close
.arg_type:
    call parse_type_name
    mov r13, rax
    mov rdi, rax
    call type_require_complete
    mov eax, [r13+TY_KIND]
    cmp eax, K_INT
    jb aggregate_error
    cmp eax, K_FLOAT
    je aggregate_error
    cmp eax, K_ARRAY
    je aggregate_error
    cmp eax, K_FUNC
    je aggregate_error
    mov [rbx+N_TYPE], r13
    cmp eax, K_STRUCT
    jb .close
    mov rdi, r13
    call local_temporary
    mov [rbx+N_TEMP], rax
.close:
    mov edi, ')'
    call expect
    mov rax, rbx
.done:
    pop r13
    pop r12
    pop rbx
    ret
; Validate array/pointer va_list layout, requiring writable cursors when ESI is one.
require_va_list:
    push rbx
    push r12
    push r13
    mov r12d, esi
    mov rax, [rdi+N_TYPE]
    cmp dword [rax+TY_KIND], K_PTR
    je .base
    cmp dword [rax+TY_KIND], K_ARRAY
    jne aggregate_error
    cmp qword [rax+TY_COUNT], 1
    jne aggregate_error
.base:
    mov rdi, [rax+TY_BASE]
    call type_require_complete
    cmp dword [rax+TY_KIND], K_STRUCT
    jne aggregate_error
    cmp dword [rax+TY_BYTES], 24
    jne aggregate_error
    test r12d, r12d
    jz .fields
    test dword [rax+TY_FLAGS], TF_CONST|TF_CONST_MEMBER
    jnz aggregate_error
.fields:
    mov rbx, [rax+TY_MEMBERS]
    xor r13d, r13d
.field:
    test rbx, rbx
    jz aggregate_error
    mov rdi, [rbx+MEM_NAME]
    lea rax, [va_field_names]
    mov rsi, [rax+r13*8]
    call token_is
    test eax, eax
    jz aggregate_error
    lea rax, [va_field_offsets]
    mov eax, [rax+r13*4]
    cmp rax, [rbx+MEM_OFFSET]
    jne aggregate_error
    mov rax, [rbx+MEM_TYPE]
    cmp r13, 2
    jae .pointer
    cmp dword [rax+TY_KIND], K_INT
    jne aggregate_error
    test dword [rax+TY_FLAGS], 1
    jz aggregate_error
    jmp .next
.pointer:
    cmp dword [rax+TY_KIND], K_PTR
    jne aggregate_error
.next:
    mov rbx, [rbx+MEM_NEXT]
    inc r13
    cmp r13, 4
    jb .field
    test rbx, rbx
    jnz aggregate_error
    pop r13
    pop r12
    pop rbx
    ret
