; Floating-point templates use SSE/SSE2 for binary32/64 and isolated binary80.
section .rodata
; Encode and render f32 to xmm.
i_f32_to_xmm:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    movd xmm0,eax
.bytes_end:
.text:
    db '    movd xmm0,eax',10
.text_end:
; Encode and render f64 to xmm.
i_f64_to_xmm:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    movq xmm0,rax
.bytes_end:
.text:
    db '    movq xmm0,rax',10
.text_end:
; Encode and render xmm to f32.
i_xmm_to_f32:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    movd eax,xmm0
.bytes_end:
.text:
    db '    movd eax,xmm0',10
.text_end:
; Encode and render xmm to f64.
i_xmm_to_f64:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    movq rax,xmm0
.bytes_end:
.text:
    db '    movq rax,xmm0',10
.text_end:
; Encode and render f32 truth.
i_f32_truth:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    mov ecx,eax
    and ecx,0x7fffffff
    test ecx,ecx
.bytes_end:
.text:
    db '    mov ecx,eax',10
    db '    and ecx,0x7fffffff',10
    db '    test ecx,ecx',10
.text_end:
; Encode and render f64 truth.
i_f64_truth:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    mov rcx,rax
    btr rcx,63
    test rcx,rcx
.bytes_end:
.text:
    db '    mov rcx,rax',10
    db '    btr rcx,63',10
    db '    test rcx,rcx',10
.text_end:
; Encode and render f32 neg.
i_f32_neg:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    xor eax,0x80000000
.bytes_end:
.text:
    db '    xor eax,0x80000000',10
.text_end:
; Encode and render f64 neg.
i_f64_neg:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    btc rax,63
.bytes_end:
.text:
    db '    btc rax,63',10
.text_end:
; Encode and render ld neg.
i_ld_neg:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    xor edx,0x8000
.bytes_end:
.text:
    db '    xor edx,0x8000',10
.text_end:
; Encode and render f32 to f64.
i_f32_to_f64:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    movd xmm0,eax
    cvtss2sd xmm0,xmm0
    movq rax,xmm0
.bytes_end:
.text:
    db '    movd xmm0,eax',10
    db '    cvtss2sd xmm0,xmm0',10
    db '    movq rax,xmm0',10
.text_end:
; Encode and render f64 to f32.
i_f64_to_f32:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    movq xmm0,rax
    cvtsd2ss xmm0,xmm0
    movd eax,xmm0
.bytes_end:
.text:
    db '    movq xmm0,rax',10
    db '    cvtsd2ss xmm0,xmm0',10
    db '    movd eax,xmm0',10
.text_end:
; Encode and render i64 to f32.
i_i64_to_f32:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    cvtsi2ss xmm0,rax
    movd eax,xmm0
.bytes_end:
.text:
    db '    cvtsi2ss xmm0,rax',10
    db '    movd eax,xmm0',10
.text_end:
; Encode and render i64 to f64.
i_i64_to_f64:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    cvtsi2sd xmm0,rax
    movq rax,xmm0
.bytes_end:
.text:
    db '    cvtsi2sd xmm0,rax',10
    db '    movq rax,xmm0',10
.text_end:
; Encode and render f32 to i64.
i_f32_to_i64:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    movd xmm0,eax
    cvttss2si rax,xmm0
.bytes_end:
.text:
    db '    movd xmm0,eax',10
    db '    cvttss2si rax,xmm0',10
.text_end:
; Encode and render f64 to i64.
i_f64_to_i64:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    movq xmm0,rax
    cvttsd2si rax,xmm0
.bytes_end:
.text:
    db '    movq xmm0,rax',10
    db '    cvttsd2si rax,xmm0',10
.text_end:
