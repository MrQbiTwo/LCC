; Tokenize source bytes with exact token spans and checked literal boundaries.
section .rodata
; Longest spellings precede their prefixes in this operator table.
lex_operators:
    dd T_SHL_EQ
    db '<<=',0
    dd T_SHR_EQ
    db '>>=',0
    dd T_ELLIPSIS
    db '...',0
    dd T_EQ
    db '==',0
    dd T_NE
    db '!=',0
    dd T_LE
    db '<=',0
    dd T_GE
    db '>=',0
    dd T_SHL
    db '<<',0
    dd T_SHR
    db '>>',0
    dd T_AND
    db '&&',0
    dd T_OR
    db '||',0
    dd T_INC
    db '++',0
    dd T_DEC
    db '--',0
    dd T_ARROW
    db '->',0
    dd T_ADD_EQ
    db '+=',0
    dd T_SUB_EQ
    db '-=',0
    dd T_MUL_EQ
    db '*=',0
    dd T_DIV_EQ
    db '/=',0
    dd T_MOD_EQ
    db '%=',0
    dd T_AND_EQ
    db '&=',0
    dd T_OR_EQ
    db '|=',0
    dd T_XOR_EQ
    db '^=',0
    dd T_PASTE
    db '##',0
    dd 0
lex_punctuation: db '{}[]();,:?~!+-*/%&|^<>=.#',0
section .text
; Advance the source cursor and its line/column counters in the lexer frame.
%macro lex_step 0
    cmp byte [r13], 10
    jne %%column
    inc dword [rsp]
    mov dword [rsp+4], 0
    or dword [rsp+16], 2
%%column:
    inc dword [rsp+4]
    inc r13
%endmacro
