; Aggregate value copying, result marshalling and result reconstruction.
; Include order preserves symbols, local labels and section state.
%include "src/target/x86_64/calling/value_copy.asm"
%include "src/target/x86_64/calling/return.asm"
%include "src/target/x86_64/calling/receive.asm"
