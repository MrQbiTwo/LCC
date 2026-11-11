; Token spelling comparison and scoped symbol lookup.
section .bss
align 8
global_symbols: resq 1 ; File bindings and canonical static/linkage entities.
local_symbols: resq 1 ; Current function locals, innermost first.
current_function: resq 1 ; Function whose body is being parsed.
local_bytes: resq 1 ; Current function stack allocation.
parse_token: resq 1 ; Current token in the parser context.
parse_depth: resd 1 ; Recursive expression nesting limit.
scope_depth: resd 1 ; Lexical scope depth.
section .text
; Compare token RDI with a zero-terminated spelling RSI.
token_is:
    test rdi, rdi
    jz .no
    mov ecx, [rdi+TK_LEN]
    mov rdi, [rdi+TK_TEXT]
    xor eax, eax
.loop:
    cmp eax, ecx
    jae .end
    mov dl, [rdi+rax]
    cmp dl, [rsi+rax]
    jne .no
    inc eax
    jmp .loop
.end:
    cmp byte [rsi+rax], 0
    sete al
    movzx eax, al
    ret
.no:
    xor eax, eax
    ret
; Compare the identifier spellings of tokens RDI and RSI.
token_same:
    mov ecx, [rdi+TK_LEN]
    cmp ecx, [rsi+TK_LEN]
    jne .no
    mov rdi, [rdi+TK_TEXT]
    mov rsi, [rsi+TK_TEXT]
    repe cmpsb
    sete al
    movzx eax, al
    ret
.no:
    xor eax, eax
    ret
; Consume a token of kind EDI; return its pointer or zero.
accept:
    mov rax, [parse_token]
    cmp edi, [rax+TK_KIND]
    jne .no
    mov rdx, [rax+TK_NEXT]
    mov [parse_token], rdx
    mov [diag_token], rax
    ret
.no:
    xor eax, eax
    ret
; Require a token of kind EDI and return the consumed token.
expect:
    sub rsp, 8
    call accept
    add rsp, 8
    test rax, rax
    jz syntax_error
    ret
; Consume a keyword spelling RDI, returning its token or zero.
accept_keyword:
    push rbx
    mov rsi, rdi
    mov rbx, [parse_token]
    mov rdi, rbx
    call token_is
    test eax, eax
    jz .done
    mov rax, [rbx+TK_NEXT]
    mov [parse_token], rax
    mov [diag_token], rbx
    mov rax, rbx
.done:
    pop rbx
    ret
; Look up token RDI in locals then global declarations.
symbol_find:
    push rbx
    push r12
    sub rsp, 8
    mov r12, rdi
    mov rbx, [local_symbols]
.local:
    test rbx, rbx
    jz .globals
    mov rdi, [rbx+S_NAME]
    mov rsi, r12
    call token_same
    test eax, eax
    jnz .found
    mov rbx, [rbx+S_NEXT]
    jmp .local
.globals:
    mov rbx, [global_symbols]
.global:
    test rbx, rbx
    jz .found
    test dword [rbx+S_FLAGS], SF_HIDDEN
    jnz .next_global
    mov rdi, [rbx+S_NAME]
    mov rsi, r12
    call token_same
    test eax, eax
    jnz .found
.next_global:
    mov rbx, [rbx+S_NEXT]
    jmp .global
.found:
    mov rax, rbx
    add rsp, 8
    pop r12
    pop rbx
    ret
; Report a stable parser diagnostic at the current token.
syntax_error:
    mov rax, [parse_token]
    mov [diag_token], rax
    lea rdi, [msg_syntax]
    jmp fatal
