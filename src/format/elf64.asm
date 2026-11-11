; ELF64 serialization with separate read-only, executable, and writable loads.
section .rodata
align 8
elf_header:
    db 0x7f,'ELF',2,1,1,0
    dq 0
    dw 2,62
    dd 1
    dq 0,64,0
    dd 0
    dw 64,56,4,0,0,0
    ; Read-only file and program headers.
    dd 1,4
    dq 0,0x400000,0x400000,288,288,4096
    ; Executable code segment; sizes are patched below.
    dd 1,5
    dq 4096,0x401000,0x401000,0,0,4096
    ; Writable initialized data; position and sizes are patched below.
    dd 1,6
    dq 0,0,0,0,0,4096
    ; A non-executable stack is part of every generated executable.
    dd 0x6474e551,6
    dq 0,0,0,0,0,16
elf_header_end:
section .text
; Resolve native relocations, then build the final executable image.
elf_create:
    push rbx
    push r12
    push r13
    push r14
    push r15
    mov r12, [code_buffer+BUF_LEN]
    lea r13, [r12+8191]
    and r13, -4096
    lea r14, [r13+0x400000]
    mov rbx, [fixup_list]
.fixup:
    test rbx, rbx
    jz .image
    mov rax, [rbx+F_TOKEN]
    mov [diag_token], rax
    mov rax, [rbx+F_LABEL]
    cmp qword [rax+L_POS], -1
    je .unresolved
    mov rdx, [rax+L_POS]
    cmp qword [rax+L_SECTION], 1
    jne .data_address
    add rdx, 0x401000
    jmp .addressed
.data_address:
    add rdx, r14
.addressed:
    mov rax, [rbx+F_ADDEND]
    test rax, rax
    js .subtract_addend
    add rdx, rax
    jc .range
    jmp .patch
.subtract_addend:
    ; A negative signed addend may not wrap below address zero.
    add rdx, rax
    jnc .range
.patch:
    mov rcx, [rbx+F_POS]
    mov rsi, [code_buffer+BUF_PTR]
    mov rdi, [code_buffer+BUF_LEN]
    mov r8d, 0x401000
    cmp qword [rbx+F_SECTION], 1
    je .patch_buffer
    cmp qword [rbx+F_SECTION], 2
    jne .range
    mov rsi, [data_buffer+BUF_PTR]
    mov rdi, [data_buffer+BUF_LEN]
    mov r8, r14
.patch_buffer:
    mov eax, [rbx+F_WIDTH]
    add rax, rcx
    jc .range
    cmp rax, rdi
    ja .range
    cmp dword [rbx+F_WIDTH], 8
    je .absolute
    cmp dword [rbx+F_WIDTH], 4
    jne .range
    sub rdx, rcx
    sub rdx, r8
    sub rdx, 4
    movsxd rax, edx
    cmp rax, rdx
    jne .range
    mov [rsi+rcx], edx
    jmp .next
.absolute:
    mov [rsi+rcx], rdx
.next:
    mov rbx, [rbx+F_NEXT]
    jmp .fixup
.image:
    mov r15, [data_buffer+BUF_LEN]
    lea rdi, [r13+r15]
    cmp rdi, MAX_MEMORY
    ja .range
    call arena_alloc
    mov rbx, rax
    mov [elf_buffer+BUF_PTR], rax
    lea rax, [r13+r15]
    mov [elf_buffer+BUF_LEN], rax
    mov [elf_buffer+BUF_CAP], rax
    mov rdi, rbx
    lea rsi, [elf_header]
    mov ecx, elf_header_end-elf_header
    rep movsb
    mov rax, [entry_label]
    mov rax, [rax+L_POS]
    add rax, 0x401000
    mov [rbx+24], rax
    mov [rbx+152], r12
    mov [rbx+160], r12
    mov [rbx+184], r13
    mov [rbx+192], r14
    mov [rbx+200], r14
    mov [rbx+208], r15
    mov [rbx+216], r15
    lea rdi, [rbx+4096]
    mov rsi, [code_buffer+BUF_PTR]
    mov rcx, r12
    rep movsb
    lea rdi, [rbx+r13]
    mov rsi, [data_buffer+BUF_PTR]
    mov rcx, r15
    rep movsb
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
.unresolved:
    lea rdi, [msg_name]
    jmp fatal
.range:
    lea rdi, [msg_range]
    jmp fatal
