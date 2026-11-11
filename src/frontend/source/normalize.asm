; Translation phases normalize trigraphs and splice lines before tokenization.
section .rodata
trigraph_from: db "=/'()!<>-",0
trigraph_to: db '#',92,'^[]|{}~'
section .text
; Decode one physical character, including trigraphs and CRLF, without advancing state.
source_character:
    movzx eax, byte [rdi]
    lea rdx, [rdi+1]
    cmp al, 13
    je .cr
    cmp al, '?'
    jne .done
    lea rcx, [rdi+2]
    cmp rcx, rsi
    jae .done
    cmp byte [rdi+1], '?'
    jne .done
    mov r8b, [rdi+2]
    lea r9, [trigraph_from]
    xor ecx, ecx
.find:
    cmp byte [r9+rcx], 0
    je .done
    cmp r8b, [r9+rcx]
    je .replace
    inc ecx
    jmp .find
.replace:
    lea r9, [trigraph_to]
    movzx eax, byte [r9+rcx]
    lea rdx, [rdi+3]
    ret
.cr:
    mov eax, 10
    cmp rdx, rsi
    jae .done
    cmp byte [rdx], 10
    jne .done
    inc rdx
.done:
    ret
; Normalize a source record RDI while recording original line/column positions.
source_normalize:
    push rbx
    push r12
    push r13
    push r14
    push r15
    sub rsp, 32
    mov rbx, rdi
    mov r12, [rbx+FILE_DATA]
    mov r13, [rbx+FILE_LEN]
    lea rdi, [r13+2]
    call arena_alloc
    mov r14, rax
    mov [rsp+16], rax
    lea rdi, [r13*8+8]
    call arena_alloc
    mov r15, rax
    mov [rbx+FILE_MAP], rax
    add r13, r12
    mov dword [rsp], 1
    mov dword [rsp+4], 1
    mov dword [rsp+8], 0
.next:
    cmp r12, r13
    jae .done
    mov rdi, r12
    mov rsi, r13
    call source_character
    test al, al
    jz .invalid
    mov r10d, eax
    cmp al, 92
    jne .emit
    cmp rdx, r13
    jae .emit
    mov [rsp+24], rdx
    mov rdi, rdx
    mov rsi, r13
    call source_character
    cmp al, 10
    je .splice
    mov rdx, [rsp+24]
    mov r10d, 92
    jmp .emit
.splice:
    mov r10d, -1
.emit:
    cmp r10d, -1
    je .advance
    mov [r14], r10b
    inc r14
    mov rax, [rsp]
    mov [r15], rax
    add r15, 8
.advance:
    cmp r12, rdx
    jae .next
    mov al, [r12]
    inc r12
    cmp al, 13
    je .carriage
    cmp al, 10
    je .newline
    inc dword [rsp+4]
    mov dword [rsp+8], 0
    jmp .advance
.carriage:
    inc dword [rsp]
    mov dword [rsp+4], 1
    mov dword [rsp+8], 1
    jmp .advance
.newline:
    cmp dword [rsp+8], 0
    jne .after_newline
    inc dword [rsp]
.after_newline:
    mov dword [rsp+4], 1
    mov dword [rsp+8], 0
    jmp .advance
.done:
    mov rax, [rsp]
    mov [r15], rax
    mov rax, [rsp+16]
    mov [rbx+FILE_DATA], rax
    sub r14, rax
    mov [rbx+FILE_LEN], r14
    mov rax, rbx
    add rsp, 32
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
.invalid:
    lea rdi, [msg_lex]
    jmp fatal
