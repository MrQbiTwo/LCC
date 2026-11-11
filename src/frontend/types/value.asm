; Record values need a layout at the point of use, not just at unit end.
section .text
; RDI node -> RAX same node; preserve RDI and callee-saved registers.
; Apply only at value consumers: parentheses, addresses and dereferences alone
; must still be able to designate an incomplete object without reading it.
require_complete_value:
    mov rax, rdi
    mov rdx, [rdi+N_TYPE]
    cmp dword [rdx+TY_KIND], K_STRUCT
    jb .done
    push rdi
    mov rdi, rdx
    call type_resolve
    pop rdi
    test dword [rax+TY_FLAGS], TF_INCOMPLETE
    jnz .bad
    mov rax, rdi
.done:
    ret
.bad:
    mov rax, [rdi+N_TOKEN]
    mov [diag_token], rax
    jmp aggregate_error
