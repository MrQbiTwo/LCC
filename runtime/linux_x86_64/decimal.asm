; Integer-only exact decimal conversion: bounded bigint primitives and rounded parsing.
; Ordered NASM includes share section state and instruction macros; do not assemble alone.
%include "runtime/linux_x86_64/decimal/bigint.asm"
%include "runtime/linux_x86_64/decimal/parse.asm"
