; System V entry reconstruction and outgoing calls use the shared ABI classifier.
; Ordered NASM includes share section state and instruction macros; do not assemble alone.
%include "src/target/x86_64/calling/parameters.asm"
%include "src/target/x86_64/calling/call.asm"
