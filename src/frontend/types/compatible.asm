; Read-only compatibility for the type shapes represented by this frontend.
section .text
; RDI/RSI=pointees -> EAX=boolean. Ignore their outer const/volatile only.
; No allocation, diagnostics or type completion; preserve callee-saved registers.
type_pointee_compatible:
    mov edx, 1
    mov ecx, 256
    jmp type_compatible_query

; EDX=ignore outer qualifiers, ECX=remaining recursive depth.
; Nested types retain qualifiers except adjusted function parameter top levels.
type_compatible_query:
    cmp rdi, rsi
    je .identical
    test ecx, ecx
    jz .exhausted
    push rbx
    push r12
    push r13
    push r14
    push r15
    mov rbx, rdi
    mov r12, rsi
    lea r13d, [rcx-1]
    mov r14d, [rbx+TY_KIND]
    cmp r14d, [r12+TY_KIND]
    jne .no
    test edx, edx
    jnz .kind
    mov eax, [rbx+TY_FLAGS]
    xor eax, [r12+TY_FLAGS]
    test eax, TF_CONST|TF_VOLATILE
    jnz .no
.kind:
    cmp r14d, K_STRUCT
    jae .record
    cmp r14d, K_FUNC
    je .function
    cmp r14d, K_PTR
    je .base
    cmp r14d, K_ARRAY
    je .array
    mov rdi, rbx
    mov rsi, r12
    call type_primitive_compatible
    jmp .done
.array:
    mov rax, [rbx+TY_COUNT]
    test rax, rax
    jz .base
    cmp qword [r12+TY_COUNT], 0
    je .base
    cmp rax, [r12+TY_COUNT]
    jne .no
.base:
    mov rdi, [rbx+TY_BASE]
    mov rsi, [r12+TY_BASE]
    xor edx, edx
    mov ecx, r13d
    call type_compatible_query
    jmp .done
.record:
    ; Qualified record copies link to their canonical tag; never inspect fields.
    mov ecx, 256
.left_record:
    mov rax, [rbx+TY_BASE]
    test rax, rax
    jz .right_record_start
    dec ecx
    jz .no
    mov rbx, rax
    jmp .left_record
.right_record_start:
    mov ecx, 256
.right_record:
    mov rax, [r12+TY_BASE]
    test rax, rax
    jz .record_identity
    dec ecx
    jz .no
    mov r12, rax
    jmp .right_record
.record_identity:
    cmp rbx, r12
    sete al
    movzx eax, al
    jmp .done
.function:
    ; Compatible returns retain their qualifiers even when parameters ignore
    ; their outer qualifiers. Declarator parsing already adjusts array/functions.
    mov rdi, [rbx+TY_BASE]
    mov rsi, [r12+TY_BASE]
    xor edx, edx
    mov ecx, r13d
    call type_compatible_query
    test eax, eax
    jz .done
    mov eax, [rbx+TY_FLAGS]
    mov edx, [r12+TY_FLAGS]
    xor eax, edx
    test eax, 2
    jnz .mixed_prototype
    test edx, 2
    jnz .yes
    test eax, 1
    jnz .no
    mov r14, [rbx+TY_PARAMS]
    mov r15, [r12+TY_PARAMS]
.parameter:
    test r14, r14
    jz .parameters_end
    test r15, r15
    jz .no
    mov rdi, [r14+S_TYPE]
    mov rsi, [r15+S_TYPE]
    mov edx, 1
    mov ecx, r13d
    call type_compatible_query
    test eax, eax
    jz .done
    mov r14, [r14+S_PARAM_NEXT]
    mov r15, [r15+S_PARAM_NEXT]
    jmp .parameter
.parameters_end:
    test r15, r15
    jnz .no
    jmp .yes
.mixed_prototype:
    ; The prototype must be nonvariadic and unchanged by default promotions.
    test dword [rbx+TY_FLAGS], 2
    cmovnz rbx, r12
    test dword [rbx+TY_FLAGS], 1
    jnz .no
    mov r14, [rbx+TY_PARAMS]
.promoted_parameter:
    test r14, r14
    jz .yes
    mov rax, [r14+S_TYPE]
    mov eax, [rax+TY_KIND]
    cmp eax, K_INT
    jb .no
    cmp eax, K_FLOAT
    je .no
    mov r14, [r14+S_PARAM_NEXT]
    jmp .promoted_parameter
.yes:
    mov eax, 1
    jmp .done
.no:
    xor eax, eax
.done:
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
.identical:
    mov eax, 1
    ret
.exhausted:
    xor eax, eax
    ret
