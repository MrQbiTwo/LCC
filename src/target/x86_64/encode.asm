; Shared native-byte and NASM emission: templates, labels, typed objects and value stack.
; Ordered NASM includes share section state and instruction macros; do not assemble alone.
%include "src/target/x86_64/encoding/templates.asm"
%include "src/target/x86_64/encoding/emit.asm"
%include "src/target/x86_64/encoding/labels.asm"
%include "src/target/x86_64/encoding/memory.asm"
%include "src/target/x86_64/encoding/stack.asm"
