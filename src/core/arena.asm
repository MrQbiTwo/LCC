; Zero-initialized monotonic storage and growable byte buffers.
section .bss
align 8
arena_cursor: resq 1 ; Next unused byte in the current mapping.
arena_end: resq 1 ; End of the current mapping.
arena_total: resq 1 ; Total mapped bytes for the process budget.
section .text
; Allocate RDI bytes, zeroed and aligned to sixteen bytes; return RAX.
arena_alloc:
    add rdi, 15
    jc .fail
    and rdi, -16
    mov rax, [arena_cursor]
    mov rdx, rax
    add rdx, rdi
    jc .fail
    cmp rdx, [arena_end]
    ja .map
    test rax, rax
    jz .map
    mov [arena_cursor], rdx
    ret
.map:
    push rdi
    mov rsi, 1048576
    cmp rdi, rsi
    cmova rsi, rdi
    add rsi, 4095
    and rsi, -4096
    mov rax, [arena_total]
    add rax, rsi
    jc .map_fail
    cmp rax, MAX_MEMORY
    ja .map_fail
    mov [arena_total], rax
    push rsi
    xor edi, edi
    mov edx, 3
    mov r10d, 0x22
    mov r8, -1
    xor r9d, r9d
    mov eax, 9
    syscall
    pop rsi
    pop rdi
    cmp rax, -4095
    jae .fail
    lea rdx, [rax+rsi]
    mov [arena_end], rdx
    lea rdx, [rax+rdi]
    mov [arena_cursor], rdx
    ret
.map_fail:
    pop rdi
.fail:
    lea rdi, [msg_memory]
    jmp fatal
; Append RDX bytes from RSI to a buffer record at RDI.
buffer_append:
    push rbx
    push r12
    push r13
    push r14
    push r15
    mov rbx, rdi
    mov r12, rsi
    mov r13, rdx
    mov r14, [rbx+BUF_LEN]
    add rdx, r14
    jc .fail
    cmp rdx, MAX_MEMORY
    ja .fail
    cmp rdx, [rbx+BUF_CAP]
    jbe .copy
    mov r15, [rbx+BUF_CAP]
    test r15, r15
    jnz .grow
    mov r15d, 4096
.grow:
    cmp r15, rdx
    jae .allocate
    shl r15, 1
    jmp .grow
.allocate:
    mov rdi, r15
    call arena_alloc
    mov rdi, rax
    mov rsi, [rbx+BUF_PTR]
    mov rcx, r14
    rep movsb
    mov [rbx+BUF_PTR], rax
    mov [rbx+BUF_CAP], r15
.copy:
    mov rdi, [rbx+BUF_PTR]
    add rdi, r14
    mov rsi, r12
    mov rcx, r13
    rep movsb
    add r14, r13
    mov [rbx+BUF_LEN], r14
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
.fail:
    lea rdi, [msg_memory]
    jmp fatal
; Append the zero-terminated string RSI to buffer RDI.
buffer_string:
    xor edx, edx
.scan:
    cmp byte [rsi+rdx], 0
    je buffer_append
    inc rdx
    jmp .scan
; Copy a zero-terminated string RDI into immutable arena storage.
string_copy:
    push rbx
    mov rbx, rdi
    call string_length
    lea rdi, [rax+1]
    call arena_alloc
    mov rdx, rax
.copy:
    mov cl, [rbx]
    mov [rax], cl
    inc rbx
    inc rax
    test cl, cl
    jnz .copy
    mov rax, rdx
    pop rbx
    ret
; Return the byte length of a zero-terminated string RDI.
string_length:
    xor eax, eax
.loop:
    cmp byte [rdi+rax], 0
    je .done
    inc rax
    jmp .loop
.done:
    ret
; Return one when the two zero-terminated strings RDI and RSI match.
string_equal:
.loop:
    mov al, [rdi]
    cmp al, [rsi]
    jne .no
    inc rdi
    inc rsi
    test al, al
    jnz .loop
    mov eax, 1
    ret
.no:
    xor eax, eax
    ret
; Render unsigned RDI in decimal at RSI; return the byte count.
format_unsigned:
    mov rax, rdi
    mov r8, rsi
    sub rsp, 32
    lea r9, [rsp+32]
    mov r10d, 10
.loop:
    xor edx, edx
    div r10
    add dl, '0'
    dec r9
    mov [r9], dl
    test rax, rax
    jnz .loop
    lea rcx, [rsp+32]
    sub rcx, r9
    mov rax, rcx
    mov rdi, r8
    mov rsi, r9
    rep movsb
    add rsp, 32
    ret
