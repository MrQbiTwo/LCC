; Implicit assignment conversions shared by stores, arguments and returns.
section .text
; RDI=source expression, RSI=destination type -> RAX=source expression.
; Preserve callee-saved registers. Resolve source record completeness, but do
; not allocate or rewrite AST nodes. Optional null recognition is nonfatal.
require_implicit_conversion:
    push rbx
    push r12
    push r13
    mov rbx, rdi
    mov r12, rsi
    call require_complete_value
    mov r13, [rbx+N_TYPE]
    mov eax, [r12+TY_KIND]
    cmp eax, K_STRUCT
    jae .record
    cmp eax, K_PTR
    je .pointer
    ; Only arithmetic destinations accept arithmetic source values.
    cmp eax, K_CHAR
    jb .bad
    cmp eax, K_LDOUBLE
    ja .bad
    mov eax, [r13+TY_KIND]
    cmp eax, K_CHAR
    jb .bad
    cmp eax, K_LDOUBLE
    ja .bad
    jmp .done
.record:
    cmp dword [r13+TY_KIND], K_STRUCT
    jb .bad
    mov rdi, r12
    mov rsi, r13
    call type_pointee_compatible
    test eax, eax
    jz .bad
    jmp .done
.pointer:
    mov eax, [r13+TY_KIND]
    cmp eax, K_CHAR
    jb .bad
    cmp eax, K_LONG
    jbe .integer_null
    cmp eax, K_PTR
    jb .bad
    cmp eax, K_FUNC
    ja .bad
    ; Unqualified (void*) ICE zero is also a null pointer constant, including
    ; when the destination points to a function rather than an object.
    mov rdi, rbx
    call conditional_null
    test eax, eax
    jnz .done
    mov r12, [r12+TY_BASE]
    cmp dword [r13+TY_KIND], K_FUNC
    je .pointees
    mov r13, [r13+TY_BASE]
.pointees:
    ; Destination pointees may add immediate qualifiers, never discard them.
    mov eax, [r12+TY_FLAGS]
    not eax
    and eax, [r13+TY_FLAGS]
    test eax, TF_CONST|TF_VOLATILE
    jnz .bad
    cmp dword [r12+TY_KIND], K_VOID
    je .destination_void
    cmp dword [r13+TY_KIND], K_VOID
    jne .compatible
    cmp dword [r12+TY_KIND], K_FUNC
    je .bad
    jmp .done
.destination_void:
    cmp dword [r13+TY_KIND], K_FUNC
    je .bad
    jmp .done
.compatible:
    mov rdi, r12
    mov rsi, r13
    call type_pointee_compatible
    test eax, eax
    jz .bad
    jmp .done
.integer_null:
    mov rdi, rbx
    call comparison_null
    test eax, eax
    jz .bad
.done:
    mov rax, rbx
    pop r13
    pop r12
    pop rbx
    ret
.bad:
    mov rax, [rbx+N_TOKEN]
    mov [diag_token], rax
    jmp aggregate_error

; RDI=call node. Counts are checked by the parser; walk fixed parameters here.
; Variadic/unspecified arguments retain default promotions but need a value.
require_call_conversions:
    push rbx
    push r12
    sub rsp, 8
    mov rbx, [rdi+N_ARGS]
    mov rax, [rdi+N_AUX]
    mov r12, [rax+TY_PARAMS]
    test dword [rax+TY_FLAGS], 2
    jz .next
    xor r12d, r12d
.next:
    test rbx, rbx
    jz .done
    test r12, r12
    jz .promoted
    mov rdi, rbx
    mov rsi, [r12+S_TYPE]
    call require_implicit_conversion
    mov r12, [r12+S_PARAM_NEXT]
    jmp .advance
.promoted:
    mov rax, [rbx+N_TYPE]
    cmp dword [rax+TY_KIND], K_VOID
    je .bad
.advance:
    mov rbx, [rbx+N_NEXT]
    jmp .next
.done:
    add rsp, 8
    pop r12
    pop rbx
    ret
.bad:
    mov rax, [rbx+N_TOKEN]
    mov [diag_token], rax
    jmp aggregate_error
