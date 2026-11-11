; The fixed printf frame owns independent GP/FP cursors and conversion storage.
%define PF_GP 0
%define PF_FP 40
%define PF_STACK 104
%define PF_GP_INDEX 112
%define PF_FP_INDEX 120
%define PF_FLAGS 128
%define PF_WIDTH 136
%define PF_PREC 144
%define PF_MOD 152
%define PF_VALUE 160
%define PF_SIGN 168
%define PF_TEXT 176
%define PF_LENGTH 184
%define PF_ZEROS 192
%define PF_PAD 200
%define PF_SPECIAL 208
%define PF_COUNT 216
%define PF_BUFFER 224
%define PF_PAD_BUFFER 4640
%define PF_FRAME 4712
section .text
global lcc_rt_printf
; Public SysV variadic entry; every call preserves incoming FP control state.
lcc_rt_printf:
    push rbp
    mov rbp, rsp
    push rbx
    push r12
    push r13
    push r14
    push r15
    sub rsp, PF_FRAME
    mov rbx, rsp
    mov r12, rdi
    mov [rbx+PF_GP], rsi
    mov [rbx+PF_GP+8], rdx
    mov [rbx+PF_GP+16], rcx
    mov [rbx+PF_GP+24], r8
    mov [rbx+PF_GP+32], r9
    movq [rbx+PF_FP], xmm0
    movq [rbx+PF_FP+8], xmm1
    movq [rbx+PF_FP+16], xmm2
    movq [rbx+PF_FP+24], xmm3
    movq [rbx+PF_FP+32], xmm4
    movq [rbx+PF_FP+40], xmm5
    movq [rbx+PF_FP+48], xmm6
    movq [rbx+PF_FP+56], xmm7
    lea rax, [rbp+16]
    mov [rbx+PF_STACK], rax
    mov qword [rbx+PF_GP_INDEX], 0
    mov qword [rbx+PF_FP_INDEX], 0
    mov qword [rbx+PF_COUNT], 0
.next:
    cmp byte [r12], 0
    je .done
    mov rdi, r12
.literal:
    mov al, [r12]
    test al, al
    jz .literal_done
    cmp al, '%'
    je .literal_done
    inc r12
    jmp .literal
.literal_done:
    mov rsi, r12
    sub rsi, rdi
    call lcc_fmt_emit
    test eax, eax
    js .error
    cmp byte [r12], 0
    je .done
    inc r12
    call lcc_fmt_parse
    test eax, eax
    js .error
    call lcc_fmt_convert
    test eax, eax
    js .error
    jmp .next
.done:
    mov rax, [rbx+PF_COUNT]
    jmp .return
.error:
    mov eax, -1
.return:
    add rsp, PF_FRAME
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    pop rbp
    ret

; Fetch a promoted integer/pointer or binary64 value from the saved argument areas.
lcc_fmt_gp:
    mov rcx, [rbx+PF_GP_INDEX]
    cmp rcx, 5
    jae lcc_fmt_stack
    mov rax, [rbx+PF_GP+rcx*8]
    inc qword [rbx+PF_GP_INDEX]
    ret
lcc_fmt_fp:
    mov rcx, [rbx+PF_FP_INDEX]
    cmp rcx, 8
    jae lcc_fmt_stack
    mov rax, [rbx+PF_FP+rcx*8]
    inc qword [rbx+PF_FP_INDEX]
    ret
lcc_fmt_stack:
    mov rcx, [rbx+PF_STACK]
    mov rax, [rcx]
    add qword [rbx+PF_STACK], 8
    ret

; Write one span and keep the public int result bounded. Zero-length writes succeed.
lcc_fmt_emit:
    test rsi, rsi
    jz .ok
    mov rax, [rbx+PF_COUNT]
    add rax, rsi
    jc .error
    cmp rax, 0x7fffffff
    ja .error
    push rsi
    call lcc_rt_write
    pop rsi
    test rax, rax
    js .error
    add [rbx+PF_COUNT], rsi
.ok:
    xor eax, eax
    ret
.error:
    mov eax, -1
    ret

; Emit RSI repetitions of AL using a small fixed buffer.
lcc_fmt_pad:
    push r14
    mov r14, rsi
    lea rdi, [rbx+PF_PAD_BUFFER]
    mov ecx, 64
    rep stosb
.loop:
    test r14, r14
    jz .ok
    mov esi, 64
    cmp r14, rsi
    cmovb rsi, r14
    sub r14, rsi
    lea rdi, [rbx+PF_PAD_BUFFER]
    call lcc_fmt_emit
    test eax, eax
    js .done
    jmp .loop
.ok:
    xor eax, eax
.done:
    pop r14
    ret
