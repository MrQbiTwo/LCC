; Independent application runtime packed into generated executables at build time.
bits 64
default rel
%include "runtime/linux_x86_64/io.asm"
%include "runtime/linux_x86_64/start.asm"
%include "runtime/linux_x86_64/decimal.asm"
%include "runtime/linux_x86_64/stdio.asm"
section .note.GNU-stack noalloc noexec nowrite progbits
