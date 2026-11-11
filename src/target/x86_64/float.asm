; Floating-point templates and selection: SSE/SSE2 scalar arithmetic and isolated binary80.
; Ordered NASM includes share section state and instruction macros; do not assemble alone.
%include "src/target/x86_64/floating/scalar_templates.asm"
%include "src/target/x86_64/floating/value_templates.asm"
%include "src/target/x86_64/floating/compare_templates.asm"
%include "src/target/x86_64/floating/truncate_template.asm"
%include "src/target/x86_64/floating/sse32_templates.asm"
%include "src/target/x86_64/floating/sse64_templates.asm"
%include "src/target/x86_64/floating/binary80_templates.asm"
%include "src/target/x86_64/floating/argument_templates.asm"
%include "src/target/x86_64/floating/convert.asm"
%include "src/target/x86_64/floating/operations.asm"
%include "src/target/x86_64/floating/startup.asm"
