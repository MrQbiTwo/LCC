; Translation-unit and statement lowering, and object/literal data.
; Ordered NASM includes share section state and instruction macros; do not assemble alone.
%include "src/ir/lower/unit.asm"
%include "src/ir/lower/statement.asm"
%include "src/ir/lower/data.asm"
