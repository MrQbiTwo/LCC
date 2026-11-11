; Encode and render f64 add.
i_f64_add:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    movq xmm0,rax
    movq xmm1,rcx
    addsd xmm0,xmm1
    movq rax,xmm0
.bytes_end:
.text:
    db '    movq xmm0,rax',10
    db '    movq xmm1,rcx',10
    db '    addsd xmm0,xmm1',10
    db '    movq rax,xmm0',10
.text_end:
; Encode and render f64 sub.
i_f64_sub:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    movq xmm0,rax
    movq xmm1,rcx
    subsd xmm0,xmm1
    movq rax,xmm0
.bytes_end:
.text:
    db '    movq xmm0,rax',10
    db '    movq xmm1,rcx',10
    db '    subsd xmm0,xmm1',10
    db '    movq rax,xmm0',10
.text_end:
; Encode and render f64 mul.
i_f64_mul:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    movq xmm0,rax
    movq xmm1,rcx
    mulsd xmm0,xmm1
    movq rax,xmm0
.bytes_end:
.text:
    db '    movq xmm0,rax',10
    db '    movq xmm1,rcx',10
    db '    mulsd xmm0,xmm1',10
    db '    movq rax,xmm0',10
.text_end:
; Encode and render f64 div.
i_f64_div:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    movq xmm0,rax
    movq xmm1,rcx
    divsd xmm0,xmm1
    movq rax,xmm0
.bytes_end:
.text:
    db '    movq xmm0,rax',10
    db '    movq xmm1,rcx',10
    db '    divsd xmm0,xmm1',10
    db '    movq rax,xmm0',10
.text_end:
; Encode and render f64 compare.
i_f64_compare:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    movq xmm0,rax
    movq xmm1,rcx
    ucomisd xmm0,xmm1
.bytes_end:
.text:
    db '    movq xmm0,rax',10
    db '    movq xmm1,rcx',10
    db '    ucomisd xmm0,xmm1',10
.text_end:
; Encode and render f64 double.
i_f64_double:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    movq xmm0,rax
    addsd xmm0,xmm0
    movq rax,xmm0
.bytes_end:
.text:
    db '    movq xmm0,rax',10
    db '    addsd xmm0,xmm0',10
    db '    movq rax,xmm0',10
.text_end:
; Encode and render f64 subtract limit.
i_f64_subtract_limit:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    subsd xmm0,xmm1
    cvttsd2si rax,xmm0
.bytes_end:
.text:
    db '    subsd xmm0,xmm1',10
    db '    cvttsd2si rax,xmm0',10
.text_end:
; Encode and render f64 xmm to i64.
i_f64_xmm_to_i64:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    cvttsd2si rax,xmm0
.bytes_end:
.text:
    db '    cvttsd2si rax,xmm0',10
.text_end:
; Encode and render f64 to ld.
i_f64_to_ld:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    sub rsp,16
    mov qword [rsp],rax
    fld qword [rsp]
    fstp tword [rsp]
    mov rax,qword [rsp]
    movzx edx,word [rsp+8]
    add rsp,16
.bytes_end:
.text:
    db '    sub rsp,16',10
    db '    mov qword [rsp],rax',10
    db '    fld qword [rsp]',10
    db '    fstp tword [rsp]',10
    db '    mov rax,qword [rsp]',10
    db '    movzx edx,word [rsp+8]',10
    db '    add rsp,16',10
.text_end:
; Encode and render ld to f64.
i_ld_to_f64:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    sub rsp,16
    mov qword [rsp],rax
    mov qword [rsp+8],rdx
    fld tword [rsp]
    fstp qword [rsp]
    mov rax,qword [rsp]
    add rsp,16
.bytes_end:
.text:
    db '    sub rsp,16',10
    db '    mov qword [rsp],rax',10
    db '    mov qword [rsp+8],rdx',10
    db '    fld tword [rsp]',10
    db '    fstp qword [rsp]',10
    db '    mov rax,qword [rsp]',10
    db '    add rsp,16',10
.text_end:
