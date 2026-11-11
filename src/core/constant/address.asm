; Address-only lvalue traversal and scaled symbolic pointer arithmetic.
section .text
; RDI lvalue -> RAX byte addend/address, R8 label or zero, RDX zero.
static_lvalue:
    push rbx
    mov rbx, rdi
    mov rax, [rbx+N_TOKEN]
    mov [diag_token], rax
    mov eax, [rbx+N_KIND]
    cmp eax, A_VAR
    je .variable
    cmp eax, A_STRING
    je .string
    cmp eax, A_DEREF
    je .indirect
    cmp eax, A_MEMBER
    jne generation_error
    mov rax, [rbx+N_SYM]
    cmp dword [rax+MEM_BITS], 0
    jne generation_error
    mov rdi, [rbx+N_LEFT]
    cmp dword [rbx+N_OP], T_ARROW
    je .member_pointer
    call static_lvalue
    jmp .member_offset
.member_pointer:
    call evaluate_static_constant
.member_offset:
    test r8, r8
    jz .numeric_member
    add rax, [rbx+N_VALUE]
    jo static_range
    jmp .done
.numeric_member:
    add rax, [rbx+N_VALUE]
    jc static_range
    jmp .done
.indirect:
    mov rdi, [rbx+N_LEFT]
    call evaluate_static_constant
    jmp .done
.variable:
    mov rdi, [rbx+N_SYM]
    call symbol_entity
    test dword [rax+S_FLAGS], SF_LOCAL|SF_ENUM|SF_TYPEDEF
    jnz generation_error
    mov r8, [rax+S_LABEL]
    test r8, r8
    jz generation_error
    xor eax, eax
    jmp .done
.string:
    call string_label
    mov r8, rax
    xor eax, eax
.done:
    xor edx, edx
    pop rbx
    ret
; Evaluate +/- with at most one symbolic base. Pointer offsets use element size.
static_binary:
    push rbx
    push r12
    push r13
    push r14
    push r15
    sub rsp, 16
    mov rbx, rdi
    mov rax, [rbx+N_AUX]
    mov eax, [rax+TY_KIND]
    cmp eax, K_FLOAT
    jb .operands
    cmp eax, K_LDOUBLE
    jbe .numeric
.operands:
    mov rdi, [rbx+N_LEFT]
    call evaluate_static_constant
    mov r12, rax                 ; Left value or addend.
    mov r13, r8                  ; Left label, if present.
    mov rdi, [rbx+N_RIGHT]
    call evaluate_static_constant
    mov r14, rax                 ; Right value or addend.
    mov r15, r8                  ; Right label, if present.
    test r13, r13
    jz .types
    test r15, r15
    jnz generation_error
.types:
    mov rax, [rbx+N_LEFT]
    mov rax, [rax+N_TYPE]
    mov rdx, [rbx+N_RIGHT]
    mov rdx, [rdx+N_TYPE]
    cmp dword [rax+TY_KIND], K_PTR
    je .left_pointer
    cmp dword [rax+TY_KIND], K_ARRAY
    je .left_pointer
    cmp dword [rdx+TY_KIND], K_PTR
    je .right_pointer
    cmp dword [rdx+TY_KIND], K_ARRAY
    je .right_pointer
    mov rcx, r13
    or rcx, r15
    jz .integer_value
    ; A retained integer address is an explicit target extension, byte scaled.
    mov qword [rsp], 1
    test r13, r13
    jnz .integer_left
    cmp dword [rbx+N_OP], '-'
    je generation_error
    xchg r12, r14
    xchg r13, r15
    mov rdi, rax
    jmp .offset
.integer_left:
    mov rdi, rdx
    jmp .offset
.right_pointer:
    cmp dword [rbx+N_OP], '-'
    je generation_error
    xchg r12, r14
    xchg r13, r15
    xchg rax, rdx
.left_pointer:
    cmp dword [rdx+TY_KIND], K_CHAR
    jb generation_error
    cmp dword [rdx+TY_KIND], K_LONG
    ja generation_error
    test r15, r15
    jnz generation_error
    mov [rsp+8], rdx
    mov rdi, [rax+TY_BASE]
    call type_require_complete
    cmp dword [rax+TY_KIND], K_VOID
    je generation_error
    cmp dword [rax+TY_KIND], K_FUNC
    je generation_error
    mov eax, [rax+TY_BYTES]
    mov [rsp], rax
    mov rdi, [rsp+8]
.offset:
    mov rax, r14
    call static_offset
    imul rax, [rsp]
    jo static_range
    cmp dword [rbx+N_OP], '-'
    jne .add
    neg rax
    jo static_range
.add:
    test r13, r13
    jz .numeric_address
    add rax, r12
    jo static_range
    mov r8, r13
    xor edx, edx
    jmp .done
.numeric_address:
    test rax, rax
    js .subtract_address
    add rax, r12
    jc static_range
    jmp .address_done
.subtract_address:
    add rax, r12
    jnc static_range
.address_done:
    xor r8d, r8d
    xor edx, edx
    jmp .done
.integer_value:
    mov rdi, rbx
    mov rsi, r12
    mov rdx, r14
    call constant_integer_binary
    xor r8d, r8d
    jmp .done
.numeric:
    mov rdi, rbx
    call static_numeric_expression
    xor r8d, r8d
.done:
    add rsp, 16
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
