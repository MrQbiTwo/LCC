; Target-width arithmetic shared by ordinary and static numeric constants.
section .text
; RDI node, RSI left bits, RDX right bits -> RAX result, zero RDX.
; Children are already evaluated; never evaluate an AST subtree twice here.
constant_integer_binary:
    mov rax, [rdi+N_TOKEN]
    mov [diag_token], rax
    sub rsp, 8
    call constant_try_integer_binary
    add rsp, 8
    test ecx, ecx
    jnz constant_arithmetic_error
    ret

; Same operands/result, with ECX=0/8/11 instead of diagnostics or fatal exits.
constant_try_integer_binary:
    push rbx
    push r12
    push r13
    mov rbx, rdi
    mov r12, rsi
    mov r13, [rbx+N_AUX]
    test r13, r13
    jz .type
    mov rdi, r13
    mov rax, r12
    call constant_integer_normalize
    mov r12, rax
    mov rax, rdx
    ; A shift count keeps its own promoted width, independently of the left.
    cmp dword [rbx+N_OP], T_SHL
    je .shift
    cmp dword [rbx+N_OP], T_SHR
    je .shift
    call constant_integer_normalize
    mov ecx, [rbx+N_OP]
    cmp ecx, '+'
    je .add
    cmp ecx, '-'
    je .subtract
    cmp ecx, '*'
    je .multiply
    cmp ecx, '/'
    je .divide
    cmp ecx, '%'
    je .divide
    cmp ecx, '&'
    je .and
    cmp ecx, '|'
    je .or
    cmp ecx, '^'
    je .xor
    cmp r12, rax
    sete dl
    setl r8b
    setb r9b
    cmp ecx, T_EQ
    je .equal
    cmp ecx, T_NE
    je .not_equal
    mov al, r8b
    test dword [r13+TY_FLAGS], 1
    jz .ordered
    mov al, r9b
.ordered:
    cmp ecx, '<'
    je .boolean
    cmp ecx, T_LE
    je .less_equal
    cmp ecx, '>'
    je .greater
    cmp ecx, T_GE
    jne .type
    xor al, 1
    jmp .boolean
.less_equal:
    or al, dl
    jmp .boolean
.greater:
    or al, dl
    xor al, 1
    jmp .boolean
.equal:
    mov al, dl
    jmp .boolean
.not_equal:
    mov al, dl
    xor al, 1
    jmp .boolean
.add:
    add rax, r12
    seto dl
    jmp .overflow
.subtract:
    sub r12, rax
    seto dl
    mov rax, r12
    jmp .overflow
.multiply:
    imul rax, r12
    seto dl
.overflow:
    test dword [r13+TY_FLAGS], 1
    jnz .normalize
    test dl, dl
    jnz .range
    mov rdi, r13
    call constant_try_integer_range
    test ecx, ecx
    jnz .done
    jmp .normalize
.and:
    and rax, r12
    jmp .normalize
.or:
    or rax, r12
    jmp .normalize
.xor:
    xor rax, r12
    jmp .normalize
.shift:
    mov ecx, [r13+TY_BYTES]
    shl ecx, 3
    cmp rax, rcx
    jae .range
    mov ecx, eax
    mov rax, r12
    cmp dword [rbx+N_OP], T_SHR
    je .right_shift
    shl rax, cl
    jmp .normalize
.right_shift:
    test dword [r13+TY_FLAGS], 1
    jnz .logical_shift
    sar rax, cl
    jmp .normalize
.logical_shift:
    shr rax, cl
    jmp .normalize
.divide:
    test rax, rax
    jz .range
    mov rcx, rax
    mov rax, r12
    test dword [r13+TY_FLAGS], 1
    jnz .unsigned_divide
    mov rdx, 0x8000000000000000
    cmp dword [r13+TY_BYTES], 4
    jne .minimum
    mov rdx, -2147483648
.minimum:
    cmp rax, rdx
    jne .signed_divide
    cmp rcx, -1
    je .range
.signed_divide:
    cqo
    idiv rcx
    jmp .remainder
.unsigned_divide:
    xor edx, edx
    div rcx
.remainder:
    cmp dword [rbx+N_OP], '%'
    cmove rax, rdx
    jmp .normalize
.boolean:
    movzx eax, al
.normalize:
    mov rdi, [rbx+N_TYPE]
    call constant_integer_normalize
    xor edx, edx
    xor ecx, ecx
.done:
    pop r13
    pop r12
    pop rbx
    ret
.type:
    mov ecx, 8
    jmp .done
.range:
    mov ecx, 11
    jmp .done
