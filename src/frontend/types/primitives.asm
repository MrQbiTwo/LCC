; Target type records and typed node construction.
section .rodata
align 8
; Primitive target types use LP64 widths, independent of host tool types.
%macro primitive 5
%1: dd %2,%3,%3,%4
    dq 0,0,0,0,0,0
%endmacro
primitive type_void,K_VOID,1,0,0
primitive type_char,K_CHAR,1,TF_PLAIN_CHAR,0
primitive type_schar,K_CHAR,1,0,0
primitive type_uchar,K_CHAR,1,1,0
primitive type_short,K_SHORT,2,0,0
primitive type_ushort,K_SHORT,2,1,0
primitive type_int,K_INT,4,0,0
primitive type_uint,K_INT,4,1,0
primitive type_long,K_LONG,8,0,0
primitive type_ulong,K_LONG,8,1,0
primitive type_float,K_FLOAT,4,0,0
primitive type_double,K_DOUBLE,8,0,0
primitive type_ldouble,K_LDOUBLE,16,0,0
kw_void: db 'void',0
kw_char: db 'char',0
kw_short: db 'short',0
kw_int: db 'int',0
kw_long: db 'long',0
kw_signed: db 'signed',0
kw_unsigned: db 'unsigned',0
kw_float: db 'float',0
kw_double: db 'double',0
kw_const: db 'const',0
kw_volatile: db 'volatile',0
kw_static: db 'static',0
kw_extern: db 'extern',0
kw_typedef: db 'typedef',0
kw_auto: db 'auto',0
kw_register: db 'register',0
kw_return: db 'return',0
kw_if: db 'if',0
kw_else: db 'else',0
kw_while: db 'while',0
kw_do: db 'do',0
kw_for: db 'for',0
kw_break: db 'break',0
kw_continue: db 'continue',0
kw_sizeof: db 'sizeof',0
kw_main: db 'main',0
kw_switch: db 'switch',0
kw_case: db 'case',0
kw_default: db 'default',0
kw_goto: db 'goto',0
kw_enum: db 'enum',0
kw_struct: db 'struct',0
kw_union: db 'union',0
