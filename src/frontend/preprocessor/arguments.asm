%define ARG_NAME 0
%define ARG_RAW 8
%define ARG_EXPANDED 16
%define ARG_NEXT 24
%define ARG_DONE 32
%define ARG_SIZE 40
section .bss
align 8
pp_expansion_depth: resq 1 ; Recursive argument expansion depth.
section .text
; Union hide sets RDI and RSI, sharing immutable tails where possible.
hide_union:
    push rbx
    push r12
    push r13
    mov rbx, rdi
    mov r12, rsi
.loop:
    test rbx, rbx
    jz .done
    mov r13, [rbx]
    mov rdi, r12
    mov rsi, r13
    call hide_contains
    test eax, eax
    jnz .next
    mov edi, 16
    call arena_alloc
    mov [rax], r13
    mov [rax+8], r12
    mov r12, rax
.next:
    mov rbx, [rbx+8]
    jmp .loop
.done:
    mov rax, r12
    pop r13
    pop r12
    pop rbx
    ret
; Find an argument record in RDI by parameter spelling RSI.
argument_find:
    push rbx
    push r12
    sub rsp, 8
    mov rbx, rdi
    mov r12, rsi
.loop:
    test rbx, rbx
    jz .done
    mov rdi, [rbx+ARG_NAME]
    mov rsi, r12
    call token_same
    test eax, eax
    jnz .done
    mov rbx, [rbx+ARG_NEXT]
    jmp .loop
.done:
    mov rax, rbx
    add rsp, 8
    pop r12
    pop rbx
    ret
; Substitute a function macro RSI invoked at token RDI and attach its remainder.
macro_function:
    push rbx
    push r12
    push r13
    push r14
    push r15
    sub rsp, 80
    mov r12, rdi
    mov r13, rsi
    mov qword [rsp], 0
    mov qword [rsp+8], 0
    mov qword [rsp+16], 0
    mov qword [rsp+24], 0
    mov qword [rsp+56], 0
    mov qword [rsp+72], 0
    mov rbx, [r12+TK_NEXT]
    mov rbx, [rbx+TK_NEXT]
    mov r14, [r13+M_PARAMS]
    xor r15d, r15d
    test r14, r14
    jnz .argument
    test rbx, rbx
    jz pp_failure
    cmp dword [rbx+TK_KIND], ')'
    jne pp_failure
    jmp .arguments_done
.argument:
    mov edi, ARG_SIZE
    call arena_alloc
    mov [rax+ARG_NAME], r14
    test r15, r15
    jz .argument_first
    mov [r15+ARG_NEXT], rax
    jmp .argument_linked
.argument_first:
    mov [rsp], rax
.argument_linked:
    mov r15, rax
    mov qword [rsp+40], 0
    mov qword [rsp+48], 0
.collect:
    test rbx, rbx
    jz pp_failure
    mov eax, [rbx+TK_KIND]
    test eax, eax
    jz pp_failure
    cmp eax, '('
    je .open
    cmp eax, ')'
    je .close
    cmp eax, ','
    jne .copy
    cmp qword [rsp+40], 0
    je .argument_done
    jmp .copy
.open:
    inc qword [rsp+40]
    jmp .copy
.close:
    cmp qword [rsp+40], 0
    je .argument_done
    dec qword [rsp+40]
.copy:
    mov rdi, rbx
    call token_clone
    mov rdx, [rsp+48]
    test rdx, rdx
    jz .raw_first
    mov [rdx+TK_NEXT], rax
    jmp .raw_linked
.raw_first:
    mov [r15+ARG_RAW], rax
.raw_linked:
    mov [rsp+48], rax
    mov rbx, [rbx+TK_NEXT]
    jmp .collect
.argument_done:
    mov r14, [r14+TK_NEXT]
    test r14, r14
    jz .last_argument
    cmp dword [rbx+TK_KIND], ','
    jne pp_failure
    mov rbx, [rbx+TK_NEXT]
    jmp .argument
.last_argument:
    cmp dword [rbx+TK_KIND], ')'
    jne pp_failure
.arguments_done:
    mov rax, [rbx+TK_NEXT]
    mov [rsp+32], rax
    mov edi, 16
    call arena_alloc
    mov [rax], r13
    mov rdx, [r12+TK_HIDE]
    mov [rax+8], rdx
    mov [rsp+24], rax
    mov r14, [r13+M_BODY]
.body:
    test r14, r14
    jz .finish
    cmp dword [r14+TK_KIND], T_PASTE
    je .paste_marker
    cmp dword [r14+TK_KIND], '#'
    je .stringify
    mov rdi, [rsp]
    mov rsi, r14
    call argument_find
    test rax, rax
    jz .literal
    mov r15, rax
    cmp qword [rsp+56], 0
    jne .raw_argument
    mov rax, [r14+TK_NEXT]
    test rax, rax
    jz .expanded_argument
    cmp dword [rax+TK_KIND], T_PASTE
    je .raw_argument
.expanded_argument:
    cmp qword [r15+ARG_DONE], 0
    jne .expanded_ready
    mov rdi, [r15+ARG_RAW]
    call pp_expand
    mov [r15+ARG_EXPANDED], rax
    mov qword [r15+ARG_DONE], 1
.expanded_ready:
    mov r15, [r15+ARG_EXPANDED]
    jmp .substitute
.raw_argument:
    mov r15, [r15+ARG_RAW]
.substitute:
    test r15, r15
    jz .empty_argument
.substitute_next:
    test r15, r15
    jz .next_body
    mov rdi, r15
    call token_clone
    mov [rsp+64], rax
    mov rdi, [rax+TK_HIDE]
    mov rsi, [rsp+24]
    call hide_union
    mov rdx, [rsp+64]
    mov [rdx+TK_HIDE], rax
    mov rax, rdx
    call .append
    mov r15, [r15+TK_NEXT]
    jmp .substitute_next
.empty_argument:
    ; An empty paste operand is a placemarker, not an emitted token.
    cmp qword [rsp+56], 1
    sete al
    movzx eax, al
    xor eax, 1
    mov [rsp+72], rax
    mov qword [rsp+56], 0
    jmp .next_body
.literal:
    mov rdi, r14
    call token_clone
    mov rdx, [rsp+24]
    mov [rax+TK_HIDE], rdx
    mov rdx, [r12+TK_FILE]
    mov [rax+TK_FILE], rdx
    mov rdx, [r12+TK_LINE]
    mov [rax+TK_LINE], rdx
    call .append
    jmp .next_body
.paste_marker:
    cmp r14, [r13+M_BODY]
    je pp_failure
    cmp qword [r14+TK_NEXT], 0
    je pp_failure
    mov rax, [rsp+72]
    inc rax
    mov [rsp+56], rax
    jmp .next_body
.stringify:
    mov r14, [r14+TK_NEXT]
    test r14, r14
    jz pp_failure
    mov rdi, [rsp]
    mov rsi, r14
    call argument_find
    test rax, rax
    jz pp_failure
    mov rdi, [rax+ARG_RAW]
    mov rsi, r12
    call macro_stringify
    mov rdx, [rsp+24]
    mov [rax+TK_HIDE], rdx
    call .append
.next_body:
    mov r14, [r14+TK_NEXT]
    jmp .body
.append:
    ; This local helper sees its caller frame eight bytes above its own RSP.
    mov qword [rsp+80], 0
    cmp qword [rsp+64], 1
    jne .append_normal
    mov qword [rsp+64], 0
    mov rdi, [rsp+24]
    test rdi, rdi
    jz .append_normal
    push rax
    mov rsi, rax
    call macro_paste
    pop rdx
    mov rdi, [rsp+24]
    mov rdx, [rax+TK_HIDE]
    mov rsi, rax
    mov ecx, TK_SIZE/8
    rep movsq
    ret
.append_normal:
    mov qword [rsp+64], 0
    mov rdx, [rsp+24]
    test rdx, rdx
    jz .append_first
    mov [rdx+TK_NEXT], rax
    jmp .append_tail
.append_first:
    mov [rsp+16], rax
.append_tail:
    mov [rsp+24], rax
    ret
.finish:
    mov rdx, [rsp+16]
    test rdx, rdx
    jz .empty
    mov rax, [rsp+32]
    mov [rdx+TK_NEXT], rax
    mov rax, [rsp+8]
    jmp .done
.empty:
    mov rax, [rsp+32]
.done:
    add rsp, 80
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
section .text
