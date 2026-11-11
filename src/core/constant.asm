; Typed constant conversion and evaluation run in the compiler-owned floating environment.
; Ordered NASM includes share section state and instruction macros; do not assemble alone.
%include "src/core/constant/convert.asm"
%include "src/core/constant/integer_value.asm"
%include "src/core/constant/integer_cast.asm"
%include "src/core/constant/integer_binary.asm"
%include "src/core/constant/integer.asm"
%include "src/core/constant/classify.asm"
%include "src/core/constant/probe.asm"
%include "src/core/constant/evaluate.asm"
%include "src/core/constant/static.asm"
%include "src/core/constant/address.asm"
%include "src/core/constant/static_numeric.asm"
