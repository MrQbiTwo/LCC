; Parse one statement, including a scoped compound statement.
parse_statement:
    push rbx
    push r12
    push r13
    push r14
    push r15
    sub rsp, 16
    inc qword [statement_depth]
    cmp qword [statement_depth], 256
    ja syntax_error
    mov r12, [parse_token]
    mov edi, '{'
    call accept
    test rax, rax
    jnz .block
    lea rdi, [kw_return]
    call accept_keyword
    test rax, rax
    jnz .return
    lea rdi, [kw_if]
    call accept_keyword
    test rax, rax
    jnz .if
    lea rdi, [kw_while]
    call accept_keyword
    test rax, rax
    jnz .while
    lea rdi, [kw_for]
    call accept_keyword
    test rax, rax
    jnz .for
    lea rdi, [kw_do]
    call accept_keyword
    test rax, rax
    jnz .do
    lea rdi, [kw_break]
    call accept_keyword
    test rax, rax
    jnz .break
    lea rdi, [kw_continue]
    call accept_keyword
    test rax, rax
    jnz .continue
    lea rdi, [kw_switch]
    call accept_keyword
    test rax, rax
    jnz .switch
    lea rdi, [kw_case]
    call accept_keyword
    test rax, rax
    jnz .case
    lea rdi, [kw_default]
    call accept_keyword
    test rax, rax
    jnz .default
    lea rdi, [kw_goto]
    call accept_keyword
    test rax, rax
    jnz .goto
    cmp dword [r12+TK_KIND], T_IDENT
    jne .ordinary
    mov rax, [r12+TK_NEXT]
    test rax, rax
    jz .ordinary
    cmp dword [rax+TK_KIND], ':'
    je .label
.ordinary:
    call is_type_token
    test eax, eax
    jnz .declaration
    mov edi, A_EXPR
    mov rsi, r12
    call node_new
    mov rbx, rax
    mov edi, ';'
    call accept
    test rax, rax
    jnz .finished
    mov edi, 1
    call parse_value_expression
    mov [rbx+N_LEFT], rax
    mov edi, ';'
    call expect
    jmp .finished
.block:
    mov edi, A_BLOCK
    mov rsi, r12
    call node_new
    mov rbx, rax
    mov r14, [local_symbols]
    mov rax, [tag_symbols]
    mov [rsp+8], rax
    inc dword [scope_depth]
    xor r13d, r13d
.block_loop:
    mov edi, '}'
    call accept
    test rax, rax
    jnz .block_end
    mov rax, [parse_token]
    cmp dword [rax+TK_KIND], 0
    je syntax_error
    call parse_statement
    test r13, r13
    jz .first_statement
    mov [r13+N_NEXT], rax
    jmp .linked_statement
.first_statement:
    mov [rbx+N_LEFT], rax
.linked_statement:
    mov r13, rax
    jmp .block_loop
.block_end:
    mov [local_symbols], r14
    mov rax, [rsp+8]
    mov [tag_symbols], rax
    dec dword [scope_depth]
    jmp .finished
.return:
    mov edi, A_RETURN
    mov rsi, r12
    call node_new
    mov rbx, rax
    mov edi, ';'
    call accept
    test rax, rax
    jnz .finished
    mov edi, 1
    call parse_value_expression
    mov [rbx+N_LEFT], rax
    mov rdi, rax
    mov rsi, [current_function]
    mov rsi, [rsi+S_TYPE]
    mov rsi, [rsi+TY_BASE]
    call require_implicit_conversion
    mov edi, ';'
    call expect
    jmp .finished
.if:
    mov edi, A_IF
    mov rsi, r12
    call node_new
    mov rbx, rax
    call parse_parenthesized
    mov [rbx+N_COND], rax
    call parse_statement
    mov [rbx+N_THEN], rax
    lea rdi, [kw_else]
    call accept_keyword
    test rax, rax
    jz .finished
    call parse_statement
    mov [rbx+N_ELSE], rax
    jmp .finished
.while:
    mov edi, A_FOR
    mov rsi, r12
    call node_new
    mov rbx, rax
    call parse_parenthesized
    mov [rbx+N_COND], rax
    jmp .loop_body
.for:
    mov edi, A_FOR
    mov rsi, r12
    call node_new
    mov rbx, rax
    mov edi, '('
    call expect
    mov edi, ';'
    call accept
    test rax, rax
    jnz .for_condition
    mov edi, 1
    call parse_value_expression
    mov [rbx+N_LEFT], rax
    mov edi, ';'
    call expect
.for_condition:
    mov edi, ';'
    call accept
    test rax, rax
    jnz .for_step
    mov edi, 1
    call parse_value_expression
    mov rdi, rax
    call require_scalar_value
    mov [rbx+N_COND], rax
    mov edi, ';'
    call expect
.for_step:
    mov edi, ')'
    call accept
    test rax, rax
    jnz .loop_body
    mov edi, 1
    call parse_value_expression
    mov [rbx+N_RIGHT], rax
    mov edi, ')'
    call expect
.loop_body:
    inc qword [parse_loops]
    call parse_statement
    dec qword [parse_loops]
    mov [rbx+N_THEN], rax
    jmp .finished
.do:
    mov edi, A_DO
    mov rsi, r12
    call node_new
    mov rbx, rax
    inc qword [parse_loops]
    call parse_statement
    dec qword [parse_loops]
    mov [rbx+N_THEN], rax
    lea rdi, [kw_while]
    call accept_keyword
    test rax, rax
    jz syntax_error
    call parse_parenthesized
    mov [rbx+N_COND], rax
    mov edi, ';'
    call expect
    jmp .finished
.break:
    mov edi, A_BREAK
    cmp qword [parse_switch], 0
    jne .jump_node
    jmp .jump
.continue:
    mov edi, A_CONTINUE
.jump:
    cmp qword [parse_loops], 0
    je syntax_error
.jump_node:
    mov rsi, r12
    call node_new
    mov rbx, rax
    mov edi, ';'
    call expect
    jmp .finished
.switch:
    mov edi, A_SWITCH
    mov rsi, r12
    call node_new
    mov rbx, rax
    call parse_parenthesized
    mov [rbx+N_COND], rax
    mov rdi, [rax+N_TYPE]
    cmp dword [rdi+TY_KIND], K_CHAR
    jb .bad_type
    cmp dword [rdi+TY_KIND], K_LONG
    ja .bad_type
    lea rsi, [type_int]
    call type_common
    mov [rbx+N_TYPE], rax
    mov r13, [parse_switch]
    mov [parse_switch], rbx
    call parse_statement
    mov [rbx+N_THEN], rax
    mov [parse_switch], r13
    jmp .finished
.case:
    mov r13, [parse_switch]
    test r13, r13
    jz syntax_error
    mov edi, 3
    call parse_value_expression
    mov r14, rax
    mov rax, [rax+N_TYPE]
    cmp dword [rax+TY_KIND], K_CHAR
    jb .bad_type
    cmp dword [rax+TY_KIND], K_LONG
    ja .bad_type
    mov rdi, r14
    call evaluate_required_integer_constant
    mov rdi, [r14+N_TYPE]
    mov rsi, [r13+N_TYPE]
    call constant_convert
    mov r14, rax
    mov rdx, [r13+N_ARGS]
.case_duplicate:
    test rdx, rdx
    jz .new_case
    cmp r14, [rdx+N_VALUE]
    je .duplicate
    mov rdx, [rdx+N_ARGS]
    jmp .case_duplicate
.new_case:
    mov edi, A_CASE
    mov rsi, r12
    call node_new
    mov rbx, rax
    mov [rbx+N_VALUE], r14
    mov rax, [r13+N_ARGS]
    mov [rbx+N_ARGS], rax
    mov [r13+N_ARGS], rbx
    jmp .case_body
.default:
    mov r13, [parse_switch]
    test r13, r13
    jz syntax_error
    cmp qword [r13+N_ELSE], 0
    jne .duplicate
    mov edi, A_CASE
    mov rsi, r12
    call node_new
    mov rbx, rax
    mov [r13+N_ELSE], rbx
.case_body:
    xor edi, edi
    call label_new
    mov [rbx+N_SYM], rax
    mov edi, ':'
    call expect
    call parse_statement
    mov [rbx+N_LEFT], rax
    jmp .finished
.goto:
    mov edi, T_IDENT
    call expect
    mov rdi, rax
    call parse_label_find
    mov r13, rax
    mov edi, A_GOTO
    mov rsi, r12
    call node_new
    mov rbx, rax
    mov rax, [r13+16]
    mov [rbx+N_SYM], rax
    mov edi, ';'
    call expect
    jmp .finished
.label:
    mov rdi, r12
    call parse_label_find
    cmp qword [rax+24], 0
    jne .duplicate
    mov qword [rax+24], 1
    mov r13, rax
    mov edi, T_IDENT
    call expect
    mov edi, ':'
    call expect
    mov edi, A_LABEL
    mov rsi, r12
    call node_new
    mov rbx, rax
    mov rax, [r13+16]
    mov [rbx+N_SYM], rax
    call parse_statement
    mov [rbx+N_LEFT], rax
    jmp .finished
.declaration:
    call parse_local_declaration
    mov rbx, rax
.finished:
    dec qword [statement_depth]
    mov rax, rbx
    add rsp, 16
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
.duplicate:
    mov [diag_token], r12
    lea rdi, [msg_name]
    jmp fatal
.bad_type:
    mov [diag_token], r12
    lea rdi, [msg_type]
    jmp fatal
; Parse a parenthesized condition and return its expression node.
parse_parenthesized:
    push rbx
    mov edi, '('
    call expect
    mov edi, 1
    call parse_value_expression
    mov rdi, rax
    call require_scalar_value
    mov rbx, rax
    mov edi, ')'
    call expect
    mov rax, rbx
    pop rbx
    ret
section .text
