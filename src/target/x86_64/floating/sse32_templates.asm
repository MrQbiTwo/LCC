; Encode and render f32 add.
i_f32_add:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    movd xmm0,eax
    movd xmm1,ecx
    addss xmm0,xmm1
    movd eax,xmm0
.bytes_end:
.text:
    db '    movd xmm0,eax',10
    db '    movd xmm1,ecx',10
    db '    addss xmm0,xmm1',10
    db '    movd eax,xmm0',10
.text_end:
; Encode and render f32 sub.
i_f32_sub:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    movd xmm0,eax
    movd xmm1,ecx
    subss xmm0,xmm1
    movd eax,xmm0
.bytes_end:
.text:
    db '    movd xmm0,eax',10
    db '    movd xmm1,ecx',10
    db '    subss xmm0,xmm1',10
    db '    movd eax,xmm0',10
.text_end:
; Encode and render f32 mul.
i_f32_mul:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    movd xmm0,eax
    movd xmm1,ecx
    mulss xmm0,xmm1
    movd eax,xmm0
.bytes_end:
.text:
    db '    movd xmm0,eax',10
    db '    movd xmm1,ecx',10
    db '    mulss xmm0,xmm1',10
    db '    movd eax,xmm0',10
.text_end:
; Encode and render f32 div.
i_f32_div:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    movd xmm0,eax
    movd xmm1,ecx
    divss xmm0,xmm1
    movd eax,xmm0
.bytes_end:
.text:
    db '    movd xmm0,eax',10
    db '    movd xmm1,ecx',10
    db '    divss xmm0,xmm1',10
    db '    movd eax,xmm0',10
.text_end:
; Encode and render f32 compare.
i_f32_compare:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    movd xmm0,eax
    movd xmm1,ecx
    ucomiss xmm0,xmm1
.bytes_end:
.text:
    db '    movd xmm0,eax',10
    db '    movd xmm1,ecx',10
    db '    ucomiss xmm0,xmm1',10
.text_end:
; Encode and render f32 double.
i_f32_double:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    movd xmm0,eax
    addss xmm0,xmm0
    movd eax,xmm0
.bytes_end:
.text:
    db '    movd xmm0,eax',10
    db '    addss xmm0,xmm0',10
    db '    movd eax,xmm0',10
.text_end:
; Encode and render f32 subtract limit.
i_f32_subtract_limit:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    subss xmm0,xmm1
    cvttss2si rax,xmm0
.bytes_end:
.text:
    db '    subss xmm0,xmm1',10
    db '    cvttss2si rax,xmm0',10
.text_end:
; Encode and render f32 xmm to i64.
i_f32_xmm_to_i64:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    cvttss2si rax,xmm0
.bytes_end:
.text:
    db '    cvttss2si rax,xmm0',10
.text_end:
; Encode and render f32 to ld.
i_f32_to_ld:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    sub rsp,16
    mov qword [rsp],rax
    fld dword [rsp]
    fstp tword [rsp]
    mov rax,qword [rsp]
    movzx edx,word [rsp+8]
    add rsp,16
.bytes_end:
.text:
    db '    sub rsp,16',10
    db '    mov qword [rsp],rax',10
    db '    fld dword [rsp]',10
    db '    fstp tword [rsp]',10
    db '    mov rax,qword [rsp]',10
    db '    movzx edx,word [rsp+8]',10
    db '    add rsp,16',10
.text_end:
; Encode and render ld to f32.
i_ld_to_f32:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    sub rsp,16
    mov qword [rsp],rax
    mov qword [rsp+8],rdx
    fld tword [rsp]
    fstp dword [rsp]
    mov eax,dword [rsp]
    add rsp,16
.bytes_end:
.text:
    db '    sub rsp,16',10
    db '    mov qword [rsp],rax',10
    db '    mov qword [rsp+8],rdx',10
    db '    fld tword [rsp]',10
    db '    fstp dword [rsp]',10
    db '    mov eax,dword [rsp]',10
    db '    add rsp,16',10
.text_end:
