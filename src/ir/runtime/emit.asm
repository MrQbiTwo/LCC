; Append validated packed sections and translate relocation metadata to fixups.
section .text
runtime_emit:
    cmp qword [runtime_active], 0
    je .absent
    push rbx
    push r12
    push r13
    mov edi, 1
    lea rsi, [runtime_code_bytes]
    mov edx, runtime_code_size
    mov ecx, runtime_code_alignment
    call runtime_blob
    mov edi, 2
    lea rsi, [runtime_data_bytes]
    mov edx, runtime_data_size
    mov ecx, runtime_data_alignment
    call runtime_blob
    xor ebx, ebx
.public:
    lea rax, [runtime_symbols]
    mov rdi, [rax+rbx*8]
    test rdi, rdi
    jz .next_public
    mov rdi, [rdi+S_LABEL]
    lea rax, [runtime_public_offsets]
    mov rsi, [rax+rbx*8]
    call runtime_alias
.next_public:
    inc rbx
    cmp rbx, 3
    jb .public
    mov rdi, [runtime_exit_label]
    mov esi, runtime_exit_offset
    call runtime_alias
    lea rbx, [runtime_relocations]
    mov r12d, runtime_relocation_count
.fixup:
    test r12, r12
    jz .done
    mov edi, F_SIZE
    call arena_alloc
    mov r13, rax
    mov rdx, [fixup_list]
    mov [rax+F_NEXT], rdx
    mov [fixup_list], rax
    mov rcx, [rbx]
    mov [rax+F_SECTION], rcx
    lea rdx, [runtime_offsets-8]
    mov rdx, [rdx+rcx*8]
    add rdx, [rbx+8]
    mov [rax+F_POS], rdx
    mov rcx, [rbx+16]
    lea rdx, [runtime_bases-8]
    mov rdx, [rdx+rcx*8]
    mov [rax+F_LABEL], rdx
    mov rdx, [rbx+24]
    add rdx, [rbx+40]
    jo generation_error
    mov [rax+F_ADDEND], rdx
    mov edx, [rbx+32]
    mov [rax+F_WIDTH], edx
    mov rdx, [diag_token]
    mov [rax+F_TOKEN], rdx
    add rbx, 48
    dec r12
    jmp .fixup
.done:
    pop r13
    pop r12
    pop rbx
.absent:
    ret

; Append section EDI from RSI/length RDX with alignment ECX, retaining its base.
runtime_blob:
    push rbx
    push r12
    push r13
    push r14
    push r15
    mov r12, rdi
    mov r13, rsi
    mov r14, rdx
    mov r15, rcx
    lea rbx, [code_buffer]
    lea rsi, [runtime_text]
    cmp r12d, 1
    je .section
    lea rbx, [data_buffer]
    lea rsi, [asm_data]
.section:
    mov [emit_section], r12
    call asm_string
    lea rsi, [runtime_align]
    call asm_string
    mov rdi, r15
    call asm_unsigned
    lea rsi, [runtime_align_tail]
    call asm_string
    mov rdi, [rbx+BUF_LEN]
    neg rdi
    lea rax, [r15-1]
    and rdi, rax
    mov r15, rdi
    test rdi, rdi
    jz .anchor
    call arena_alloc
    mov rdi, rbx
    mov rsi, rax
    mov rdx, r15
    call buffer_append
.anchor:
    lea rax, [runtime_bases-8]
    mov rdi, [rax+r12*8]
    call label_define
    mov rdx, [rbx+BUF_LEN]
    lea rax, [runtime_offsets-8]
    mov [rax+r12*8], rdx
    mov rdi, rbx
    mov rsi, r13
    mov rdx, r14
    call buffer_append
    mov rdi, r12
    mov rsi, r13
    mov rdx, r14
    call runtime_render
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
