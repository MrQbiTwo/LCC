; Complete argument parsing before performing information or compilation actions.
section .rodata
cli_v: db '-v',0
cli_version: db '-version',0
cli_h: db '-h',0
cli_help: db '-help',0
cli_o: db '-o',0
cli_s: db '-S',0
version_text: db '1.0-sp1',10,0
help_text: db 'LCC - Little C Compiler',10
    db '-v, -version  Display the compiler version',10
    db '-h, -help     Display command information',10
    db '-o <path>     Select the output filename',10
    db '-S            Emit self-contained NASM assembly',10
    db 'Compile one .c or .h file; default output is <name>.elf or <name>.asm.',10,0
suffix_elf: db '.elf',0
suffix_asm: db '.asm',0
section .bss
align 8
input_path: resq 1 ; Sole source input path.
output_path: resq 1 ; Explicit or derived output path.
assembly_mode: resd 1 ; Nonzero requests NASM text.
info_flags: resd 1 ; Bit zero is help; bit one is version.
section .text
; Parse argc RDI and argv RSI, then execute the selected action.
cli_main:
    push rbx
    push r12
    push r13
    mov rbx, rdi
    mov r12, rsi
    mov r13d, 1
.args:
    cmp r13, rbx
    jae .parsed
    mov rdi, [r12+r13*8]
    cmp byte [rdi], '-'
    jne .input
    lea rsi, [cli_v]
    call string_equal
    test eax, eax
    jnz .version
    mov rdi, [r12+r13*8]
    lea rsi, [cli_version]
    call string_equal
    test eax, eax
    jnz .version
    mov rdi, [r12+r13*8]
    lea rsi, [cli_h]
    call string_equal
    test eax, eax
    jnz .help
    mov rdi, [r12+r13*8]
    lea rsi, [cli_help]
    call string_equal
    test eax, eax
    jnz .help
    mov rdi, [r12+r13*8]
    lea rsi, [cli_s]
    call string_equal
    test eax, eax
    jnz .assembly
    mov rdi, [r12+r13*8]
    lea rsi, [cli_o]
    call string_equal
    test eax, eax
    jz .usage
    cmp qword [output_path], 0
    jne .usage
    inc r13
    cmp r13, rbx
    jae .usage
    mov rax, [r12+r13*8]
    mov [output_path], rax
    jmp .next
.input:
    cmp qword [input_path], 0
    jne .usage
    mov [input_path], rdi
    jmp .next
.version:
    or dword [info_flags], 2
    jmp .next
.help:
    or dword [info_flags], 1
    jmp .next
.assembly:
    mov dword [assembly_mode], 1
.next:
    inc r13
    jmp .args
.parsed:
    test dword [info_flags], 1
    jz .print_version
    lea rdi, [help_text]
    call print_stdout
.print_version:
    test dword [info_flags], 2
    jz .compile
    lea rdi, [version_text]
    call print_stdout
.compile:
    cmp dword [info_flags], 0
    jne .done
    mov rdi, [input_path]
    test rdi, rdi
    jz .usage
    call string_length
    cmp rax, 2
    jb .usage
    mov rdi, [input_path]
    cmp byte [rdi+rax-2], '.'
    jne .usage
    mov dl, [rdi+rax-1]
    cmp dl, 'c'
    je .extension_ok
    cmp dl, 'h'
    jne .usage
.extension_ok:
    cmp qword [output_path], 0
    jne .load
    mov rbx, rdi
    xor ecx, ecx
.basename:
    cmp byte [rdi+rcx], 0
    je .name
    cmp byte [rdi+rcx], '/'
    jne .advance
    lea rbx, [rdi+rcx+1]
.advance:
    inc rcx
    jmp .basename
.name:
    mov rdi, rbx
    call string_length
    lea rdi, [rax+8]
    mov r13, rax
    sub r13, 2
    call arena_alloc
    mov [output_path], rax
    mov rdi, rax
    mov rsi, rbx
    mov rcx, r13
    rep movsb
    lea rsi, [suffix_elf]
    cmp dword [assembly_mode], 0
    je .suffix
    lea rsi, [suffix_asm]
.suffix:
    mov ecx, 5
    rep movsb
.load:
    mov rdi, [input_path]
    call source_load
    mov rdi, rax
    call pp_file
    mov rdi, rax
    call parse_unit
    call generate_unit
    call write_output
.done:
    xor eax, eax
    pop r13
    pop r12
    pop rbx
    ret
.usage:
    lea rdi, [msg_usage]
    mov esi, 2
    jmp fatal_status
; Write a zero-terminated information string RDI to stdout.
print_stdout:
    push rdi
    call string_length
    mov rdx, rax
    pop rsi
    mov edi, 1
    call host_write_all
    test rax, rax
    js .fail
    ret
.fail:
    lea rdi, [msg_write]
    jmp fatal
