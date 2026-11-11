; Sign/zero extend the generated RAX value according to target type RDI.
emit_normalize:
    cmp dword [rdi+TY_KIND], K_STRUCT
    jae .unchanged
    cmp dword [rdi+TY_KIND], K_VOID
    je .unchanged
    cmp dword [rdi+TY_KIND], K_FLOAT
    jb .scalar
    cmp dword [rdi+TY_KIND], K_LDOUBLE
    jbe .unchanged
.scalar:
    cmp dword [rdi+TY_KIND], K_ARRAY
    je .unchanged
    cmp dword [rdi+TY_KIND], K_FUNC
    je .unchanged
    mov eax, [rdi+TY_BYTES]
    test dword [rdi+TY_FLAGS], 1
    jnz .unsigned
    cmp eax, 1
    je .i8
    cmp eax, 2
    je .i16
    cmp eax, 4
    je .i32
    ret
.i8:
    lea rdi, [i_i8]
    jmp emit
.i16:
    lea rdi, [i_i16]
    jmp emit
.i32:
    lea rdi, [i_i32]
    jmp emit
.unsigned:
    cmp eax, 1
    je .u8
    cmp eax, 2
    je .u16
    cmp eax, 4
    je .u32
    ret
.u8:
    lea rdi, [i_u8]
    jmp emit
.u16:
    lea rdi, [i_u16]
    jmp emit
.u32:
    lea rdi, [i_u32]
    jmp emit
.unchanged:
    ret
; Load through generated RAX according to target type RDI.
emit_load:
    mov eax, [rdi+TY_KIND]
    cmp eax, K_STRUCT
    jae .done
    cmp eax, K_FLOAT
    je .u32
    cmp eax, K_LDOUBLE
    je .extended
    cmp eax, K_ARRAY
    je .done
    cmp eax, K_FUNC
    je .done
    mov eax, [rdi+TY_BYTES]
    cmp eax, 8
    je .qword
    test dword [rdi+TY_FLAGS], 1
    jnz .unsigned
    cmp eax, 1
    je .i8
    cmp eax, 2
    je .i16
    cmp eax, 4
    je .i32
    jmp generation_error
.i8:
    lea rdi, [i_load_i8]
    jmp emit
.i16:
    lea rdi, [i_load_i16]
    jmp emit
.i32:
    lea rdi, [i_load_i32]
    jmp emit
.unsigned:
    cmp eax, 1
    je .u8
    cmp eax, 2
    je .u16
    cmp eax, 4
    je .u32
    jmp generation_error
.u8:
    lea rdi, [i_load_u8]
    jmp emit
.u16:
    lea rdi, [i_load_u16]
    jmp emit
.u32:
    lea rdi, [i_load_u32]
    jmp emit
.qword:
    lea rdi, [i_load_64]
    jmp emit
.extended:
    lea rdi, [i_load_128]
    jmp emit
.done:
    ret
; Store generated RAX through RCX according to target type RDI.
emit_store:
    cmp dword [rdi+TY_KIND], K_STRUCT
    jae .aggregate
    cmp dword [rdi+TY_KIND], K_LDOUBLE
    je .extended
    mov eax, [rdi+TY_BYTES]
    cmp eax, 1
    je .byte
    cmp eax, 2
    je .word
    cmp eax, 4
    je .dword
    cmp eax, 8
    jne generation_error
    lea rdi, [i_store_64]
    jmp emit
.byte:
    lea rdi, [i_store_8]
    jmp emit
.word:
    lea rdi, [i_store_16]
    jmp emit
.dword:
    lea rdi, [i_store_32]
    jmp emit
.extended:
    lea rdi, [i_store_128]
    jmp emit
.aggregate:
    mov edi, [rdi+TY_BYTES]
    jmp emit_copy_count
; Copy RDI bytes from generated RAX to RCX, returning the destination address.
emit_copy_count:
    push rbx
    mov rbx, rdi
    lea rdi, [i_copy_setup]
    call emit
    lea rdi, [p_mov_rcx]
    mov rsi, rbx
    mov edx, 8
    call emit_immediate
    lea rdi, [i_copy_bytes]
    call emit
    pop rbx
    ret
