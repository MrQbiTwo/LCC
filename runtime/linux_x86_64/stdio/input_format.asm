; Format traversal and pointer varargs for the supported scanf conversions.
section .text
global lcc_rt_scanf

; SysV scanf: RDI format, subsequent GPR/stack arguments are destination pointers.
lcc_rt_scanf:
    push rbp
    mov rbp, rsp
    push rbx
    push r12
    push r13
    push r14
    push r15
    sub rsp, LCC_IN_FRAME
    mov r15, rsp
    mov [r15+LCC_IN_ARGS], rsi
    mov [r15+LCC_IN_ARGS+8], rdx
    mov [r15+LCC_IN_ARGS+16], rcx
    mov [r15+LCC_IN_ARGS+24], r8
    mov [r15+LCC_IN_ARGS+32], r9
    lea rax, [rbp+16]
    mov [r15+LCC_IN_STACK], rax
    mov qword [r15+LCC_IN_INDEX], 0
    mov qword [r15+LCC_IN_COUNT], 0
    mov r12, rdi
    ; A public input operation publishes its prompt even at cached EOF.
    call lcc_rt_flush
.format:
    movzx ebx, byte [r12]
    test ebx, ebx
    jz .done
    inc r12
    mov edi, ebx
    call lcc_rt_input_space
    test eax, eax
    jnz .space
    cmp ebx, '%'
    je .conversion
.literal:
    call lcc_rt_input_peek
    test eax, eax
    js .input_failure
    cmp eax, ebx
    jne .done
    call lcc_rt_input_consume
    jmp .format
.space:
    call lcc_rt_input_skip_space
    jmp .format
.conversion:
    cmp byte [r12], '%'
    jne .initialize
    inc r12
    mov ebx, '%'
    jmp .literal
.initialize:
    mov qword [r15+LCC_IN_SUPPRESS], 0
    mov qword [r15+LCC_IN_LENGTH], 0
    mov qword [r15+LCC_IN_WIDTH], 0
    mov qword [r15+LCC_IN_USED], 0
    cmp byte [r12], '*'
    jne .width
    inc r12
    mov qword [r15+LCC_IN_SUPPRESS], 1
.width:
    xor ebx, ebx
.width_digit:
    movzx eax, byte [r12]
    sub eax, '0'
    cmp eax, 9
    ja .width_done
    imul rbx, 10
    add rbx, rax
    cmp rbx, 4096
    ja .invalid_format
    inc r12
    jmp .width_digit
.width_done:
    mov [r15+LCC_IN_WIDTH], rbx
    movzx eax, byte [r12]
    cmp eax, 'h'
    je .short
    cmp eax, 'l'
    jne .specifier
    mov qword [r15+LCC_IN_LENGTH], 2
    inc r12
    jmp .specifier
.short:
    mov qword [r15+LCC_IN_LENGTH], 1
    inc r12
.specifier:
    movzx ebx, byte [r12]
    inc r12
    mov [r15+LCC_IN_SPEC], rbx
    cmp ebx, 'd'
    je .numeric
    cmp ebx, 'i'
    je .numeric
    cmp ebx, 'u'
    je .numeric
    cmp ebx, 'f'
    je .floating
    cmp ebx, 'c'
    je .characters
    cmp ebx, 's'
    jne .invalid_format
.characters:
    cmp qword [r15+LCC_IN_LENGTH], 0
    jne .invalid_format
    jmp .width_default
.floating:
    cmp qword [r15+LCC_IN_LENGTH], 1
    je .invalid_format
.numeric:
.width_default:
    cmp qword [r15+LCC_IN_WIDTH], 0
    jne .argument
    mov eax, 4096
    cmp ebx, 'c'
    jne .set_width
    mov eax, 1
.set_width:
    mov [r15+LCC_IN_WIDTH], rax
.argument:
    cmp qword [r15+LCC_IN_SUPPRESS], 0
    jne .skip_space
    mov rcx, [r15+LCC_IN_INDEX]
    cmp rcx, 5
    jae .stack_argument
    mov rax, [r15+LCC_IN_ARGS+rcx*8]
    inc qword [r15+LCC_IN_INDEX]
    jmp .destination
.stack_argument:
    mov rcx, [r15+LCC_IN_STACK]
    mov rax, [rcx]
    add qword [r15+LCC_IN_STACK], 8
.destination:
    mov [r15+LCC_IN_DEST], rax
.skip_space:
    cmp ebx, 'c'
    je .dispatch
    call lcc_rt_input_skip_space
    test eax, eax
    js .input_failure
.dispatch:
    cmp ebx, 'c'
    je .string
    cmp ebx, 's'
    je .string
    cmp ebx, 'f'
    je .float
    call lcc_rt_scan_integer
    jmp .converted
.float:
    call lcc_rt_scan_float
    jmp .converted
.string:
    call lcc_rt_scan_string
.converted:
    test eax, eax
    js .input_failure
    jz .done
    cmp qword [r15+LCC_IN_SUPPRESS], 0
    jne .format
    inc qword [r15+LCC_IN_COUNT]
    jmp .format
.input_failure:
    cmp qword [r15+LCC_IN_COUNT], 0
    jne .done
.invalid_format:
    mov eax, -1
    jmp .return
.done:
    mov eax, [r15+LCC_IN_COUNT]
.return:
    add rsp, LCC_IN_FRAME
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    pop rbp
    ret

; Consume a selected numeric byte into the bounded token, preserving EAX.
lcc_rt_scan_take:
    mov rcx, [r15+LCC_IN_USED]
    mov [r15+LCC_IN_TOKEN+rcx], al
    inc qword [r15+LCC_IN_USED]
    dec qword [r15+LCC_IN_WIDTH]
    jmp lcc_rt_input_consume

; Read %s or %c. Width limits input bytes; only %s appends a terminator.
lcc_rt_scan_string:
    push rbx
    push r12
    push r13
    mov r12, [r15+LCC_IN_DEST]
    xor ebx, ebx
.next:
    call lcc_rt_input_peek
    test eax, eax
    js .ended
    cmp qword [r15+LCC_IN_SPEC], 'c'
    je .save
    cmp eax, 32
    je .ended
    lea edx, [rax-9]
    cmp edx, 4
    jbe .ended
.save:
    cmp qword [r15+LCC_IN_SUPPRESS], 0
    jne .consume
    mov [r12+rbx], al
.consume:
    call lcc_rt_input_consume
    inc rbx
    cmp rbx, [r15+LCC_IN_WIDTH]
    jb .next
.ended:
    test eax, eax
    jns .count
    cmp qword [r15+LCC_IN_SPEC], 'c'
    je .failure
.count:
    test rbx, rbx
    jz .failure
    cmp qword [r15+LCC_IN_SPEC], 'c'
    je .success
    cmp qword [r15+LCC_IN_SUPPRESS], 0
    jne .success
    mov byte [r12+rbx], 0
.success:
    mov eax, 1
    jmp .return
.failure:
    mov eax, -1
.return:
    pop r13
    pop r12
    pop rbx
    ret
