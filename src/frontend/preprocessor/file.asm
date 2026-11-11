; Flush ordinary tokens before a directive changes the macro environment.
pp_flush:
    push rbx
    mov rbx, rdi
    mov rdi, [rbx+PP_CHUNK]
    call pp_expand
    mov qword [rbx+PP_CHUNK], 0
    mov qword [rbx+PP_CHUNK_TAIL], 0
    test rax, rax
    jz .done
    mov rcx, [rbx+PP_TAIL]
    test rcx, rcx
    jz .first
    mov [rcx+TK_NEXT], rax
    jmp .tail
.first:
    mov [rbx+PP_HEAD], rax
.tail:
    mov [rbx+PP_TAIL], rdx
.done:
    pop rbx
    ret
; Preprocess one file, retaining state only in this invocation's explicit frame.
pp_file:
    push rbx
    push r12
    push r13
    push r14
    push r15
    sub rsp, PP_SIZE
    mov r12, rdi
    mov rbx, rsp
    mov rdi, rbx
    xor eax, eax
    mov ecx, PP_SIZE/8
    rep stosq
    mov [rbx+PP_FILE], r12
    mov qword [rbx+PP_ACTIVE], 1
    inc qword [pp_include_depth]
    cmp qword [pp_include_depth], 64
    ja pp_failure
    mov edi, 64*24
    call arena_alloc
    mov [rbx+PP_COND], rax
    mov rdi, r12
    call source_normalize
    mov rdi, rax
    call lex_source
    mov r12, rax
.next:
    cmp dword [r12+TK_KIND], 0
    je .end
    mov [diag_token], r12
    cmp dword [r12+TK_KIND], '#'
    jne .ordinary
    test dword [r12+TK_FLAGS], 2
    jz .ordinary
    mov rdi, rbx
    call pp_flush
    mov r13, [r12+TK_NEXT]
    mov r14, r12
    mov r15, r13
.directive_end:
    test r15, r15
    jz pp_failure
    cmp dword [r15+TK_KIND], 0
    je .directive
    test dword [r15+TK_FLAGS], 2
    jnz .directive
    mov r14, r15
    mov r15, [r15+TK_NEXT]
    jmp .directive_end
.directive:
    mov [rbx+PP_NEXT], r15
    mov qword [r14+TK_NEXT], 0
    cmp r13, r15
    je .directive_done
    mov rdi, r13
    mov rsi, rbx
    call pp_directive
.directive_done:
    mov r12, [rbx+PP_NEXT]
    jmp .next
.ordinary:
    cmp qword [rbx+PP_ACTIVE], 0
    je .skip
    mov rdi, r12
    call token_clone
    mov rdx, [rbx+PP_CHUNK_TAIL]
    test rdx, rdx
    jz .chunk_first
    mov [rdx+TK_NEXT], rax
    jmp .chunk_linked
.chunk_first:
    mov [rbx+PP_CHUNK], rax
.chunk_linked:
    mov [rbx+PP_CHUNK_TAIL], rax
.skip:
    mov r12, [r12+TK_NEXT]
    jmp .next
.end:
    cmp qword [rbx+PP_DEPTH], 0
    jne pp_failure
    mov rdi, rbx
    call pp_flush
    mov rdx, [rbx+PP_TAIL]
    test rdx, rdx
    jz .empty
    mov [rdx+TK_NEXT], r12
    mov rax, [rbx+PP_HEAD]
    jmp .done
.empty:
    mov rax, r12
.done:
    dec qword [pp_include_depth]
    add rsp, PP_SIZE
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
; Diagnose unsupported or malformed directives at their original source token.
pp_failure:
    lea rdi, [msg_preprocess]
    jmp fatal
section .text
