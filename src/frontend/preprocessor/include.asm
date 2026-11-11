; Built-in headers are immutable compiler data and need no installed files.
header_stdio_name: db 'stdio.h',0
header_stdarg_name: db 'stdarg.h',0
header_stddef_name: db 'stddef.h',0
header_limits_name: db 'limits.h',0
header_float_name: db 'float.h',0
header_stdio: incbin 'include/stdio.h'
header_stdio_end:
header_stdarg: incbin 'include/stdarg.h'
header_stdarg_end:
header_stddef: incbin 'include/stddef.h'
header_stddef_end:
header_limits: incbin 'include/limits.h'
header_limits_end:
header_float: incbin 'include/float.h'
header_float_end:
header_table:
    dq header_stdio_name,header_stdio,header_stdio_end-header_stdio
    dq header_stdarg_name,header_stdarg,header_stdarg_end-header_stdarg
    dq header_stddef_name,header_stddef,header_stddef_end-header_stddef
    dq header_limits_name,header_limits,header_limits_end-header_limits
    dq header_float_name,header_float,header_float_end-header_float
    dq 0,0,0
section .text
; Resolve an expanded include through the source directory and built-in headers.
pp_include_file:
    push rbx
    push r12
    push r13
    push r14
    push r15
    sub rsp, 32
    mov rbx, rsi
    call pp_expand
    mov r12, rax
    test r12, r12
    jz pp_failure
    mov edi, 4096
    call arena_alloc
    mov r13, rax
    mov [rsp], rax
    xor r14d, r14d
    cmp dword [r12+TK_KIND], T_STRING
    je .quoted
    cmp dword [r12+TK_KIND], '<'
    jne pp_failure
    mov r12, [r12+TK_NEXT]
.angle:
    test r12, r12
    jz pp_failure
    cmp dword [r12+TK_KIND], '>'
    je .angle_end
    mov ecx, [r12+TK_LEN]
    mov rax, r13
    sub rax, [rsp]
    add rax, rcx
    cmp rax, 4095
    ja pp_failure
    mov rdi, r13
    mov rsi, [r12+TK_TEXT]
    rep movsb
    mov r13, rdi
    mov r12, [r12+TK_NEXT]
    jmp .angle
.angle_end:
    cmp qword [r12+TK_NEXT], 0
    jne pp_failure
    mov byte [r13], 0
    mov r13, [rsp]
    jmp .embedded
.quoted:
    cmp qword [r12+TK_NEXT], 0
    jne pp_failure
    mov ecx, [r12+TK_LEN]
    sub ecx, 2
    cmp ecx, 4095
    ja pp_failure
    mov rsi, [r12+TK_TEXT]
    inc rsi
    mov rdi, r13
    rep movsb
    mov byte [rdi], 0
    cmp byte [r13], '/'
    je .probe_name
    mov rax, [rbx+PP_FILE]
    mov r12, [rax+FILE_PATH]
    xor r15d, r15d
    xor ecx, ecx
.directory:
    cmp byte [r12+rcx], 0
    je .join
    cmp byte [r12+rcx], '/'
    jne .directory_next
    lea r15, [rcx+1]
.directory_next:
    inc rcx
    jmp .directory
.join:
    mov rdi, r13
    call string_length
    lea rdi, [rax+r15+1]
    cmp rdi, 4096
    ja pp_failure
    mov [rsp+8], rax
    call arena_alloc
    mov r14, rax
    mov rdi, rax
    mov rsi, r12
    mov rcx, r15
    rep movsb
    mov rsi, r13
    mov rcx, [rsp+8]
    inc rcx
    rep movsb
    jmp .probe
.probe_name:
    mov r14, r13
.probe:
    mov rdi, r14
    mov esi, 0x80900
    xor edx, edx
    mov eax, 2
    syscall
    test rax, rax
    js .embedded
    mov rdi, rax
    mov eax, 3
    syscall
    mov rdi, r14
    call source_load
    mov rdi, rax
    call pp_file
    jmp .append
.embedded:
    lea r12, [header_table]
.find:
    cmp qword [r12], 0
    je pp_failure
    mov rdi, r13
    mov rsi, [r12]
    call string_equal
    test eax, eax
    jnz .builtin
    add r12, 24
    jmp .find
.builtin:
    mov edi, FILE_SIZE
    call arena_alloc
    mov rdx, [r12]
    mov [rax+FILE_PATH], rdx
    mov rdx, [r12+8]
    mov [rax+FILE_DATA], rdx
    mov rdx, [r12+16]
    mov [rax+FILE_LEN], rdx
    mov rdi, rax
    call pp_file
.append:
    mov r12, rax
    xor r13d, r13d
.tail:
    cmp dword [rax+TK_KIND], 0
    je .link
    mov r13, rax
    mov rax, [rax+TK_NEXT]
    jmp .tail
.link:
    test r13, r13
    jz .done
    mov qword [r13+TK_NEXT], 0
    mov rax, [rbx+PP_TAIL]
    test rax, rax
    jz .first
    mov [rax+TK_NEXT], r12
    jmp .tail_set
.first:
    mov [rbx+PP_HEAD], r12
.tail_set:
    mov [rbx+PP_TAIL], r13
.done:
    add rsp, 32
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
