; Recursive-descent declarations/statements and precedence-climbing expressions.
section .rodata
; Operator spellings map to C precedence; assignment is right associative.
precedence_table:
    dd ',',1, '=',2, T_ADD_EQ,2,T_SUB_EQ,2,T_MUL_EQ,2,T_DIV_EQ,2,T_MOD_EQ,2
    dd T_AND_EQ,2,T_OR_EQ,2,T_XOR_EQ,2,T_SHL_EQ,2,T_SHR_EQ,2
    dd '?',3,T_OR,4,T_AND,5,'|',6,'^',7,'&',8,T_EQ,9,T_NE,9
    dd '<',10,'>',10,T_LE,10,T_GE,10,T_SHL,11,T_SHR,11
    dd '+',12,'-',12,'*',13,'/',13,'%',13,0,0
section .text
; Read the precedence of operator EDI; zero means it ends the expression.
operator_precedence:
    lea rdx, [precedence_table]
.loop:
    mov eax, [rdx]
    test eax, eax
    jz .done
    cmp eax, edi
    je .found
    add rdx, 8
    jmp .loop
.found:
    mov eax, [rdx+4]
.done:
    ret
; Statement/argument consumers require a complete record value immediately.
parse_value_expression:
    sub rsp, 8
    call parse_expression
    mov rdi, rax
    call require_complete_value
    add rsp, 8
    ret
; Parse an expression with minimum precedence EDI and return its typed node.
parse_expression:
    push rbx
    push r12
    push r13
    push r14
    push r15
    mov r12d, edi
    inc dword [parse_depth]
    cmp dword [parse_depth], 256
    ja syntax_error
    call parse_unary
    mov rbx, rax
.loop:
    mov r15, [parse_token]
    mov r13d, [r15+TK_KIND]
    mov edi, r13d
    call operator_precedence
    cmp eax, r12d
    jb .done
    test eax, eax
    jz .done
    mov r14d, eax
    mov edi, r13d
    call accept
    cmp r13d, '?'
    je .conditional
    lea edi, [r14+1]
    cmp r14d, 2
    cmove edi, r14d
    call parse_expression
    mov r14, rax
    mov edi, A_BINARY
    cmp r13d, ','
    jne .assignment
    mov edi, A_COMMA
.assignment:
    cmp r13d, '='
    je .assign_kind
    cmp r13d, T_ADD_EQ
    jb .new
    cmp r13d, T_SHR_EQ
    ja .new
.assign_kind:
    mov edi, A_ASSIGN
.new:
    mov rsi, r15
    call node_new
    mov [rax+N_LEFT], rbx
    mov [rax+N_RIGHT], r14
    mov [rax+N_OP], r13d
    mov r15, rax
    mov rdi, rbx
    call require_complete_value
    mov rdi, r14
    call require_complete_value
    cmp dword [r15+N_KIND], A_COMMA
    je .comma
    mov rdi, rbx
    mov rsi, r14
    mov edx, r13d
    call require_binary_operands
    cmp r13d, '='
    jne .common_type
    mov rdi, r14
    mov rsi, [rbx+N_TYPE]
    call require_implicit_conversion
.common_type:
    mov rdi, [rbx+N_TYPE]
    mov rsi, [r14+N_TYPE]
    call type_common
    cmp r13d, T_SHL
    je .shift_type
    cmp r13d, T_SHR
    je .shift_type
    cmp r13d, T_SHL_EQ
    je .shift_type
    cmp r13d, T_SHR_EQ
    jne .operation_type
.shift_type:
    ; The operand gate already requires integers; shifts promote only the left.
    mov rdi, [rbx+N_TYPE]
    call type_promote
.operation_type:
    mov [r15+N_TYPE], rax
    mov [r15+N_AUX], rax
    cmp dword [rax+TY_KIND], K_STRUCT
    jb .arithmetic_checked
    cmp dword [r15+N_KIND], A_ASSIGN
    jne .bad_type
    cmp r13d, '='
    jne .bad_type
.arithmetic_checked:
    cmp dword [r15+N_KIND], A_ASSIGN
    je .assigned
    cmp dword [r15+N_KIND], A_COMMA
    je .comma
    cmp r13d, T_OR
    je .boolean
    cmp r13d, T_AND
    je .boolean
    cmp r13d, T_EQ
    je .boolean
    cmp r13d, T_NE
    je .boolean
    cmp r13d, T_LE
    je .boolean
    cmp r13d, T_GE
    je .boolean
    cmp r13d, '<'
    je .boolean
    cmp r13d, '>'
    je .boolean
    cmp r13d, '+'
    je .pointer
    cmp r13d, '-'
    jne .merged
.pointer:
    mov rax, [rbx+N_TYPE]
    cmp dword [rax+TY_KIND], K_PTR
    je .left_pointer
    cmp dword [rax+TY_KIND], K_ARRAY
    je .left_pointer
    mov rax, [r14+N_TYPE]
    cmp dword [rax+TY_KIND], K_PTR
    je .right_pointer
    cmp dword [rax+TY_KIND], K_ARRAY
    jne .merged
.right_pointer:
    cmp r13d, '-'
    je .bad_type
    mov rdx, [rbx+N_TYPE]
    cmp dword [rdx+TY_KIND], K_CHAR
    jb .bad_type
    cmp dword [rdx+TY_KIND], K_LONG
    ja .bad_type
    jmp .pointer_type
.left_pointer:
    mov rdx, [r14+N_TYPE]
    cmp dword [rdx+TY_KIND], K_PTR
    je .pointer_difference
    cmp dword [rdx+TY_KIND], K_ARRAY
    je .pointer_difference
    cmp dword [rdx+TY_KIND], K_CHAR
    jb .bad_type
    cmp dword [rdx+TY_KIND], K_LONG
    ja .bad_type
    jmp .pointer_type
.pointer_difference:
    cmp r13d, '-'
    jne .bad_type
    ; Both pointed-to layouts must already exist where subtraction is parsed.
    mov rdi, [rax+TY_BASE]
    push rdi
    sub rsp, 8
    call type_require_complete
    mov rax, [r14+N_TYPE]
    mov rdi, [rax+TY_BASE]
    call type_require_complete
    mov rdx, rax
    add rsp, 8
    pop rax
    mov ecx, [rax+TY_KIND]
    cmp ecx, K_VOID
    je .bad_type
    cmp ecx, K_FUNC
    je .bad_type
    mov rdi, rax
    mov rsi, rdx
    call type_pointee_compatible
    test eax, eax
    jz .bad_type
    lea rax, [type_long]
    mov [r15+N_TYPE], rax
    jmp .merged
.pointer_type:
    push rax
    sub rsp, 8
    mov rdi, [rax+TY_BASE]
    call type_require_complete
    add rsp, 8
    pop rax
    mov rdx, [rax+TY_BASE]
    cmp dword [rdx+TY_KIND], K_VOID
    je .bad_type
    cmp dword [rdx+TY_KIND], K_FUNC
    je .bad_type
    cmp dword [rax+TY_KIND], K_ARRAY
    jne .pointer_ready
    mov rdi, [rax+TY_BASE]
    call type_pointer
.pointer_ready:
    mov [r15+N_TYPE], rax
    jmp .merged
.boolean:
    lea rax, [type_int]
    mov [r15+N_TYPE], rax
    jmp .merged
.comma:
    mov rax, [r14+N_TYPE]
    mov [r15+N_TYPE], rax
    jmp .merged
.assigned:
    mov rdi, rbx
    call require_modifiable
    mov eax, [rbx+N_KIND]
    cmp eax, A_VAR
    je .lvalue
    cmp eax, A_MEMBER
    je .lvalue
    cmp eax, A_DEREF
    jne .bad_type
.lvalue:
    mov rax, [rbx+N_TYPE]
    mov [r15+N_TYPE], rax
.merged:
    mov rbx, r15
    jmp .loop
.conditional:
    mov rdi, rbx
    call require_scalar_value
    mov edi, 1
    call parse_expression
    mov r14, rax
    mov edi, ':'
    call expect
    mov edi, 3
    call parse_expression
    push rax
    sub rsp, 8
    mov edi, A_COND
    mov rsi, r15
    call node_new
    mov r15, rax
    add rsp, 8
    pop rdx
    mov [rax+N_COND], rbx
    mov [rax+N_THEN], r14
    mov [rax+N_ELSE], rdx
    mov rdi, r14
    mov rsi, rdx
    call type_conditional
    mov [r15+N_TYPE], rax
    mov rbx, r15
    jmp .loop
.done:
    mov rax, rbx
    dec dword [parse_depth]
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
.bad_type:
    lea rdi, [msg_type]
    jmp fatal
