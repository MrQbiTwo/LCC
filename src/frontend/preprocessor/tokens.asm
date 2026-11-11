; Token-based directives and expansion preserve source origins and hide sets.
section .bss
align 8
macro_list: resq 1 ; Active macro definitions, shared across recursive includes.
pp_include_depth: resq 1 ; Include recursion guard.
pp_token_count: resq 1 ; Total allocated expansion tokens.
section .rodata
pp_define: db 'define',0
pp_undef: db 'undef',0
pp_if: db 'if',0
pp_ifdef: db 'ifdef',0
pp_ifndef: db 'ifndef',0
pp_elif: db 'elif',0
pp_else: db 'else',0
pp_endif: db 'endif',0
pp_include: db 'include',0
pp_error: db 'error',0
pp_line: db 'line',0
pp_pragma: db 'pragma',0
pp_defined: db 'defined',0
msg_preprocess: db 'E013: invalid preprocessing directive or expansion',0
section .text
; Clone a token RDI without sharing its mutable next pointer.
token_clone:
    push rbx
    mov rbx, rdi
    inc qword [pp_token_count]
    cmp qword [pp_token_count], 8000000
    ja pp_failure
    mov edi, TK_SIZE
    call arena_alloc
    mov rdi, rax
    mov rsi, rbx
    mov ecx, TK_SIZE/8
    rep movsq
    mov qword [rax+TK_NEXT], 0
    pop rbx
    ret
; Find a macro by token spelling RDI.
macro_find:
    push rbx
    push r12
    sub rsp, 8
    mov r12, rdi
    mov rbx, [macro_list]
.loop:
    test rbx, rbx
    jz .done
    mov rdi, [rbx+M_NAME]
    mov rsi, r12
    call token_same
    test eax, eax
    jnz .done
    mov rbx, [rbx+M_NEXT]
    jmp .loop
.done:
    mov rax, rbx
    add rsp, 8
    pop r12
    pop rbx
    ret
; Test whether hide-set RDI contains macro identity RSI.
hide_contains:
    xor eax, eax
.loop:
    test rdi, rdi
    jz .done
    cmp [rdi], rsi
    je .yes
    mov rdi, [rdi+8]
    jmp .loop
.yes:
    mov eax, 1
.done:
    ret
