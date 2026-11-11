; Primitive identity shared by declarations and expression compatibility.
section .text
; RDI/RSI have the same kind below K_PTR; EAX=compatible ignoring qualifiers.
; Clobber only RAX/RDX. Enum TY_BASE is a direct identity token, never a chain.
type_primitive_compatible:
    mov eax, [rdi+TY_FLAGS]
    mov edx, [rsi+TY_FLAGS]
    xor eax, edx
    test eax, 1|TF_PLAIN_CHAR
    jnz .no
    ; Each enum is compatible with signed int, but distinct enums are not.
    and edx, [rdi+TY_FLAGS]
    test edx, TF_ENUM
    jz .yes
    mov rax, [rdi+TY_BASE]
    cmp rax, [rsi+TY_BASE]
    sete al
    movzx eax, al
    ret
.yes:
    mov eax, 1
    ret
.no:
    xor eax, eax
    ret
