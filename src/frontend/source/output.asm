; Validate the destination, write an exclusive sibling, and atomically rename it.
write_output:
    push rbx
    push r12
    push r13
    push r14
    push r15
    sub rsp, 144
    mov qword [diag_token], 0
    mov rdi, [output_path]
    call string_length
    cmp rax, 4030
    ja .fail
    test rax, rax
    jz .fail
    mov r14, rax
    mov rdi, [output_path]
    mov rsi, rsp
    mov eax, 6
    syscall
    cmp rax, -2
    je .temp
    test rax, rax
    js .fail
    mov eax, [rsp+24]
    and eax, 0xf000
    cmp eax, 0x8000
    jne .fail
    mov rbx, [source_files]
.alias:
    test rbx, rbx
    jz .temp
    mov rax, [rbx+FILE_DEV]
    cmp rax, [rsp]
    jne .next
    mov rax, [rbx+FILE_INO]
    cmp rax, [rsp+8]
    je .alias_fail
.next:
    mov rbx, [rbx+FILE_NEXT]
    jmp .alias
.temp:
    lea rdi, [output_temp]
    mov rsi, [output_path]
    mov rcx, r14
    rep movsb
    lea rsi, [temp_suffix]
    mov ecx, 5
    rep movsb
    mov r15, rdi
    mov eax, 39
    syscall
    mov rdi, rax
    mov rsi, r15
    call format_unsigned
    add r15, rax
    mov byte [r15], '-'
    inc r15
    xor ebx, ebx
.try:
    mov rdi, rbx
    mov rsi, r15
    call format_unsigned
    mov byte [r15+rax], 0
    lea rdi, [output_temp]
    mov esi, 0x800c1
    mov edx, 0777o
    cmp dword [assembly_mode], 0
    je .open
    mov edx, 0666o
.open:
    mov eax, 2
    syscall
    cmp rax, -17
    jne .opened
    inc ebx
    cmp ebx, 100
    jb .try
    jmp .fail
.opened:
    test rax, rax
    js .fail
    mov r12, rax
    lea r13, [elf_buffer]
    cmp dword [assembly_mode], 0
    je .write
    lea r13, [asm_buffer]
.write:
    mov rdi, r12
    mov rsi, [r13+BUF_PTR]
    mov rdx, [r13+BUF_LEN]
    call host_write_all
    mov rbx, rax
    mov rdi, r12
    mov eax, 3
    syscall
    or rax, rbx
    js .unlink_fail
    lea rdi, [output_temp]
    mov rsi, [output_path]
    mov eax, 82
    syscall
    test rax, rax
    js .unlink_fail
    add rsp, 144
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
.unlink_fail:
    lea rdi, [output_temp]
    mov eax, 87
    syscall
.fail:
    lea rdi, [msg_write]
    jmp fatal
.alias_fail:
    lea rdi, [msg_alias]
    jmp fatal
