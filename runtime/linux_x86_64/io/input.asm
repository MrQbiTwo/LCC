; A single buffered input cursor is shared by scanf and getchar.
section .bss
align 8
lcc_rt_input_pos: resq 1
lcc_rt_input_end: resq 1
lcc_rt_input_eof: resq 1
lcc_rt_input_bytes: resb 4096

section .text
global lcc_rt_getchar

; Peek an unsigned byte, or -1 at EOF/error. Flush before a blocking read.
lcc_rt_input_peek:
    mov rcx, [rel lcc_rt_input_pos]
    cmp rcx, [rel lcc_rt_input_end]
    jb .buffered
    cmp qword [rel lcc_rt_input_eof], 0
    jne .eof
    sub rsp, 8
    call lcc_rt_flush
.read:
    xor eax, eax
    xor edi, edi
    lea rsi, [rel lcc_rt_input_bytes]
    mov edx, 4096
    syscall
    cmp rax, -4
    je .read
    add rsp, 8
    test rax, rax
    jle .ended
    mov [rel lcc_rt_input_end], rax
    mov qword [rel lcc_rt_input_pos], 0
    xor ecx, ecx
.buffered:
    lea rdx, [rel lcc_rt_input_bytes]
    movzx eax, byte [rdx+rcx]
    ret
.ended:
    mov qword [rel lcc_rt_input_eof], 1
.eof:
    mov eax, -1
    ret

; Consume only a successful peek; no refill occurs in this helper.
lcc_rt_input_consume:
    mov rcx, [rel lcc_rt_input_pos]
    cmp rcx, [rel lcc_rt_input_end]
    jae .done
    inc qword [rel lcc_rt_input_pos]
.done:
    ret

; Return the next unsigned byte as int, or EOF (-1). Publish pending prompts
; even when the input is already buffered or an earlier read established EOF.
lcc_rt_getchar:
    sub rsp, 8
    call lcc_rt_flush
    call lcc_rt_input_peek
    test eax, eax
    js .done
    inc qword [rel lcc_rt_input_pos]
.done:
    add rsp, 8
    ret

; C-locale whitespace test, EDI byte -> EAX boolean.
lcc_rt_input_space:
    xor eax, eax
    cmp edi, 32
    je .yes
    sub edi, 9
    cmp edi, 4
    ja .return
.yes:
    inc eax
.return:
    ret

; Discard whitespace and return the first other byte without consuming it.
lcc_rt_input_skip_space:
    sub rsp, 8
.next:
    call lcc_rt_input_peek
    test eax, eax
    js .return
    cmp eax, 32
    je .consume
    lea edx, [rax-9]
    cmp edx, 4
    ja .return
.consume:
    inc qword [rel lcc_rt_input_pos]
    jmp .next
.return:
    add rsp, 8
    ret
