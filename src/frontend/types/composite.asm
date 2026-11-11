; Construct conditional pointee composites without changing either input graph.
section .text
; RDI/RSI=pointees -> RAX=composite. The caller unions outer qualifiers.
; Validate once; recursive construction shares the query's 256-level bound.
type_composite_pointee:
    push rbx
    push r12
    sub rsp, 8
    mov rbx, rdi
    mov r12, rsi
    call type_pointee_compatible
    test eax, eax
    jz aggregate_error
    mov rdi, rbx
    mov rsi, r12
    mov ecx, 256
    call type_composite_build
    add rsp, 8
    pop r12
    pop rbx
    ret

; Compatible RDI/RSI, ECX=remaining depth. Leaves retain canonical identity.
; Copy derived records only when needed; never complete an input array in place.
type_composite_build:
    mov rax, rdi
    cmp rdi, rsi
    je .same
    test ecx, ecx
    jz aggregate_error
    cmp dword [rdi+TY_KIND], K_PTR
    jb .same
    cmp dword [rdi+TY_KIND], K_STRUCT
    jae .same
    cmp dword [rdi+TY_KIND], K_FUNC
    je type_composite_function
    push rbx
    push r12
    push r13
    push r14
    push r15
    mov rbx, rdi
    mov r12, rsi
    lea r13d, [rcx-1]
    mov rdi, [rbx+TY_BASE]
    mov rsi, [r12+TY_BASE]
    mov ecx, r13d
    call type_composite_build
    mov r14, rax
    mov r15, rbx
    cmp dword [rbx+TY_KIND], K_ARRAY
    jne .base
    cmp qword [rbx+TY_COUNT], 0
    jne .base
    ; Compatibility has checked known bounds. Prefer the known array record.
    cmp qword [r12+TY_COUNT], 0
    cmovne r15, r12
.base:
    mov rax, r15
    cmp r14, [r15+TY_BASE]
    je .done
    mov rdi, r15
    call type_composite_copy
    mov [rax+TY_BASE], r14
    ; Composing element types changes neither their storage size nor alignment.
.done:
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
.same:
    ret

; Copy the type record RDI into the arena; RAX=new record, RDI preserved.
type_composite_copy:
    push rdi
    mov edi, TY_SIZE
    call arena_alloc
    mov rsi, [rsp]
    mov rdi, rax
    mov ecx, TY_SIZE/8
    rep movsq
    pop rdi
    ret
