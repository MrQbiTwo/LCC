; Declaration-specifier lookup maps C keywords to target type properties.
section .rodata
decl_keywords:
    dq kw_unsigned,1, kw_signed,2, kw_char,4, kw_short,8
    dq kw_int,16, kw_long,32, kw_float,64, kw_double,128
    dq kw_void,256, kw_const,512, kw_volatile,1024
    dq kw_static,2048, kw_extern,4096, kw_typedef,8192
    dq kw_auto,16384, kw_register,32768, kw_enum,65536
    dq kw_struct,131072, kw_union,262144, 0,0
section .bss
align 8
decl_storage: resq 1 ; Storage-class bits for the last declaration specifier.
section .text
; Test whether the current token starts a supported type declaration.
is_type_token:
    push rbx
    lea rbx, [decl_keywords]
.loop:
    mov rsi, [rbx]
    test rsi, rsi
    jz .alias
    mov rdi, [parse_token]
    call token_is
    test eax, eax
    jnz .yes
    add rbx, 16
    jmp .loop
.alias:
    mov rdi, [parse_token]
    call symbol_find
    test rax, rax
    jz .done
    test dword [rax+S_FLAGS], SF_TYPEDEF
    setnz al
    movzx eax, al
    jmp .done
.yes:
    mov eax, 1
.done:
    pop rbx
    ret
; Parse a declaration's type specifiers and storage classes.
parse_declspec:
    push rbx
    push r12
    push r13
    push r14
    sub rsp, 8
    xor r12d, r12d
    xor r14d, r14d
.next:
    lea rbx, [decl_keywords]
.find:
    mov rsi, [rbx]
    test rsi, rsi
    jz .alias
    mov rdi, [parse_token]
    call token_is
    test eax, eax
    jnz .found
    add rbx, 16
    jmp .find
.found:
    mov r13, [rbx+8]
    cmp r13, 65536
    je .enum
    cmp r13, 131072
    je .aggregate
    cmp r13, 262144
    je .aggregate
    cmp r13, 512
    jae .merge
    test r14, r14
    jnz .bad
    test r12, r13
    jnz .bad
.merge:
    cmp r13, 2048
    jb .merge_bits
    cmp r13, 32768
    ja .merge_bits
    test r12d, 0xf800
    jnz .bad
.merge_bits:
    or r12, r13
    mov rax, [parse_token]
    mov [diag_token], rax
    mov rax, [rax+TK_NEXT]
    mov [parse_token], rax
    jmp .next
.enum:
    test r14, r14
    jnz .bad
    test r12d, 511
    jnz .bad
    lea rdi, [kw_enum]
    call accept_keyword
    call parse_enum
    mov r14, rax
    jmp .next
.aggregate:
    test r14, r14
    jnz .bad
    test r12d, 511
    jnz .bad
    mov edi, T_IDENT
    call expect
    mov edi, K_STRUCT
    cmp r13, 131072
    je .aggregate_kind
    mov edi, K_UNION
.aggregate_kind:
    call parse_aggregate
    mov r14, rax
    jmp .next
.alias:
    test r12d, 511
    jnz .finish
    test r14, r14
    jnz .finish
    mov rdi, [parse_token]
    call symbol_find
    test rax, rax
    jz .finish
    test dword [rax+S_FLAGS], SF_TYPEDEF
    jz .finish
    mov r14, [rax+S_TYPE]
    mov edi, T_IDENT
    call expect
    jmp .next
.finish:
    mov rax, r12
    and eax, 0xf800
    lea ecx, [eax-1]
    test eax, ecx
    jnz .bad
    mov [decl_storage], rax
    test r14, r14
    jz .primitive
    mov rax, r14
    jmp .done
.primitive:
    mov eax, r12d
    and eax, 511
    test eax, eax
    jz .bad
    mov ecx, eax
    and ecx, 3
    cmp ecx, 3
    je .bad
    test eax, 256
    jnz .void
    test eax, 64
    jnz .float
    test eax, 128
    jnz .double
    mov ecx, eax
    and ecx, 44
    cmp ecx, 4
    je .char
    cmp ecx, 8
    je .short
    cmp ecx, 32
    je .long
    test ecx, ecx
    jnz .bad
    lea rax, [type_int]
    lea rdx, [type_uint]
    jmp .sign
.char:
    test eax, 16
    jnz .bad
    lea rax, [type_char]
    lea rdx, [type_schar]
    test r12d, 2
    cmovnz rax, rdx
    lea rdx, [type_uchar]
    jmp .sign
.short:
    lea rax, [type_short]
    lea rdx, [type_ushort]
    jmp .sign
.long:
    lea rax, [type_long]
    lea rdx, [type_ulong]
.sign:
    test r12d, 1
    cmovnz rax, rdx
    jmp .done
.void:
    cmp eax, 256
    jne .bad
    lea rax, [type_void]
    jmp .done
.float:
    cmp eax, 64
    jne .bad
    lea rax, [type_float]
    jmp .done
.double:
    cmp eax, 128
    je .plain_double
    cmp eax, 160
    jne .bad
    lea rax, [type_ldouble]
    jmp .done
.plain_double:
    lea rax, [type_double]
.done:
    mov esi, r12d
    and esi, TF_CONST|TF_VOLATILE
    jz .unqualified
    mov rdi, rax
    call type_qualify
.unqualified:
    add rsp, 8
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
.bad:
    lea rdi, [msg_type]
    jmp fatal
