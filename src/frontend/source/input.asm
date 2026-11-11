; Source files and atomic output replacement retain device/inode identities.
section .bss
align 8
source_files: resq 1 ; Linked list of all read source records.
source_total: resq 1 ; Cumulative input bytes.
output_temp: resb 4096 ; Exclusive temporary sibling path.
section .rodata
temp_suffix: db '.lcc-',0
section .text
; Read a regular file RDI into an immutable record and return it in RAX.
source_load:
    push rbx
    push r12
    push r13
    push r14
    push r15
    sub rsp, 144
    mov rbx, rdi
    mov edi, FILE_SIZE
    call arena_alloc
    mov r13, rax
    mov rdi, rbx
    call string_copy
    mov [r13+FILE_PATH], rax
    mov rdi, rbx
    mov esi, 0x80900
    xor edx, edx
    mov eax, 2
    syscall
    test rax, rax
    js .fail
    mov r12, rax
    mov rdi, rax
    mov rsi, rsp
    mov eax, 5
    syscall
    test rax, rax
    js .close_fail
    mov eax, [rsp+24]
    and eax, 0xf000
    cmp eax, 0x8000
    jne .close_fail
    mov r14, [rsp+48]
    cmp r14, MAX_FILE
    ja .close_fail
    mov rax, [source_total]
    add rax, r14
    cmp rax, 268435456
    ja .close_fail
    mov [source_total], rax
    mov rax, [rsp]
    mov [r13+FILE_DEV], rax
    mov rax, [rsp+8]
    mov [r13+FILE_INO], rax
    mov [r13+FILE_LEN], r14
    lea rdi, [r14+2]
    call arena_alloc
    mov [r13+FILE_DATA], rax
    mov r15, rax
.read:
    test r14, r14
    jz .check_end
    mov rdi, r12
    mov rsi, r15
    mov rdx, r14
    xor eax, eax
    syscall
    cmp rax, -4
    je .read
    test rax, rax
    jle .close_fail
    add r15, rax
    sub r14, rax
    jmp .read
.check_end:
    mov rdi, r12
    mov rsi, r15
    mov edx, 1
    xor eax, eax
    syscall
    cmp rax, -4
    je .check_end
    test rax, rax
    jne .close_fail
    mov rdi, r12
    mov eax, 3
    syscall
    mov rax, [source_files]
    mov [r13+FILE_NEXT], rax
    mov [source_files], r13
    mov rax, r13
    add rsp, 144
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
.close_fail:
    mov rdi, r12
    mov eax, 3
    syscall
.fail:
    lea rdi, [msg_read]
    jmp fatal
