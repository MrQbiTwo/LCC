; Only an LCC-owned process entry establishes the default floating environment.
section .rodata
i_fp_startup:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    fninit
    sub rsp,16
    mov dword [rsp],0x1f80
    ldmxcsr [rsp]
    add rsp,16
.bytes_end:
.text:
    db '    fninit',10
    db '    sub rsp,16',10
    db '    mov dword [rsp],0x1f80',10
    db '    ldmxcsr [rsp]',10
    db '    add rsp,16',10
.text_end:
