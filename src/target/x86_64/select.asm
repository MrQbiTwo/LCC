; Typed instruction selection: lvalue addresses, expression effects and scalar operations.
; Ordered NASM includes share section state and instruction macros; do not assemble alone.
%include "src/target/x86_64/selection/address.asm"
%include "src/target/x86_64/selection/expression.asm"
%include "src/target/x86_64/selection/binary.asm"
