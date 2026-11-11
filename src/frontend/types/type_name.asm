; Type names have no outer storage class or declared identifier.
section .text
; Parse from parse_token, returning RAX/type; nested prototype names are legal.
parse_type_name:
    sub rsp, 8
    call parse_declspec
    cmp qword [decl_storage], 0
    jne .bad
    mov rdi, rax
    mov esi, 1
    call parse_declarator
    test rdx, rdx
    jnz .named
    add rsp, 8
    ret
.named:
    mov [diag_token], rdx
.bad:
    lea rdi, [msg_type]
    jmp fatal
