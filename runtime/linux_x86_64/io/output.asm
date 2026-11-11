; Buffered stdout with newline/capacity flushing and complete syscall writes.
section .bss
align 8
lcc_rt_output_used: resq 1
lcc_rt_output_error: resq 1
lcc_rt_output_bytes: resb 4096

section .text
global lcc_rt_write, lcc_rt_flush, lcc_rt_exit

; Flush pending bytes. Return zero or -1; retry EINTR and partial writes.
lcc_rt_flush:
    push rbx
    push r12
    sub rsp, 8
    cmp qword [rel lcc_rt_output_error], 0
    jne .error
    mov r12, [rel lcc_rt_output_used]
    lea rbx, [rel lcc_rt_output_bytes]
.write:
    test r12, r12
    jz .done
    mov eax, 1
    mov edi, 1
    mov rsi, rbx
    mov rdx, r12
    syscall
    cmp rax, -4
    je .write
    test rax, rax
    jle .failed
    add rbx, rax
    sub r12, rax
    jmp .write
.done:
    mov qword [rel lcc_rt_output_used], 0
    xor eax, eax
    jmp .return
.failed:
    mov qword [rel lcc_rt_output_error], 1
    mov qword [rel lcc_rt_output_used], 0
.error:
    mov eax, -1
.return:
    add rsp, 8
    pop r12
    pop rbx
    ret

; Append RDI/RSI bytes. Return original length, or -1 after an output error.
lcc_rt_write:
    push rbx
    push r12
    push r13
    push r14
    sub rsp, 8
    mov r12, rdi
    mov r13, rsi
    mov r14, rsi
    cmp qword [rel lcc_rt_output_error], 0
    jne .error
    lea rbx, [rel lcc_rt_output_bytes]
.append:
    test r13, r13
    jz .done
    mov rcx, [rel lcc_rt_output_used]
    mov al, [r12]
    mov [rbx+rcx], al
    inc rcx
    mov [rel lcc_rt_output_used], rcx
    inc r12
    dec r13
    cmp al, 10
    je .flush
    cmp ecx, 4096
    jb .append
.flush:
    call lcc_rt_flush
    test eax, eax
    js .error
    jmp .append
.done:
    mov rax, r14
    jmp .return
.error:
    mov rax, -1
.return:
    add rsp, 8
    pop r14
    pop r13
    pop r12
    pop rbx
    ret

; Normal process termination flushes pending prompts and formatted output.
lcc_rt_exit:
    push rdi
    call lcc_rt_flush
    pop rdi
    mov eax, 60
    syscall
    ud2
