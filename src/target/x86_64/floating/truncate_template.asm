; Encode and render ld to i64.
i_ld_to_i64:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    mov ecx,edx
    and ecx,0x7fff
    sub ecx,16383
    mov r8d,63
    sub r8d,ecx
    mov ecx,r8d
    shr rax,cl
    xor r9d,r9d
    cmp ecx,64
    cmovae rax,r9
    mov r8,rax
    neg r8
    test edx,0x8000
    cmovnz rax,r8
.bytes_end:
.text:
    db '    mov ecx,edx',10
    db '    and ecx,0x7fff',10
    db '    sub ecx,16383',10
    db '    mov r8d,63',10
    db '    sub r8d,ecx',10
    db '    mov ecx,r8d',10
    db '    shr rax,cl',10
    db '    xor r9d,r9d',10
    db '    cmp ecx,64',10
    db '    cmovae rax,r9',10
    db '    mov r8,rax',10
    db '    neg r8',10
    db '    test edx,0x8000',10
    db '    cmovnz rax,r8',10
.text_end:
