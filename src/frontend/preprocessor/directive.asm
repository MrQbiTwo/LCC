; Dispatch a directive token RDI using explicit file state RSI.
pp_directive:
    push rbx
    push r12
    push r13
    push r14
    push r15
    mov rbx, rsi
    mov r12, rdi
    mov r13, [r12+TK_NEXT]
%macro pp_match 2
    mov rdi, r12
    lea rsi, [%1]
    call token_is
    test eax, eax
    jnz %2
%endmacro
    pp_match pp_if, .if
    pp_match pp_ifdef, .ifdef
    pp_match pp_ifndef, .ifndef
    pp_match pp_elif, .elif
    pp_match pp_else, .else
    pp_match pp_endif, .endif
    cmp qword [rbx+PP_ACTIVE], 0
    je .done
    pp_match pp_define, .define
    pp_match pp_undef, .undef
    pp_match pp_include, .include
    pp_match pp_error, pp_failure
    pp_match pp_line, .line
    pp_match pp_pragma, .pragma
    jmp pp_failure
.define:
    test r13, r13
    jz pp_failure
    cmp dword [r13+TK_KIND], T_IDENT
    jne pp_failure
    mov rdi, r13
    call macro_find
    mov r14, rax
    test rax, rax
    jnz .define_body
    mov edi, M_SIZE
    call arena_alloc
    mov r14, rax
    mov [r14+M_NAME], r13
    mov rax, [macro_list]
    mov [r14+M_NEXT], rax
    mov [macro_list], r14
.define_body:
    mov qword [r14+M_KIND], 0
    mov qword [r14+M_PARAMS], 0
    mov qword [r14+M_COUNT], 0
    mov r15, [r13+TK_NEXT]
    test r15, r15
    jz .object_body
    cmp dword [r15+TK_KIND], '('
    jne .object_body
    test dword [r15+TK_FLAGS], 3
    jnz .object_body
    mov qword [r14+M_KIND], 1
    mov r15, [r15+TK_NEXT]
    test r15, r15
    jz pp_failure
    cmp dword [r15+TK_KIND], ')'
    je .params_done
    xor r13d, r13d
.parameter:
    cmp dword [r15+TK_KIND], T_IDENT
    jne pp_failure
    mov rdi, r15
    call token_clone
    test r13, r13
    jz .param_first
    mov [r13+TK_NEXT], rax
    jmp .param_linked
.param_first:
    mov [r14+M_PARAMS], rax
.param_linked:
    mov r13, rax
    inc qword [r14+M_COUNT]
    cmp qword [r14+M_COUNT], 4096
    ja pp_failure
    mov r15, [r15+TK_NEXT]
    test r15, r15
    jz pp_failure
    cmp dword [r15+TK_KIND], ')'
    je .params_done
    cmp dword [r15+TK_KIND], ','
    jne pp_failure
    mov r15, [r15+TK_NEXT]
    test r15, r15
    jz pp_failure
    jmp .parameter
.params_done:
    mov r15, [r15+TK_NEXT]
.object_body:
    mov [r14+M_BODY], r15
    jmp .done
.undef:
    test r13, r13
    jz pp_failure
    cmp qword [r13+TK_NEXT], 0
    jne pp_failure
    mov rdi, r13
    call macro_find
    test rax, rax
    jz .done
    lea rdx, [macro_list]
.undef_find:
    cmp [rdx], rax
    je .undef_found
    mov rdx, [rdx]
    add rdx, M_NEXT
    jmp .undef_find
.undef_found:
    mov rax, [rax+M_NEXT]
    mov [rdx], rax
    jmp .done
.if:
    xor r14d, r14d
    cmp qword [rbx+PP_ACTIVE], 0
    je .push_condition
    mov rdi, r13
    call pp_evaluate
    mov r14, rax
    jmp .push_condition
.ifdef:
    xor r15d, r15d
    jmp .defined_condition
.ifndef:
    mov r15d, 1
.defined_condition:
    test r13, r13
    jz pp_failure
    cmp dword [r13+TK_KIND], T_IDENT
    jne pp_failure
    cmp qword [r13+TK_NEXT], 0
    jne pp_failure
    mov rdi, r13
    call macro_find
    test rax, rax
    setnz r14b
    movzx r14d, r14b
    xor r14d, r15d
.push_condition:
    mov rax, [rbx+PP_DEPTH]
    cmp rax, 64
    jae pp_failure
    imul rax, 24
    add rax, [rbx+PP_COND]
    mov rdx, [rbx+PP_ACTIVE]
    mov [rax], rdx
    mov [rax+8], r14
    mov qword [rax+16], 0
    and r14, rdx
    mov [rbx+PP_ACTIVE], r14
    inc qword [rbx+PP_DEPTH]
    jmp .done
.elif:
    call .condition_top
    cmp qword [rax+16], 0
    jne pp_failure
    mov r14, rax
    mov qword [rbx+PP_ACTIVE], 0
    cmp qword [r14], 0
    je .done
    cmp qword [r14+8], 0
    jne .done
    mov rdi, r13
    call pp_evaluate
    mov [r14+8], rax
    mov [rbx+PP_ACTIVE], rax
    jmp .done
.else:
    test r13, r13
    jnz pp_failure
    call .condition_top
    cmp qword [rax+16], 0
    jne pp_failure
    mov qword [rax+16], 1
    cmp qword [rax+8], 0
    sete dl
    movzx edx, dl
    and rdx, [rax]
    mov [rbx+PP_ACTIVE], rdx
    mov qword [rax+8], 1
    jmp .done
.endif:
    test r13, r13
    jnz pp_failure
    call .condition_top
    mov rax, [rax]
    mov [rbx+PP_ACTIVE], rax
    dec qword [rbx+PP_DEPTH]
    jmp .done
.condition_top:
    mov rax, [rbx+PP_DEPTH]
    test rax, rax
    jz pp_failure
    dec rax
    imul rax, 24
    add rax, [rbx+PP_COND]
    ret
.include:
    mov rdi, r13
    mov rsi, rbx
    call pp_include_file
    jmp .done
.line:
    mov rdi, r13
    mov rsi, rbx
    call pp_line_directive
    jmp .done
.pragma:
    lea rdi, [pp_pragma_warning]
    call print_error
.done:
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
