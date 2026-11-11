; Explicit conversions have different constraints from implicit assignment.
section .text
; RDI=operand node, RSI=destination type, RDX=cast token -> RAX=operand.
; Preserve RDI/RSI and callee-saved registers; clobber RAX/RCX. The parser
; separately requires complete record values, including discarded void casts.
require_cast_value:
    mov ecx, [rsi+TY_KIND]
    cmp ecx, K_VOID
    je .done
    cmp ecx, K_CHAR
    jb .bad
    cmp ecx, K_PTR
    ja .bad
    mov rax, [rdi+N_TYPE]
    mov eax, [rax+TY_KIND]
    cmp eax, K_CHAR
    jb .bad
    ; Array/function operands decay, but cannot be cast to floating types.
    cmp eax, K_FUNC
    ja .bad
    cmp eax, K_PTR
    jae .pointer_source
    cmp ecx, K_PTR
    jne .done
    cmp eax, K_FLOAT
    jae .bad
    jmp .done
.pointer_source:
    cmp ecx, K_FLOAT
    jb .done
    cmp ecx, K_LDOUBLE
    jbe .bad
.done:
    mov rax, rdi
    ret
.bad:
    mov [diag_token], rdx
    jmp aggregate_error
