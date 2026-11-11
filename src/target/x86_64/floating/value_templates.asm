; Encode and render high to rsp.
i_high_to_rsp:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    mov qword [rsp+8],rdx
.bytes_end:
.text:
    db '    mov qword [rsp+8],rdx',10
.text_end:
; Encode and render high from rsp.
i_high_from_rsp:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    mov rdx,qword [rsp]
.bytes_end:
.text:
    db '    mov rdx,qword [rsp]',10
.text_end:
; Encode and render left high from rsp.
i_left_high_from_rsp:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    mov r8,qword [rsp]
.bytes_end:
.text:
    db '    mov r8,qword [rsp]',10
.text_end:
; Encode and render exchange high.
i_exchange_high:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    xchg rdx,r8
.bytes_end:
.text:
    db '    xchg rdx,r8',10
.text_end:
; Encode and render store 128.
i_store_128:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    mov qword [rcx],rax
    mov qword [rcx+8],rdx
.bytes_end:
.text:
    db '    mov qword [rcx],rax',10
    db '    mov qword [rcx+8],rdx',10
.text_end:
; Encode and render load 128.
i_load_128:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    movzx edx,word [rax+8]
    mov rax,qword [rax]
.bytes_end:
.text:
    db '    movzx edx,word [rax+8]',10
    db '    mov rax,qword [rax]',10
.text_end:
; Encode and render u64 halve.
i_u64_halve:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    mov rcx,rax
    and eax,1
    shr rcx,1
    or rax,rcx
.bytes_end:
.text:
    db '    mov rcx,rax',10
    db '    and eax,1',10
    db '    shr rcx,1',10
    db '    or rax,rcx',10
.text_end:
; Encode and render set high bit.
i_set_high_bit:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    bts rax,63
.bytes_end:
.text:
    db '    bts rax,63',10
.text_end:
