; Fixed instruction templates share native bytes with NASM text rendering.
section .bss
align 8
code_buffer: resb BUF_SIZE ; Generated executable instruction bytes.
data_buffer: resb BUF_SIZE ; Generated writable initialized data.
asm_buffer: resb BUF_SIZE ; Self-contained NASM source.
elf_buffer: resb BUF_SIZE ; Final ELF image.
label_count: resq 1 ; Stable serial for internal labels.
label_list: resq 1 ; All definitions and unresolved declarations.
fixup_list: resq 1 ; Relocations to patch after segment layout.
emit_section: resq 1 ; One for code, two for data.
section .rodata
; Each record stores native-byte length, text length, bytes, and NASM text.
%macro instruction 3+
%1: dd %%end-%%bytes, %%textend-%%text
%%bytes: db %3
%%end:
%%text: db %2,10
%%textend:
%endmacro
%macro prefix 3+
%1: dd %%end-%%bytes, %%textend-%%text
%%bytes: db %3
%%end:
%%text: db %2
%%textend:
%endmacro
instruction i_prologue, '    push rbp', 0x55
instruction i_frame, '    mov rbp,rsp', 0x48,0x89,0xe5
instruction i_leave, '    leave', 0xc9
instruction i_ret, '    ret', 0xc3
instruction i_zero, '    xor eax,eax', 0x31,0xc0
instruction i_push, '    sub rsp,8', 0x48,0x83,0xec,8
instruction i_push_rax, '    push rax', 0x50
instruction i_pop_rcx, '    pop rcx', 0x59
instruction i_pop_rdx, '    pop rdx', 0x5a
instruction i_unpad, '    add rsp,8', 0x48,0x83,0xc4,8
instruction i_add, '    add rax,rcx', 0x48,0x01,0xc8
instruction i_sub, '    sub rax,rcx', 0x48,0x29,0xc8
instruction i_exchange, '    xchg rax,rcx', 0x48,0x91
instruction i_mul, '    imul rax,rcx', 0x48,0x0f,0xaf,0xc1
instruction i_cqo, '    cqo', 0x48,0x99
instruction i_div, '    idiv rcx', 0x48,0xf7,0xf9
instruction i_udiv, '    div rcx', 0x48,0xf7,0xf1
instruction i_zero_rdx, '    xor edx,edx', 0x31,0xd2
instruction i_remainder, '    mov rax,rdx', 0x48,0x89,0xd0
instruction i_and, '    and rax,rcx', 0x48,0x21,0xc8
instruction i_or, '    or rax,rcx', 0x48,0x09,0xc8
instruction i_xor, '    xor rax,rcx', 0x48,0x31,0xc8
instruction i_shl, '    shl rax,cl', 0x48,0xd3,0xe0
instruction i_sar, '    sar rax,cl', 0x48,0xd3,0xf8
instruction i_shr, '    shr rax,cl', 0x48,0xd3,0xe8
instruction i_cmp, '    cmp rax,rcx', 0x48,0x39,0xc8
instruction i_test, '    test rax,rax', 0x48,0x85,0xc0
instruction i_eq, '    sete al', 0x0f,0x94,0xc0
instruction i_ne, '    setne al', 0x0f,0x95,0xc0
instruction i_lt, '    setl al', 0x0f,0x9c,0xc0
instruction i_le, '    setle al', 0x0f,0x9e,0xc0
instruction i_gt, '    setg al', 0x0f,0x9f,0xc0
instruction i_ge, '    setge al', 0x0f,0x9d,0xc0
instruction i_b, '    setb al', 0x0f,0x92,0xc0
instruction i_be, '    setbe al', 0x0f,0x96,0xc0
instruction i_a, '    seta al', 0x0f,0x97,0xc0
instruction i_ae, '    setae al', 0x0f,0x93,0xc0
instruction i_bool, '    movzx eax,al', 0x0f,0xb6,0xc0
instruction i_neg, '    neg rax', 0x48,0xf7,0xd8
instruction i_not, '    not rax', 0x48,0xf7,0xd0
instruction i_i8, '    movsx rax,al', 0x48,0x0f,0xbe,0xc0
instruction i_u8, '    movzx eax,al', 0x0f,0xb6,0xc0
instruction i_i16, '    movsx rax,ax', 0x48,0x0f,0xbf,0xc0
instruction i_u16, '    movzx eax,ax', 0x0f,0xb7,0xc0
instruction i_i32, '    movsxd rax,eax', 0x48,0x63,0xc0
instruction i_u32, '    mov eax,eax', 0x89,0xc0
instruction i_load_i8, '    movsx rax,byte [rax]', 0x48,0x0f,0xbe,0x00
instruction i_load_u8, '    movzx eax,byte [rax]', 0x0f,0xb6,0x00
instruction i_load_i16, '    movsx rax,word [rax]', 0x48,0x0f,0xbf,0x00
instruction i_load_u16, '    movzx eax,word [rax]', 0x0f,0xb7,0x00
instruction i_load_i32, '    movsxd rax,dword [rax]', 0x48,0x63,0x00
instruction i_load_u32, '    mov eax,dword [rax]', 0x8b,0x00
instruction i_load_64, '    mov rax,qword [rax]', 0x48,0x8b,0x00
instruction i_store_8, '    mov byte [rcx],al', 0x88,0x01
instruction i_store_16, '    mov word [rcx],ax', 0x66,0x89,0x01
instruction i_store_32, '    mov dword [rcx],eax', 0x89,0x01
instruction i_store_64, '    mov qword [rcx],rax', 0x48,0x89,0x01
instruction i_rax_rcx, '    mov rcx,rax', 0x48,0x89,0xc1
instruction i_rax_rdi, '    mov rdi,rax', 0x48,0x89,0xc7
instruction i_rax_rsi, '    mov rsi,rax', 0x48,0x89,0xc6
instruction i_rax_rdx, '    mov rdx,rax', 0x48,0x89,0xc2
instruction i_rax_r8, '    mov r8,rax', 0x49,0x89,0xc0
instruction i_rax_r9, '    mov r9,rax', 0x49,0x89,0xc1
instruction i_rdi_rax, '    mov rax,rdi', 0x48,0x89,0xf8
instruction i_rsi_rax, '    mov rax,rsi', 0x48,0x89,0xf0
instruction i_rdx_rax, '    mov rax,rdx', 0x48,0x89,0xd0
instruction i_rcx_rax, '    mov rax,rcx', 0x48,0x89,0xc8
instruction i_r8_rax, '    mov rax,r8', 0x4c,0x89,0xc0
instruction i_r9_rax, '    mov rax,r9', 0x4c,0x89,0xc8
instruction i_call_rax, '    call rax', 0xff,0xd0
instruction i_call_r11, '    call r11', 0x41,0xff,0xd3
instruction i_rax_r11, '    mov r11,rax', 0x49,0x89,0xc3
instruction i_argc, '    mov rdi,qword [rsp]', 0x48,0x8b,0x3c,0x24
instruction i_argv, '    lea rsi,[rsp+8]', 0x48,0x8d,0x74,0x24,8
instruction i_exit_result, '    mov edi,eax', 0x89,0xc7
instruction i_exit_number, '    mov eax,60', 0xb8,60,0,0,0
instruction i_syscall, '    syscall', 0x0f,0x05
prefix p_mov64, '    mov rax,strict qword ', 0x48,0xb8
prefix p_mov32, '    mov eax,strict dword ', 0xb8
prefix p_stack_sub, '    sub rsp,strict dword ', 0x48,0x81,0xec
prefix p_stack_add, '    add rsp,strict dword ', 0x48,0x81,0xc4
prefix p_imul, '    imul rax,rax,strict dword ', 0x48,0x69,0xc0
prefix p_frame_addr, '    lea rax,[rbp-', 0x48,0x8d,0x85
prefix p_stack_load, '    mov rax,[rsp+', 0x48,0x8b,0x84,0x24
prefix p_incoming, '    mov rax,[rbp+', 0x48,0x8b,0x85
prefix p_jump, '    jmp near ', 0xe9
prefix p_jz, '    jz near ', 0x0f,0x84
prefix p_jnz, '    jnz near ', 0x0f,0x85
prefix p_call, '    call ', 0xe8
