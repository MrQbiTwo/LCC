; Allocate a stable label, optionally named by the identifier token RDI.
label_new:
    push rdi
    mov edi, L_SIZE
    call arena_alloc
    pop rdx
    mov [rax+L_NAME], rdx
    mov rcx, [label_count]
    inc rcx
    mov [label_count], rcx
    mov [rax+L_ID], rcx
    mov qword [rax+L_POS], -1
    mov rdx, [label_list]
    mov [rax+L_NEXT], rdx
    mov [label_list], rax
    ret
; Define label RDI at the current section offset.
label_define:
    push rbx
    mov rbx, rdi
    mov rax, [emit_section]
    mov [rbx+L_SECTION], rax
    mov rdx, [code_buffer+BUF_LEN]
    cmp eax, 1
    je .position
    mov rdx, [data_buffer+BUF_LEN]
.position:
    mov [rbx+L_POS], rdx
    call asm_label_name
    lea rsi, [asm_colon]
    call asm_string
    pop rbx
    ret
; Emit a branch/call prefix RDI with a relative target label RSI.
emit_branch:
    push rbx
    mov rbx, rsi
    call emit
    mov rdi, rbx
    mov esi, 4
    call emit_fixup
    pop rbx
    ret
; Emit an absolute label address RDI into RAX.
emit_label_address:
    push rdi
    lea rdi, [p_mov64]
    call emit
    pop rdi
    mov esi, 8
    jmp emit_fixup
; Append a checked relocation for label RDI and width ESI.
emit_fixup:
    push rbx
    push r12
    sub rsp, 24
    mov rbx, rdi
    mov r12d, esi
    mov edi, F_SIZE
    call arena_alloc
    mov rdx, [fixup_list]
    mov [rax+F_NEXT], rdx
    mov [fixup_list], rax
    mov rdx, [code_buffer+BUF_LEN]
    mov [rax+F_POS], rdx
    mov [rax+F_LABEL], rbx
    mov [rax+F_WIDTH], r12d
    mov qword [rax+F_ADDEND], 0
    mov qword [rax+F_SECTION], 1
    mov rdx, [diag_token]
    mov [rax+F_TOKEN], rdx
    mov qword [rsp], 0
    lea rdi, [code_buffer]
    mov rsi, rsp
    mov edx, r12d
    call buffer_append
    mov rdi, rbx
    call asm_label_name
    call asm_newline
    add rsp, 24
    pop r12
    pop rbx
    ret
section .rodata
asm_address_plus: db ' + ',0
asm_address_minus: db ' - ',0
section .text
; Append an absolute data reference to label RDI with signed byte addend RSI.
emit_data_address:
    push rbx
    push r12
    sub rsp, 24
    mov rbx, rdi
    mov r12, rsi
    mov edi, F_SIZE
    call arena_alloc
    mov rdx, [fixup_list]
    mov [rax+F_NEXT], rdx
    mov [fixup_list], rax
    mov rdx, [data_buffer+BUF_LEN]
    mov [rax+F_POS], rdx
    mov [rax+F_LABEL], rbx
    mov dword [rax+F_WIDTH], 8
    mov [rax+F_ADDEND], r12
    mov qword [rax+F_SECTION], 2
    mov rdx, [diag_token]
    mov [rax+F_TOKEN], rdx
    mov qword [rsp], 0
    lea rdi, [data_buffer]
    mov rsi, rsp
    mov edx, 8
    call buffer_append
    lea rsi, [asm_dq]
    call asm_string
    mov rdi, rbx
    call asm_label_name
    test r12, r12
    jz .newline
    lea rsi, [asm_address_plus]
    jns .addend
    lea rsi, [asm_address_minus]
    neg r12
.addend:
    call asm_string
    ; Unsigned formatting also represents the magnitude of INT64_MIN exactly.
    mov rdi, r12
    call asm_unsigned
.newline:
    call asm_newline
    add rsp, 24
    pop r12
    pop rbx
    ret
