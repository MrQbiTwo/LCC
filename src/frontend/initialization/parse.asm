; Convert nested C initializers to ordered, typed object-offset stores.
section .bss
align 8
initializer_depth: resq 1 ; Bounds both explicit braces and elided nesting.
section .text
; Parse an initializer and complete an unknown outer array bound if needed.
parse_initializer:
    push rbx
    push r12
    sub rsp, 24
    mov r12, rdi
    call type_object_pending
    mov edi, A_INIT
    mov rsi, [parse_token]
    call node_new
    mov rbx, rax
    mov [rbx+N_TYPE], r12
    mov [rsp], rbx
    mov [rsp+8], rbx
    mov qword [rsp+16], 0
    mov rdi, r12
    xor esi, esi
    mov rdx, rsp
    xor ecx, ecx
    xor r8d, r8d
    call initializer_type
    mov rax, [rbx+N_NEXT]
    mov [rbx+N_ARGS], rax
    mov qword [rbx+N_NEXT], 0
    mov rax, rbx
    add rsp, 24
    pop r12
    pop rbx
    ret
