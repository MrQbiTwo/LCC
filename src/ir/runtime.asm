; Checked runtime blobs share labels and fixups with ordinary generated code.
section .rodata
%include "runtime_pack.inc"
%include "src/ir/runtime/bindings.asm"
%include "src/ir/runtime/emit.asm"
%include "src/ir/runtime/text.asm"
