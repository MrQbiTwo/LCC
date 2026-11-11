; Explicit address-taking keeps register restrictions on objects and dot members.
section .text
; RDI node is preserved; require_lvalue returns its owning lvalue in RAX.
; Dereference and arrow terminate ownership: their pointee is a different object.
require_addressable:
    sub rsp, 8
    call require_non_bitfield
    call require_lvalue
    add rsp, 8
    cmp dword [rax+N_KIND], A_VAR
    jne .done
    mov rax, [rax+N_SYM]
    test dword [rax+S_FLAGS], SF_REGISTER
    jz .done
    mov rax, [rdi+N_TOKEN]
    mov [diag_token], rax
    jmp aggregate_error
.done:
    ret
