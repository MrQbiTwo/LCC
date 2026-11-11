; System V classification for the supported C scalar and aggregate type domain.
section .rodata
; Merge order follows NO_CLASS, INTEGER, SSE, X87, X87UP, MEMORY.
abi_merge: db 0,1,2,3,4,5, 1,1,1,1,1,5, 2,1,2,5,5,5
           db 3,1,5,3,5,5, 4,1,5,5,4,5, 5,5,5,5,5,5
section .bss
align 8
abi_depth: resq 1 ; Bound recursive array/member classification.
section .text
; Classify RDI into two packed class bytes in EAX, or AC_MEMORY.
abi_shape:
    push rbx
    sub rsp, 16
    mov rbx, rdi
    cmp dword [rdi+TY_KIND], K_STRUCT
    jae .aggregate
    mov eax, AC_INTEGER
    cmp dword [rdi+TY_KIND], K_FLOAT
    je .sse
    cmp dword [rdi+TY_KIND], K_DOUBLE
    je .sse
    cmp dword [rdi+TY_KIND], K_LDOUBLE
    jne .done
    mov eax, AC_X87 | (AC_X87UP << 8)
    jmp .done
.sse:
    mov eax, AC_SSE
    jmp .done
.aggregate:
    call type_require_complete
    cmp dword [rbx+TY_BYTES], 16
    ja .memory
    mov qword [rsp], 0
    mov rdi, rbx
    xor esi, esi
    mov rdx, rsp
    call abi_walk
    movzx eax, byte [rsp]
    movzx edx, byte [rsp+1]
    cmp eax, AC_MEMORY
    je .memory
    cmp edx, AC_MEMORY
    je .memory
    cmp edx, AC_X87UP
    jne .packed
    cmp eax, AC_X87
    jne .memory
.packed:
    shl edx, 8
    or eax, edx
    jmp .done
.memory:
    mov eax, AC_MEMORY
.done:
    add rsp, 16
    pop rbx
    ret
; Merge a recursively classified field at byte offset RSI into two bytes RDX.
abi_walk:
    push rbx
    push r12
    push r13
    push r14
    push r15
    mov rbx, rdi
    mov r12, rsi
    mov r13, rdx
    inc qword [abi_depth]
    cmp qword [abi_depth], 256
    ja generation_error
    mov ecx, [rbx+TY_ALIGN]
    dec ecx
    test r12, rcx
    jnz .memory
    cmp dword [rbx+TY_KIND], K_ARRAY
    je .array
    cmp dword [rbx+TY_KIND], K_STRUCT
    jae .record
    mov r14d, AC_INTEGER
    cmp dword [rbx+TY_KIND], K_FLOAT
    je .sse
    cmp dword [rbx+TY_KIND], K_DOUBLE
    je .sse
    cmp dword [rbx+TY_KIND], K_LDOUBLE
    je .extended
    jmp .scalar
.sse:
    mov r14d, AC_SSE
.scalar:
    mov r15, r12
    shr r15, 3
    mov edi, r14d
    lea rsi, [r13+r15]
    call abi_merge_class
    mov eax, [rbx+TY_BYTES]
    lea rax, [r12+rax-1]
    shr rax, 3
    cmp rax, r15
    je .done
    mov edi, r14d
    lea rsi, [r13+rax]
    call abi_merge_class
    jmp .done
.extended:
    mov edi, AC_X87
    mov rsi, r13
    call abi_merge_class
    mov edi, AC_X87UP
    lea rsi, [r13+1]
    call abi_merge_class
    jmp .done
.array:
    xor r14d, r14d
.element:
    cmp r14, [rbx+TY_COUNT]
    jae .done
    mov rdi, [rbx+TY_BASE]
    mov eax, [rdi+TY_BYTES]
    imul rax, r14
    lea rsi, [r12+rax]
    mov rdx, r13
    call abi_walk
    inc r14
    jmp .element
.record:
    mov r14, [rbx+TY_MEMBERS]
.member:
    test r14, r14
    jz .done
    cmp dword [r14+MEM_BITS], 0
    jne .bit_field
    mov rdi, [r14+MEM_TYPE]
    mov rsi, [r14+MEM_OFFSET]
    add rsi, r12
    mov rdx, r13
    call abi_walk
.next_member:
    mov r14, [r14+MEM_NEXT]
    jmp .member
.bit_field:
    ; Classify the occupied bits, including unnamed padding fields. Their
    ; first byte need not have the declared integer type's natural alignment.
    mov r15, [r14+MEM_OFFSET]
    add r15, r12
    shr r15, 3
    mov edi, AC_INTEGER
    lea rsi, [r13+r15]
    call abi_merge_class
    mov eax, [r14+MEM_BITS]
    add eax, [r14+MEM_SHIFT]
    dec eax
    shr eax, 3
    add rax, [r14+MEM_OFFSET]
    add rax, r12
    shr rax, 3
    cmp rax, r15
    je .next_member
    mov edi, AC_INTEGER
    lea rsi, [r13+rax]
    call abi_merge_class
    jmp .next_member
.memory:
    mov byte [r13], AC_MEMORY
.done:
    dec qword [abi_depth]
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
; Merge class EDI into the existing class byte at RSI.
abi_merge_class:
    movzx eax, byte [rsi]
    imul eax, 6
    add eax, edi
    lea rdx, [abi_merge]
    mov al, [rdx+rax]
    mov [rsi], al
    ret
