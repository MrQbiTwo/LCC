; Encode and render ld add.
i_ld_add:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    sub rsp,32
    mov qword [rsp],rax
    mov qword [rsp+8],rdx
    mov qword [rsp+16],rcx
    mov qword [rsp+24],r8
    fld tword [rsp]
    fld tword [rsp+16]
    faddp st1,st0
    fstp tword [rsp]
    mov rax,qword [rsp]
    movzx edx,word [rsp+8]
    add rsp,32
.bytes_end:
.text:
    db '    sub rsp,32',10
    db '    mov qword [rsp],rax',10
    db '    mov qword [rsp+8],rdx',10
    db '    mov qword [rsp+16],rcx',10
    db '    mov qword [rsp+24],r8',10
    db '    fld tword [rsp]',10
    db '    fld tword [rsp+16]',10
    db '    faddp st1,st0',10
    db '    fstp tword [rsp]',10
    db '    mov rax,qword [rsp]',10
    db '    movzx edx,word [rsp+8]',10
    db '    add rsp,32',10
.text_end:
; Encode and render ld sub.
i_ld_sub:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    sub rsp,32
    mov qword [rsp],rax
    mov qword [rsp+8],rdx
    mov qword [rsp+16],rcx
    mov qword [rsp+24],r8
    fld tword [rsp]
    fld tword [rsp+16]
    fsubp st1,st0
    fstp tword [rsp]
    mov rax,qword [rsp]
    movzx edx,word [rsp+8]
    add rsp,32
.bytes_end:
.text:
    db '    sub rsp,32',10
    db '    mov qword [rsp],rax',10
    db '    mov qword [rsp+8],rdx',10
    db '    mov qword [rsp+16],rcx',10
    db '    mov qword [rsp+24],r8',10
    db '    fld tword [rsp]',10
    db '    fld tword [rsp+16]',10
    db '    fsubp st1,st0',10
    db '    fstp tword [rsp]',10
    db '    mov rax,qword [rsp]',10
    db '    movzx edx,word [rsp+8]',10
    db '    add rsp,32',10
.text_end:
; Encode and render ld mul.
i_ld_mul:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    sub rsp,32
    mov qword [rsp],rax
    mov qword [rsp+8],rdx
    mov qword [rsp+16],rcx
    mov qword [rsp+24],r8
    fld tword [rsp]
    fld tword [rsp+16]
    fmulp st1,st0
    fstp tword [rsp]
    mov rax,qword [rsp]
    movzx edx,word [rsp+8]
    add rsp,32
.bytes_end:
.text:
    db '    sub rsp,32',10
    db '    mov qword [rsp],rax',10
    db '    mov qword [rsp+8],rdx',10
    db '    mov qword [rsp+16],rcx',10
    db '    mov qword [rsp+24],r8',10
    db '    fld tword [rsp]',10
    db '    fld tword [rsp+16]',10
    db '    fmulp st1,st0',10
    db '    fstp tword [rsp]',10
    db '    mov rax,qword [rsp]',10
    db '    movzx edx,word [rsp+8]',10
    db '    add rsp,32',10
.text_end:
; Encode and render ld div.
i_ld_div:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    sub rsp,32
    mov qword [rsp],rax
    mov qword [rsp+8],rdx
    mov qword [rsp+16],rcx
    mov qword [rsp+24],r8
    fld tword [rsp]
    fld tword [rsp+16]
    fdivp st1,st0
    fstp tword [rsp]
    mov rax,qword [rsp]
    movzx edx,word [rsp+8]
    add rsp,32
.bytes_end:
.text:
    db '    sub rsp,32',10
    db '    mov qword [rsp],rax',10
    db '    mov qword [rsp+8],rdx',10
    db '    mov qword [rsp+16],rcx',10
    db '    mov qword [rsp+24],r8',10
    db '    fld tword [rsp]',10
    db '    fld tword [rsp+16]',10
    db '    fdivp st1,st0',10
    db '    fstp tword [rsp]',10
    db '    mov rax,qword [rsp]',10
    db '    movzx edx,word [rsp+8]',10
    db '    add rsp,32',10
.text_end:
; Encode and render ld compare.
i_ld_compare:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    sub rsp,32
    mov qword [rsp],rax
    mov qword [rsp+8],rdx
    mov qword [rsp+16],rcx
    mov qword [rsp+24],r8
    fld tword [rsp+16]
    fld tword [rsp]
    fucomip st0,st1
    fstp st0
    lea rsp,[rsp+32]
.bytes_end:
.text:
    db '    sub rsp,32',10
    db '    mov qword [rsp],rax',10
    db '    mov qword [rsp+8],rdx',10
    db '    mov qword [rsp+16],rcx',10
    db '    mov qword [rsp+24],r8',10
    db '    fld tword [rsp+16]',10
    db '    fld tword [rsp]',10
    db '    fucomip st0,st1',10
    db '    fstp st0',10
    db '    lea rsp,[rsp+32]',10
.text_end:
; Encode and render ld return.
i_ld_return:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    sub rsp,16
    mov qword [rsp],rax
    mov qword [rsp+8],rdx
    fld tword [rsp]
    add rsp,16
.bytes_end:
.text:
    db '    sub rsp,16',10
    db '    mov qword [rsp],rax',10
    db '    mov qword [rsp+8],rdx',10
    db '    fld tword [rsp]',10
    db '    add rsp,16',10
.text_end:
; Encode and render ld receive.
i_ld_receive:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    sub rsp,16
    fstp tword [rsp]
    mov rax,qword [rsp]
    movzx edx,word [rsp+8]
    add rsp,16
.bytes_end:
.text:
    db '    sub rsp,16',10
    db '    fstp tword [rsp]',10
    db '    mov rax,qword [rsp]',10
    db '    movzx edx,word [rsp+8]',10
    db '    add rsp,16',10
.text_end:
; Encode and render i64 to ld.
i_i64_to_ld:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    mov rdx,rax
    sar rdx,63
    xor rax,rdx
    sub rax,rdx
    and edx,0x8000
    mov r8,rax
    xor ecx,ecx
    bsr rcx,rax
    mov r10d,0
    cmovz ecx,r10d
    mov r9d,63
    sub r9d,ecx
    add ecx,16383
    or edx,ecx
    mov ecx,r9d
    shl rax,cl
    test r8,r8
    cmovz edx,r10d
.bytes_end:
.text:
    db '    mov rdx,rax',10
    db '    sar rdx,63',10
    db '    xor rax,rdx',10
    db '    sub rax,rdx',10
    db '    and edx,0x8000',10
    db '    mov r8,rax',10
    db '    xor ecx,ecx',10
    db '    bsr rcx,rax',10
    db '    mov r10d,0',10
    db '    cmovz ecx,r10d',10
    db '    mov r9d,63',10
    db '    sub r9d,ecx',10
    db '    add ecx,16383',10
    db '    or edx,ecx',10
    db '    mov ecx,r9d',10
    db '    shl rax,cl',10
    db '    test r8,r8',10
    db '    cmovz edx,r10d',10
.text_end:
; Encode and render u64 to ld.
i_u64_to_ld:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    xor edx,edx
    mov r8,rax
    xor ecx,ecx
    bsr rcx,rax
    mov r10d,0
    cmovz ecx,r10d
    mov r9d,63
    sub r9d,ecx
    add ecx,16383
    or edx,ecx
    mov ecx,r9d
    shl rax,cl
    test r8,r8
    cmovz edx,r10d
.bytes_end:
.text:
    db '    xor edx,edx',10
    db '    mov r8,rax',10
    db '    xor ecx,ecx',10
    db '    bsr rcx,rax',10
    db '    mov r10d,0',10
    db '    cmovz ecx,r10d',10
    db '    mov r9d,63',10
    db '    sub r9d,ecx',10
    db '    add ecx,16383',10
    db '    or edx,ecx',10
    db '    mov ecx,r9d',10
    db '    shl rax,cl',10
    db '    test r8,r8',10
    db '    cmovz edx,r10d',10
.text_end:
