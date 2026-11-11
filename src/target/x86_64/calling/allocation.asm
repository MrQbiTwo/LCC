; Assign an argument using RSI={GP count,SSE count,stack bytes}.
; EAX: 0 GP, 1 SSE, 2 memory, 3 aggregate registers. RDX: index/offset or
; two 32-bit slot codes (0 padding, 1..6 GP, 9..16 SSE).
abi_classify:
    push rbx
    push r12
    push r13
    push r14
    push r15
    mov rbx, rdi
    mov r12, rsi
    call abi_shape
    mov r13d, eax
    cmp eax, AC_MEMORY
    je .memory
    movzx ecx, al
    cmp ecx, AC_X87
    jae .memory
    mov ecx, eax
    shr ecx, 8
    cmp ecx, AC_X87
    jae .memory
    mov r14, [r12]
    mov r15, [r12+8]
    mov edi, r13d
    and edi, 255
    call .slot
    mov r8, rax
    mov edi, r13d
    shr edi, 8
    call .slot
    shl rax, 32
    or r8, rax
    cmp r14, 6
    ja .memory
    cmp r15, 8
    ja .memory
    mov [r12], r14
    mov [r12+8], r15
    mov rdx, r8
    mov eax, 3
    cmp dword [rbx+TY_KIND], K_STRUCT
    jae .done
    xor eax, eax
    dec rdx
    cmp edx, 8
    jb .done
    sub edx, 8
    mov eax, 1
    jmp .done
.slot:
    xor eax, eax
    cmp edi, AC_INTEGER
    je .gp_slot
    cmp edi, AC_SSE
    jne .slot_done
    lea rax, [r15+9]
    inc r15
    ret
.gp_slot:
    lea rax, [r14+1]
    inc r14
.slot_done:
    ret
.memory:
    cmp dword [rbx+TY_KIND], K_ARRAY
    je .pointer_alignment
    cmp dword [rbx+TY_KIND], K_FUNC
    je .pointer_alignment
    mov ecx, [rbx+TY_ALIGN]
    cmp ecx, 8
    jae .align
.pointer_alignment:
    mov ecx, 8
.align:
    dec ecx
    mov rdx, [r12+16]
    add rdx, rcx
    not rcx
    and rdx, rcx
    mov eax, [rbx+TY_BYTES]
    ; Arrays and functions at the argument boundary decay to pointers.
    cmp dword [rbx+TY_KIND], K_ARRAY
    je .pointer_size
    cmp dword [rbx+TY_KIND], K_FUNC
    jne .stack_size
.pointer_size:
    mov eax, 8
.stack_size:
    add rax, 7
    and rax, -8
    add rax, rdx
    cmp rax, MAX_MEMORY
    ja generation_error
    mov [r12+16], rax
    mov eax, 2
.done:
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
