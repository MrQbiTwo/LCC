; Preprocessor module map. Include order preserves data and local-label scopes.
; tokens: shared state, token clones, macro lookup and hide-set membership.
; expand: object/function rescan loop; file: per-file chunks and EOF.
; directive: definitions and conditional state; condition: #if evaluation.
; include: embedded headers and lookup; line: diagnostic source remapping.
; arguments: function substitution; operators: stringification and pasting.
%include "src/frontend/preprocessor/tokens.asm"
%include "src/frontend/preprocessor/expand.asm"
%include "src/frontend/preprocessor/file.asm"
%include "src/frontend/preprocessor/directive.asm"
%include "src/frontend/preprocessor/condition.asm"
%include "src/frontend/preprocessor/include.asm"
%include "src/frontend/preprocessor/line.asm"
%include "src/frontend/preprocessor/arguments.asm"
%include "src/frontend/preprocessor/operators.asm"
