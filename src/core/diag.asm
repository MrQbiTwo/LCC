; Stable diagnostics and immediate fatal-error termination.
section .rodata
msg_prefix: db 'lcc: ',0
msg_memory: db 'E001: memory or size limit exceeded',0
msg_read: db 'E002: cannot read a regular source file',0
msg_write: db 'E003: cannot safely write output',0
msg_alias: db 'E004: output aliases an input file',0
msg_usage: db 'E005: invalid command line; use lcc -help',0
msg_syntax: db 'E006: unexpected token',0
msg_lex: db 'E007: invalid or unterminated token',0
msg_type: db 'E008: invalid or unsupported type',0
msg_name: db 'E009: undeclared or duplicate identifier',0
msg_main: db 'E010: executable requires a defined main function',0
msg_range: db 'E011: numeric value or relocation out of range',0
msg_internal: db 'E012: internal compiler invariant failed',0
msg_colon: db ':',0
msg_newline: db 10,0
section .bss
align 8
diag_token: resq 1 ; Source token associated with the current operation.
section .text
; Print a zero-terminated string RDI to stderr, preserving its pointer.
print_error:
    push rdi
    call string_length
    mov rdx, rax
    pop rsi
    mov edi, 2
    jmp host_write_all
; Print a fatal diagnostic and terminate with source-error status.
fatal:
    mov esi, 1
    jmp fatal_status
; Terminate with a message RDI and explicit status ESI.
fatal_status:
    push rbx
    push r12
    push r13
    sub rsp, 32
    mov rbx, rdi
    mov r12d, esi
    mov r13, [diag_token]
    test r13, r13
    jz .message
    mov rax, [r13+TK_FILE]
    mov rdi, [rax+FILE_PATH]
    call print_error
    lea rdi, [msg_colon]
    call print_error
    mov edi, [r13+TK_LINE]
    mov rsi, rsp
    call format_unsigned
    mov byte [rsp+rax], 0
    mov rdi, rsp
    call print_error
    lea rdi, [msg_colon]
    call print_error
    mov edi, [r13+TK_COL]
    mov rsi, rsp
    call format_unsigned
    mov byte [rsp+rax], 0
    mov rdi, rsp
    call print_error
    lea rdi, [msg_colon]
    call print_error
.message:
    lea rdi, [msg_prefix]
    call print_error
    mov rdi, rbx
    call print_error
    lea rdi, [msg_newline]
    call print_error
    mov edi, r12d
    mov eax, 60
    syscall
    ud2
