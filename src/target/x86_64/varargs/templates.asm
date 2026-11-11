; Native stdarg cursors use the shared ABI classes and a 176-byte save area.
section .rodata
; NASM assembles each instruction once; the matching text feeds application -S.
%macro va_instruction 3+
%1: dd %%end-%%bytes,%%textend-%%text
%%bytes: %3
%%end:
%%text: db %2,10
%%textend:
%endmacro
va_instruction i_va_gp, '    mov r8d,[r11]', mov r8d,[r11]
va_instruction i_va_fp, '    mov r9d,[r11+4]', mov r9d,[r11+4]
va_instruction i_va_save, '    mov r10,[r11+16]', mov r10,[r11+16]
va_instruction i_va_load_gp, '    mov rax,[r10+r8]', mov rax,[r10+r8]
va_instruction i_va_load_fp, '    mov rax,[r10+r9]', mov rax,[r10+r9]
va_instruction i_va_next_gp, '    add r8d,8', add r8d,8
va_instruction i_va_next_fp, '    add r9d,16', add r9d,16
va_instruction i_va_overflow, '    mov rax,[r11+8]', mov rax,[r11+8]
va_instruction i_va_overflow_store, '    mov [r11+8],rcx', mov [r11+8],rcx
va_instruction i_va_start_fp, '    mov [rcx+4],eax', mov [rcx+4],eax
va_instruction i_va_start_overflow, '    mov [rcx+8],rax', mov [rcx+8],rax
va_instruction i_va_start_save, '    mov [rcx+16],rax', mov [rcx+16],rax
va_instruction i_va_xmm0, '    movdqu [rax],xmm0', movdqu [rax],xmm0
va_instruction i_va_xmm1, '    movdqu [rax],xmm1', movdqu [rax],xmm1
va_instruction i_va_xmm2, '    movdqu [rax],xmm2', movdqu [rax],xmm2
va_instruction i_va_xmm3, '    movdqu [rax],xmm3', movdqu [rax],xmm3
va_instruction i_va_xmm4, '    movdqu [rax],xmm4', movdqu [rax],xmm4
va_instruction i_va_xmm5, '    movdqu [rax],xmm5', movdqu [rax],xmm5
va_instruction i_va_xmm6, '    movdqu [rax],xmm6', movdqu [rax],xmm6
va_instruction i_va_xmm7, '    movdqu [rax],xmm7', movdqu [rax],xmm7
va_xmm_stores: dq i_va_xmm0,i_va_xmm1,i_va_xmm2,i_va_xmm3,i_va_xmm4,i_va_xmm5,i_va_xmm6,i_va_xmm7
prefix p_va_al, '    cmp r10b,strict byte ', 0x41,0x80,0xfa
prefix p_va_gp_limit, '    cmp r8d,strict dword ', 0x41,0x81,0xf8
prefix p_va_fp_limit, '    cmp r9d,strict dword ', 0x41,0x81,0xf9
prefix p_va_add_gp, '    add dword [r11],strict byte ', 0x41,0x83,0x03
prefix p_va_add_fp, '    add dword [r11+4],strict byte ', 0x41,0x83,0x43,0x04
prefix p_va_and, '    and rax,strict dword ', 0x48,0x25
prefix p_va_after, '    lea rcx,[rax+', 0x48,0x8d,0x88
prefix p_ja, '    ja near ', 0x0f,0x87
prefix p_jbe, '    jbe near ', 0x0f,0x86
section .text
