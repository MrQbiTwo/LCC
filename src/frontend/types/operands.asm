; Operand-kind constraints run before common-type selection or AST lowering.
section .text
; RDI=node -> RAX=node. Preserve RDI; clobber RCX/RDX. Arrays/functions decay
; for scalar truth without requiring a complete pointed-to object.
require_scalar_value:
    mov edx, K_FUNC
    jmp require_operand_range
require_arithmetic_value:
    mov edx, K_LDOUBLE
    jmp require_operand_range
require_integer_value:
    mov edx, K_LONG
require_operand_range:
    mov rax, [rdi+N_TYPE]
    mov ecx, [rax+TY_KIND]
    cmp ecx, K_CHAR
    jb .bad
    cmp ecx, edx
    ja .bad
    mov rax, rdi
    ret
.bad:
    mov rax, [rdi+N_TOKEN]
    mov [diag_token], rax
    jmp aggregate_error

; RDI=operand, ESI=prefix operator. Address/deref/step have separate checks.
require_unary_operand:
    cmp esi, '!'
    je require_scalar_value
    cmp esi, '~'
    je require_integer_value
    cmp esi, '+'
    je require_arithmetic_value
    cmp esi, '-'
    je require_arithmetic_value
    ret

; RDI=left, RSI=right, EDX=operator. Preserve callee-saved registers.
; Assignment compatibility has separate work; comparisons have a focused gate.
require_binary_operands:
    cmp edx, T_EQ
    je require_comparison_operands
    cmp edx, T_NE
    je require_comparison_operands
    cmp edx, T_LE
    je require_comparison_operands
    cmp edx, T_GE
    je require_comparison_operands
    cmp edx, '<'
    je require_comparison_operands
    cmp edx, '>'
    je require_comparison_operands
    push rbx
    push r12
    push r13
    mov rbx, rdi
    mov r12, rsi
    mov r13d, edx
    cmp edx, T_AND
    je .logical
    cmp edx, T_OR
    je .logical
    cmp edx, '*'
    je .arithmetic
    cmp edx, '/'
    je .arithmetic
    cmp edx, T_MUL_EQ
    je .arithmetic
    cmp edx, T_DIV_EQ
    je .arithmetic
    cmp edx, '%'
    je .integer
    cmp edx, '&'
    je .integer
    cmp edx, '|'
    je .integer
    cmp edx, '^'
    je .integer
    cmp edx, T_SHL
    je .integer
    cmp edx, T_SHR
    je .integer
    cmp edx, T_MOD_EQ
    jb .additive
    cmp edx, T_SHR_EQ
    jbe .integer
.additive:
    cmp edx, T_ADD_EQ
    je .compound_additive
    cmp edx, T_SUB_EQ
    je .compound_additive
    cmp edx, '+'
    je .ordinary_additive
    cmp edx, '-'
    jne .done
.ordinary_additive:
    mov rax, [rbx+N_TYPE]
    cmp dword [rax+TY_KIND], K_PTR
    je .left_pointer
    cmp dword [rax+TY_KIND], K_ARRAY
    je .left_pointer
    mov rax, [r12+N_TYPE]
    cmp dword [rax+TY_KIND], K_PTR
    je .right_pointer
    cmp dword [rax+TY_KIND], K_ARRAY
    jne .arithmetic
.right_pointer:
    ; Integer + pointer is valid; integer - pointer is not.
    cmp r13d, '+'
    jne .arithmetic
    call require_integer_value
    jmp .done
.left_pointer:
    ; Existing pointer typing validates layout and subtraction compatibility.
    cmp r13d, '-'
    jne .integer_right
    mov rax, [r12+N_TYPE]
    cmp dword [rax+TY_KIND], K_PTR
    je .done
    cmp dword [rax+TY_KIND], K_ARRAY
    je .done
    jmp .integer_right
.compound_additive:
    mov rax, [rbx+N_TYPE]
    cmp dword [rax+TY_KIND], K_PTR
    jne .arithmetic
    ; Compound assignment bypasses ordinary pointer result typing.
    mov rcx, [rbx+N_TOKEN]
    mov [diag_token], rcx
    mov rdi, rax
    call require_step_type
.integer_right:
    mov rdi, r12
    call require_integer_value
    jmp .done
.logical:
    call require_scalar_value
    mov rdi, r12
    call require_scalar_value
    jmp .done
.arithmetic:
    call require_arithmetic_value
    mov rdi, r12
    call require_arithmetic_value
    jmp .done
.integer:
    call require_integer_value
    jmp .integer_right
.done:
    mov rax, rbx
    pop r13
    pop r12
    pop rbx
    ret
