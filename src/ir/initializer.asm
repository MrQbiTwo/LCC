; Initializer lowering: local stores and omitted zeros, then static data emission.
; Include order preserves symbols, local labels and section state.
%include "src/ir/initialization/local.asm"
%include "src/ir/initialization/global.asm"
