section .text
; Construct a pointer type with base RDI.
type_pointer:
    push rdi
    mov edi, TY_SIZE
    call arena_alloc
    pop rdx
    mov dword [rax+TY_KIND], K_PTR
    mov dword [rax+TY_BYTES], 8
    mov dword [rax+TY_ALIGN], 8
    mov [rax+TY_BASE], rdx
    ret
; Construct a typed node with kind EDI and source token RSI.
node_new:
    push rbx
    push r12
    sub rsp, 8
    mov ebx, edi
    mov r12, rsi
    mov edi, N_SIZE
    call arena_alloc
    mov [rax+N_KIND], ebx
    mov [rax+N_TOKEN], r12
    lea rdx, [type_int]
    mov [rax+N_TYPE], rdx
    add rsp, 8
    pop r12
    pop rbx
    ret
; Choose the usual arithmetic type for RDI and RSI after scalar promotion.
type_common:
    sub rsp, 8
    call type_promote
    mov r8, rax
    mov rdi, rsi
    call type_promote
    mov rsi, rax
    mov rdi, r8
    add rsp, 8
    cmp dword [rdi+TY_KIND], K_STRUCT
    jae type_aggregate_common
    cmp dword [rsi+TY_KIND], K_STRUCT
    jae type_aggregate_common
    cmp dword [rdi+TY_KIND], K_PTR
    je .pointer
    cmp dword [rdi+TY_KIND], K_ARRAY
    je .pointer
    cmp dword [rdi+TY_KIND], K_FUNC
    je .pointer
    cmp dword [rsi+TY_KIND], K_PTR
    je .pointer
    cmp dword [rsi+TY_KIND], K_ARRAY
    je .pointer
    cmp dword [rsi+TY_KIND], K_FUNC
    je .pointer
    mov eax, [rdi+TY_KIND]
    mov edx, [rsi+TY_KIND]
    cmp eax, edx
    cmovb eax, edx
    cmp eax, K_LDOUBLE
    je .extended
    cmp eax, K_DOUBLE
    je .double
    cmp eax, K_FLOAT
    je .single
    mov rax, rdi
    cmp dword [rdi+TY_BYTES], 8
    jae .wide_left
    cmp dword [rsi+TY_BYTES], 8
    jae .right
    lea rax, [type_int]
    cmp dword [rdi+TY_BYTES], 4
    jne .test_right
    test dword [rdi+TY_FLAGS], 1
    jnz .uint
.test_right:
    cmp dword [rsi+TY_BYTES], 4
    jne .done
    test dword [rsi+TY_FLAGS], 1
    jz .done
.uint:
    lea rax, [type_uint]
    ret
.wide_left:
    cmp dword [rsi+TY_BYTES], 8
    jb .done
    test dword [rsi+TY_FLAGS], 1
    jz .done
.right:
    mov rax, rsi
.done:
    ret
.pointer:
    lea rax, [type_ulong]
    ret
.single:
    lea rax, [type_float]
    ret
.double:
    lea rax, [type_double]
    ret
.extended:
    lea rax, [type_ldouble]
    ret
; Integer promotion preserves field signedness until an arithmetic use.
type_promote:
    mov rax, rdi
    cmp dword [rdi+TY_KIND], K_CHAR
    jb .done
    cmp dword [rdi+TY_KIND], K_INT
    ja .done
    jb .integer
    test dword [rdi+TY_FLAGS], TF_ENUM
    jnz .integer
    test dword [rdi+TY_FLAGS], TF_BITFIELD
    jz .done
    cmp qword [rdi+TY_COUNT], 32
    jae .done
.integer:
    lea rax, [type_int]
.done:
    ret
; Reject taking the address or direct sizeof of a bit-field expression.
require_non_bitfield:
    cmp dword [rdi+N_KIND], A_MEMBER
    jne .done
    mov rax, [rdi+N_SYM]
    cmp dword [rax+MEM_BITS], 0
    jne aggregate_error
.done:
    ret
