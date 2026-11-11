; Source module map. Shared identity/output state precedes operations.
; input: immutable file records; output: checked atomic replacement.
; normalize: physical character decoding, splicing and source map.
%include "src/frontend/source/input.asm"
%include "src/frontend/source/output.asm"
%include "src/frontend/source/normalize.asm"
