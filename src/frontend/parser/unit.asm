; Parse a complete translation unit from the linked token list RDI.
parse_unit:
    push rbx
    push r12
    push r13
    push r14
    push r15
    mov [parse_token], rdi
.next:
    mov rax, [parse_token]
    cmp dword [rax+TK_KIND], 0
    je .done
    call parse_declspec
    mov r12, rax
    mov r15, [decl_storage]
    test r15, 16384|32768
    jnz .bad_type
    mov edi, ';'
    call accept
    test rax, rax
    jz .declarator
    test dword [r12+TY_FLAGS], TF_ENUM
    jnz .next
    cmp dword [r12+TY_KIND], K_STRUCT
    jb .bad_type
    jmp .next
.declarator:
    mov rdi, r12
    xor esi, esi
    call parse_declarator
    mov r13, rax
    mov r14, rdx
    test r15, 8192
    jnz .find_global
    mov rdi, r13
    call type_array_object
    mov r13, rax
.find_global:
    mov rdi, r14
    call symbol_find
    mov rbx, rax
    test rbx, rbx
    jnz .found_global
    test r15, 8192
    jnz .new_global
    mov rdi, r14
    call symbol_linkage_find
    mov rbx, rax
    test rbx, rbx
    jz .new_global
.found_global:
    test r15, 8192
    jnz .duplicate
    test dword [rbx+S_FLAGS], SF_TYPEDEF|SF_ENUM
    jnz .duplicate
    jmp .existing
.new_global:
    mov edi, S_SIZE
    call arena_alloc
    mov rbx, rax
    mov [rbx+S_NAME], r14
    mov [rbx+S_TYPE], r13
    mov rax, [global_symbols]
    mov [rbx+S_NEXT], rax
    mov [global_symbols], rbx
    test r15, 2048
    jz .existing
    or dword [rbx+S_FLAGS], SF_STATIC
.existing:
    test r15, 8192
    jz .ordinary_global
    or dword [rbx+S_FLAGS], SF_TYPEDEF
    jmp .more
.ordinary_global:
    mov rdi, rbx
    mov rsi, r13
    call entity_type_merge
    test r15, 2048
    jz .existing_linkage
    test dword [rbx+S_FLAGS], SF_STATIC
    jz .bad_type
.existing_linkage:
    test dword [rbx+S_FLAGS], SF_STATIC
    jz .expose
    test r15, 2048|4096
    jnz .expose
    cmp dword [r13+TY_KIND], K_FUNC
    jne .bad_type
.expose:
    test dword [rbx+S_FLAGS], SF_HIDDEN
    jz .visible_type
    and dword [rbx+S_FLAGS], ~SF_HIDDEN
    mov [rbx+S_TYPE], r13
.visible_type:
    mov rax, [rbx+S_TYPE]
    mov rdi, rax
    mov rsi, r13
    call declaration_type_merge
    mov [rbx+S_TYPE], rax
    cmp dword [r13+TY_KIND], K_FUNC
    je .function_declaration
    mov r13, rax
.function_declaration:
    cmp dword [r13+TY_KIND], K_FUNC
    jne .object
    mov rax, [parse_token]
    cmp dword [rax+TK_KIND], '{'
    jne .more
    ; Empty-list definitions have no parameters, unlike unspecified declarations.
    test dword [r13+TY_FLAGS], 2
    jz .definition_type
    mov rax, [rbx+S_COMPAT]
    cmp qword [rax+TY_COUNT], 0
    jne .bad_type
    ; Preserve a visible (void) prototype when the definition uses empty ().
    mov rax, [rbx+S_TYPE]
    test dword [rax+TY_FLAGS], 2
    jnz .definition_type
    and dword [r13+TY_FLAGS], ~2
.definition_type:
    mov rax, [r13+TY_BASE]
    mov rdi, rax
    call type_require_complete
    test dword [rbx+S_FLAGS], SF_DEFINED
    jnz .duplicate
    mov [rbx+S_TYPE], r13
    or dword [rbx+S_FLAGS], SF_DEFINED
    mov [current_function], rbx
    mov qword [parse_labels], 0
    mov qword [local_symbols], 0
    mov qword [local_bytes], 0
    mov qword [rbx+S_RETURN], 0
    mov rdi, [r13+TY_BASE]
    call abi_shape
    cmp eax, AC_MEMORY
    jne .result_home
    mov qword [local_bytes], 8
    mov qword [rbx+S_RETURN], 8
.result_home:
    test dword [r13+TY_FLAGS], 1
    jz .parameter_homes
    mov rax, [local_bytes]
    add rax, 191
    and rax, -16
    mov [local_bytes], rax
    mov [rbx+S_VA_AREA], rax
.parameter_homes:
    mov dword [scope_depth], 1
    mov r14, [r13+TY_PARAMS]
.parameters:
    test r14, r14
    jz .body
    mov rdi, [r14+S_NAME]
    test rdi, rdi
    jz syntax_error
    mov rsi, [r14+S_TYPE]
    call local_new
    mov edx, [r14+S_FLAGS]
    and edx, SF_REGISTER
    or [rax+S_FLAGS], edx
    mov rdx, [rax+S_OFFSET]
    mov [r14+S_OFFSET], rdx
    mov r14, [r14+S_PARAM_NEXT]
    jmp .parameters
.body:
    ; The outer function block shares its declaration scope with parameters.
    mov dword [scope_depth], 0
    call parse_statement
    mov [rbx+S_BODY], rax
    mov r14, [parse_labels]
.check_labels:
    test r14, r14
    jz .frame_size
    mov rax, [r14]
    mov [diag_token], rax
    cmp qword [r14+24], 0
    je .duplicate
    mov r14, [r14+8]
    jmp .check_labels
.frame_size:
    mov rax, [local_bytes]
    add rax, 15
    and rax, -16
    mov [rbx+S_OFFSET], rax
    mov qword [current_function], 0
    mov qword [local_symbols], 0
    mov dword [scope_depth], 0
    jmp .next
.object:
    cmp dword [r13+TY_KIND], K_VOID
    je .bad_type
    test r15, 4096
    jnz .initializer
    mov rdi, r13
    ; Only external tentative records may wait for a later tag definition.
    test dword [rbx+S_FLAGS], SF_STATIC
    jnz .object_layout
    cmp dword [r13+TY_KIND], K_STRUCT
    jae .object_pending
.object_layout:
    call type_object_pending
.object_pending:
    or dword [rbx+S_FLAGS], SF_DEFINED
.initializer:
    mov edi, '='
    call accept
    test rax, rax
    jz .uninitialized
    cmp qword [rbx+S_BODY], 0
    jne .duplicate
    or dword [rbx+S_FLAGS], SF_DEFINED
    mov rdi, r13
    call parse_initializer
    mov [rbx+S_BODY], rax
    jmp .more
.uninitialized:
    ; C89 internal-linkage tentative objects require a complete type now.
    test r15, 4096
    jnz .more
    test dword [rbx+S_FLAGS], SF_STATIC
    jz .more
    mov rdi, r13
    call type_require_complete
.more:
    mov edi, ','
    call accept
    test rax, rax
    jnz .declarator
    mov edi, ';'
    call expect
    jmp .next
.done:
    call finalize_objects
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
.duplicate:
    lea rdi, [msg_name]
    jmp fatal
.bad_type:
    lea rdi, [msg_type]
    jmp fatal
