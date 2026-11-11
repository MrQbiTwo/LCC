; Decode one character at RDI; return unsigned byte RAX and next pointer RDX.
decode_escape:
    movzx eax, byte [rdi]
    lea rdx, [rdi+1]
    test al, al
    jz .bad
    cmp al, 10
    je .bad
    cmp al, 92
    jne .done
    movzx eax, byte [rdx]
    inc rdx
    cmp al, 'n'
    je .newline
    cmp al, 't'
    je .tab
    cmp al, 'r'
    je .cr
    cmp al, 'a'
    je .bell
    cmp al, 'b'
    je .back
    cmp al, 'f'
    je .form
    cmp al, 'v'
    je .vertical
    cmp al, 92
    je .done
    cmp al, 39
    je .done
    cmp al, '"'
    je .done
    cmp al, '?'
    je .done
    cmp al, 'x'
    je .hex
    cmp al, '0'
    jb .bad
    cmp al, '7'
    ja .bad
    sub eax, '0'
    mov ecx, 2
.octal:
    movzx esi, byte [rdx]
    sub esi, '0'
    cmp esi, 7
    ja .done
    shl eax, 3
    add eax, esi
    inc rdx
    loop .octal
    cmp eax, 255
    ja .bad
    ret
.hex:
    xor eax, eax
    xor ecx, ecx
.hex_loop:
    movzx esi, byte [rdx]
    sub esi, '0'
    cmp esi, 9
    jbe .hex_digit
    movzx esi, byte [rdx]
    or esi, 32
    sub esi, 'a'-10
    cmp esi, 10
    jb .hex_end
    cmp esi, 15
    ja .hex_end
.hex_digit:
    shl eax, 4
    add eax, esi
    cmp eax, 255
    ja .bad
    inc rdx
    inc ecx
    jmp .hex_loop
.hex_end:
    test ecx, ecx
    jz .bad
.done:
    ret
.newline:
    mov eax, 10
    ret
.tab:
    mov eax, 9
    ret
.cr:
    mov eax, 13
    ret
.bell:
    mov eax, 7
    ret
.back:
    mov eax, 8
    ret
.form:
    mov eax, 12
    ret
.vertical:
    mov eax, 11
    ret
.bad:
    lea rdi, [msg_lex]
    jmp fatal
