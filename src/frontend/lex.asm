; Lexer module map. Tables/macros precede their scanner uses.
; tokens: operator spellings and cursor step; scan: source to token list.
; escape: byte-valued character and string escape decoding.
%include "src/frontend/lexer/tokens.asm"
%include "src/frontend/lexer/scan.asm"
%include "src/frontend/lexer/escape.asm"
