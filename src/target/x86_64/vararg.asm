; System V stdarg templates, persistent register saves and independent cursor traversal.
; Ordered NASM includes share section state and instruction macros; do not assemble alone.
%include "src/target/x86_64/varargs/templates.asm"
%include "src/target/x86_64/varargs/save.asm"
%include "src/target/x86_64/varargs/operations.asm"
%include "src/target/x86_64/varargs/fetch.asm"
